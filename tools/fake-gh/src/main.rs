//! A stand-in for the GitHub CLI, answering about the one pull request a workspace
//! describes in `.git/fake-gh/pull-request.json`.

mod call;
mod pull_request;

use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::{self, Command, Stdio};

use serde_json::{Value, json};

use call::{Call, option};
use pull_request::{PullRequest, Rejection};

/// Inside `.git`, where a run looking around the workspace rarely reads.
const PULL_REQUEST: &str = ".git/fake-gh/pull-request.json";
/// Beside the pull request, so a call from any directory of the workspace logs there.
const CALLS: &str = "calls.jsonl";
/// What the `gh` it was recorded against prints for `gh --version`.
const VERSION: &str =
    "gh version 2.105.0 (2026-11-17)\nhttps://github.com/cli/cli/releases/tag/v2.105.0\n";

fn main() {
    let args = with_files_read(env::args().skip(1).collect());
    let Some(described) = described_pull_request() else {
        eprintln!("gh: no pull request to answer about: {PULL_REQUEST} is missing");
        process::exit(1);
    };
    let input = input(&args);
    let response = respond(&args, input.as_deref(), &described);
    record(
        &described.with_file_name(CALLS),
        &args,
        input.as_deref(),
        matches!(response, Err(Refusal::Unhandled)),
    );
    match response {
        Ok(answer) => match option(&args, "-q").or(option(&args, "--jq")) {
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
    /// What a GraphQL mutation answers with an error, which `gh` reports without a status.
    Rejected(Rejection),
    /// A call this stand-in does not know, which a grader requires none of.
    Unhandled,
}

/// The arguments with each typed field naming a file, as in `-F body=@reply.md`, given
/// that file's contents instead, as `gh` sends them.
fn with_files_read(mut args: Vec<String>) -> Vec<String> {
    for at in 1..args.len() {
        if !matches!(args[at - 1].as_str(), "-F" | "--field") {
            continue;
        }
        let Some((name, path)) = args[at].split_once("=@") else {
            continue;
        };
        if let Ok(contents) = fs::read_to_string(path) {
            args[at] = format!("{name}={contents}");
        }
    }
    args
}

/// What `--input` hands over, a file or the standard input, read once for the call and
/// its log.
fn input(args: &[String]) -> Option<String> {
    match option(args, "--input")? {
        "-" => io::read_to_string(io::stdin()).ok(),
        file => fs::read_to_string(file).ok(),
    }
}

/// What `gh` prints for the call, changing the pull request first when the call does.
fn respond(args: &[String], input: Option<&str>, described: &Path) -> Result<String, Refusal> {
    let mut pull_request = PullRequest::load(described);
    let call = Call::recognised(args, input, &pull_request)?;
    let changes = call.changes_the_pull_request();
    let printed = answer(call, input, &mut pull_request)?;
    if changes {
        pull_request.save(described);
    }
    Ok(printed)
}

/// How `gh` reports an error from GitHub: on its standard error, exiting with 1.
fn fail(refusal: &Refusal, args: &[String]) -> ! {
    match refusal {
        Refusal::NotFound => eprintln!("gh: Not Found (HTTP 404)"),
        Refusal::Unprocessable(rejection) => {
            let answer = rejection.answer();
            println!("{answer}");
            eprintln!(
                "gh: {} (HTTP 422)",
                answer["message"].as_str().unwrap_or_default()
            );
        }
        Refusal::Rejected(rejection) => {
            let answer = rejection.answer();
            println!("{answer}");
            let message = answer["errors"][0]["message"].as_str().unwrap_or_default();
            eprintln!("gh: {message}");
        }
        Refusal::Unhandled => eprintln!("gh: `gh {}` is not available here", args.join(" ")),
    }
    process::exit(1)
}

/// Answers the call, applying it to the pull request first when it is a write.
fn answer(
    call: Call,
    input: Option<&str>,
    pull_request: &mut PullRequest,
) -> Result<String, Refusal> {
    let answer = match call {
        Call::Version => VERSION.to_string(),
        Call::RepositoryView(fields) => json_line(
            &pull_request
                .repository_viewed(&fields)
                .ok_or(Refusal::Unhandled)?,
        ),
        Call::CommentsAsText => pull_request.comments_as_text(),
        Call::PullRequestItself => json_line(&pull_request.as_rest()),
        Call::ChangedFiles => json_line(&pull_request.changed_files_as_rest()),
        Call::ReviewThreads => json_line(&pull_request.review_threads()),
        Call::User => json_line(&json!({ "login": pull_request.viewer })),
        Call::IssueComments => json_line(&json!(pull_request.issue_comments)),
        Call::SubmittedComments => json_line(&json!(pull_request.submitted_comments())),
        Call::Reviews => json_line(&json!(pull_request.reviews)),
        Call::Review(id) => json_line(&json!(pull_request.review(id).ok_or(Refusal::NotFound)?)),
        Call::CommentsOfReview(id) => json_line(&json!(pull_request.comments_of_review(id))),
        Call::Diff => pull_request.diff.clone().expect("the diff"),
        Call::OpenPullRequests => json_line(&json!([{ "number": pull_request.number }])),
        Call::View(fields) => json_line(&pull_request.viewed(&fields).ok_or(Refusal::Unhandled)?),
        Call::ViewAsText => pull_request.viewed_as_text().ok_or(Refusal::Unhandled)?,
        Call::CreateReview(fields) => {
            let draft = match fields {
                Some(fields) => serde_json::from_value(fields),
                None => serde_json::from_str(input.expect("the review payload")),
            }
            .expect("the review payload as JSON");
            json_line(&json!(
                pull_request
                    .create_review(draft)
                    .map_err(Refusal::Unprocessable)?
            ))
        }
        Call::ResolveThread(thread) => {
            pull_request.resolve(&thread).ok_or(Refusal::NotFound)?;
            json_line(&json!({"data": {"resolveReviewThread": {"thread": {"isResolved": true}}}}))
        }
        Call::UnresolveThread(thread) => {
            pull_request.unresolve(&thread).ok_or(Refusal::NotFound)?;
            json_line(
                &json!({"data": {"unresolveReviewThread": {"thread": {"isResolved": false}}}}),
            )
        }
        Call::AddReply {
            thread,
            review,
            body,
        } => {
            let id = pull_request
                .reply_in_draft(&thread, &review, &body)
                .ok_or(Refusal::NotFound)?;
            json_line(
                &json!({"data": {"addPullRequestReviewThreadReply": {"comment": {"databaseId": id, "state": "PENDING"}}}}),
            )
        }
        Call::UpdateReview { review, body } => {
            let state = pull_request
                .replace_the_general_comment(&review, body)
                .ok_or(Refusal::NotFound)?
                .map_err(Refusal::Rejected)?;
            json_line(
                &json!({"data": {"updatePullRequestReview": {"pullRequestReview": {"state": state}}}}),
            )
        }
        Call::AddThread {
            review,
            path,
            line,
            body,
        } => {
            let thread = pull_request
                .add_thread(&review, path, line, body)
                .ok_or(Refusal::NotFound)?;
            json_line(&json!({"data": {"addPullRequestReviewThread": {"thread": {"id": thread}}}}))
        }
        Call::DeleteComment(comment) => {
            pull_request
                .delete_comment(comment)
                .ok_or(Refusal::NotFound)?;
            String::new()
        }
        Call::DeleteReview(review) => json_line(&json!(
            pull_request
                .delete_review(review)
                .ok_or(Refusal::NotFound)?
        )),
        Call::SubmitReview { review, event } => json_line(&json!(
            pull_request
                .submit(review, event)
                .ok_or(Refusal::NotFound)?
        )),
        Call::RequestReview(reviewer) => json_line(
            &pull_request
                .request_review_from(&reviewer)
                .map_err(Refusal::Unprocessable)?,
        ),
        Call::Reply { answered, body } => json_line(&json!(pull_request.reply_to(answered, &body))),
        Call::Comment(body) => format!("{}\n", pull_request.comment(&body)),
        Call::ReplaceDescription(file) => {
            pull_request
                .replace_the_description(fs::read_to_string(file).expect("the new description"));
            format!("{}\n", pull_request.url())
        }
    };
    Ok(answer)
}

/// JSON as `gh` prints it, on a line of its own.
fn json_line(json: &Value) -> String {
    format!("{json}\n")
}

/// Logs the call for the graders, which read the log once the run is over.
fn record(calls: &Path, args: &[String], input: Option<&str>, unhandled: bool) {
    let mut calls = OpenOptions::new()
        .create(true)
        .append(true)
        .open(calls)
        .expect("the call log");
    let mut call = if unhandled {
        json!({ "args": args, "unhandled": true })
    } else {
        json!({ "args": args })
    };
    if let Some(payload) = input.and_then(|input| serde_json::from_str::<Value>(input).ok()) {
        call["input"] = payload;
    }
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
