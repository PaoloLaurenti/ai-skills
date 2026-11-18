//! A stand-in for the GitHub CLI, answering about the one pull request a workspace
//! describes in `.git/fake-gh/pull-request.json`.

mod pull_request;

use std::env;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

use serde_json::{Value, json};

use pull_request::{PullRequest, Rejection};

/// Inside `.git`, where a run looking around the workspace rarely reads.
const PULL_REQUEST: &str = ".git/fake-gh/pull-request.json";
/// Beside the pull request, so a call from any directory of the workspace logs there.
const CALLS: &str = "calls.jsonl";

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    let Some(described) = described_pull_request() else {
        eprintln!("gh: no pull request to answer about: {PULL_REQUEST} is missing");
        process::exit(1);
    };
    let response = respond(&args, &described);
    record(
        &described.with_file_name(CALLS),
        &args,
        matches!(response, Err(Refusal::Unhandled)),
    );
    match response {
        Ok(answer) => match option(&args, "-q") {
            Some(expression) => print_filtered(&answer, expression),
            None => print!("{answer}"),
        },
        Err(refusal) => fail(&refusal, &args),
    }
}

/// A call answered with an error instead of an answer.
enum Refusal {
    NotFound,
    Unprocessable(Rejection),
    /// A call this stand-in does not know, which a grader requires none of.
    Unhandled,
}

/// What `gh` prints for the call, changing the pull request first when the call does.
fn respond(args: &[String], described: &Path) -> Result<String, Refusal> {
    if args.len() < 2 {
        return Err(Refusal::Unhandled);
    }
    let mut pull_request = PullRequest::load(described);
    let mut paths = args.iter().filter(|arg| arg.starts_with("repos/"));
    if paths.any(|path| !pull_request.is_reached_by(path)) {
        return Err(Refusal::NotFound);
    }
    match write(args, &mut pull_request) {
        Some(written) => {
            let printed = written?;
            pull_request.save(described);
            Ok(printed)
        }
        None => answer(args, &pull_request),
    }
}

/// How `gh` reports an error from GitHub: on its standard error, exiting with 1.
fn fail(refusal: &Refusal, args: &[String]) -> ! {
    match refusal {
        Refusal::NotFound => eprintln!("gh: Not Found (HTTP 404)"),
        Refusal::Unprocessable(Rejection::ReviewFromTheAuthor) => {
            eprintln!("gh: Review cannot be requested from pull request author. (HTTP 422)")
        }
        Refusal::Unhandled => eprintln!("gh: `gh {}` is not available here", args.join(" ")),
    }
    process::exit(1)
}

/// Applies a call that changes the pull request, and gives what `gh` prints for it.
fn write(args: &[String], pull_request: &mut PullRequest) -> Option<Result<String, Refusal>> {
    if args.iter().any(|arg| arg.ends_with("/requested_reviewers")) {
        let reviewer = field(args, "reviewers[]").expect("the reviewer");
        let requested = pull_request
            .request_review_from(reviewer)
            .map_err(Refusal::Unprocessable);
        return Some(requested.map(|requested| json_line(&requested)));
    }
    if let Some(replies) = args.iter().find(|arg| arg.ends_with("/replies")) {
        let answered = replies
            .split('/')
            .rev()
            .nth(1)
            .and_then(|id| id.parse().ok())
            .expect("the id of the answered comment");
        let body = field(args, "body").expect("the body");
        let reply = pull_request.reply_to(answered, body);
        return Some(Ok(json_line(&json!(reply))));
    }
    if args[..2] == ["pr", "comment"] {
        pull_request.comment(option(args, "--body").expect("the body"));
        return Some(Ok(String::new()));
    }
    if args[..2] == ["pr", "edit"] {
        let file = option(args, "--body-file").expect("the body file");
        pull_request
            .replace_the_description(fs::read_to_string(file).expect("the new description"));
        return Some(Ok(String::new()));
    }
    None
}

fn answer(args: &[String], pull_request: &PullRequest) -> Result<String, Refusal> {
    let answer = match (args[0].as_str(), args[1].as_str()) {
        ("api", "graphql") => json_line(&pull_request.review_threads()),
        ("api", "user") => json_line(&json!({ "login": pull_request.viewer })),
        ("api", path) if path.contains("/issues/") => {
            json_line(&json!(pull_request.issue_comments))
        }
        ("pr", "diff") => pull_request.diff.clone().expect("the diff"),
        ("pr", "list") => json_line(&json!([{ "number": pull_request.number }])),
        ("pr", "view") => json_line(&json!({ "body": pull_request.body })),
        ("api", path) if path.ends_with("/comments") => {
            json_line(&json!(pull_request.review_comments))
        }
        _ => return Err(Refusal::Unhandled),
    };
    Ok(answer)
}

/// The value given to a command-line option, as in `--body TEXT`.
fn option<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    let at = args.iter().position(|arg| arg == name)?;
    args.get(at + 1).map(String::as_str)
}

/// The value of a field `gh api` sends, as in `-f body=TEXT`.
fn field<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .find_map(|arg| arg.strip_prefix(name)?.strip_prefix('='))
}

/// JSON as `gh` prints it, on a line of its own.
fn json_line(json: &Value) -> String {
    format!("{json}\n")
}

/// Logs the call for the graders, which read the log once the run is over.
fn record(calls: &Path, args: &[String], unhandled: bool) {
    let mut calls = OpenOptions::new()
        .create(true)
        .append(true)
        .open(calls)
        .expect("the call log");
    let call = if unhandled {
        json!({ "args": args, "unhandled": true })
    } else {
        json!({ "args": args })
    };
    writeln!(calls, "{call}").expect("the call is logged");
}

/// The workspace's description of the pull request, looked for from the current directory up.
fn described_pull_request() -> Option<PathBuf> {
    let here = env::current_dir().expect("the current directory");
    here.ancestors()
        .map(|directory| directory.join(PULL_REQUEST))
        .find(|described| described.exists())
}

/// `gh`'s own `-q` runs jq over the answer and prints strings raw, as `jq -r` does.
fn print_filtered(answer: &str, expression: &str) {
    let mut jq = Command::new("jq")
        .args(["-r", expression])
        .stdin(Stdio::piped())
        .spawn()
        .expect("jq runs");
    jq.stdin
        .take()
        .expect("jq's input")
        .write_all(answer.as_bytes())
        .expect("the answer reaches jq");
    jq.wait().expect("jq finishes");
}
