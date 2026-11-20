//! What a `gh` invocation asks for, recognised once from its arguments.

use crate::Refusal;
use serde_json::json;

use crate::pull_request::{PullRequest, ReviewEvent};

/// Every call the stand-in answers. Anything else is refused as unhandled.
pub enum Call {
    Version,
    RepositoryView(String),
    CommentsAsText,
    PullRequestItself,
    ChangedFiles,
    ReviewThreads,
    User,
    IssueComments,
    SubmittedComments,
    Reviews,
    Review(u64),
    CommentsOfReview(u64),
    Diff,
    OpenPullRequests,
    View(String),
    ViewAsText,
    /// From a file, or from standard input when the file is `-`.
    CreateReview(String),
    ResolveThread(String),
    UnresolveThread(String),
    AddReply {
        thread: String,
        review: String,
        body: String,
    },
    UpdateReview {
        review: String,
        body: String,
    },
    AddThread {
        review: String,
        path: String,
        line: u64,
        body: String,
    },
    DeleteReview(u64),
    DeleteComment(u64),
    SubmitReview {
        review: u64,
        event: ReviewEvent,
    },
    RequestReview(String),
    Reply {
        answered: u64,
        body: String,
    },
    Comment(String),
    ReplaceDescription(String),
}

impl Call {
    pub fn recognised(args: &[String], pull_request: &PullRequest) -> Result<Self, Refusal> {
        if let [only] = args
            && only == "--version"
        {
            return Ok(Call::Version);
        }
        let [command, subcommand, ..] = args else {
            return Err(Refusal::Unhandled);
        };
        let call = match (command.as_str(), subcommand.as_str()) {
            ("api", "graphql") => return graphql(args),
            ("api", "user") => Call::User,
            ("api", _) => return rest(args, pull_request),
            ("repo", "view") => {
                Call::RepositoryView(option(args, "--json").unwrap_or_default().to_string())
            }
            ("pr", "diff") => Call::Diff,
            ("pr", "list") => Call::OpenPullRequests,
            ("pr", "view") if args.iter().any(|arg| arg == "--comments") => Call::CommentsAsText,
            ("pr", "view") => match option(args, "--json") {
                Some(fields) => Call::View(fields.to_string()),
                None => Call::ViewAsText,
            },
            ("pr", "comment") => Call::Comment(required(option(args, "--body"))?),
            ("pr", "edit") => Call::ReplaceDescription(required(option(args, "--body-file"))?),
            _ => return Err(Refusal::Unhandled),
        };
        Ok(call)
    }

    pub fn changes_the_pull_request(&self) -> bool {
        matches!(
            self,
            Call::CreateReview(_)
                | Call::AddThread { .. }
                | Call::UpdateReview { .. }
                | Call::AddReply { .. }
                | Call::ResolveThread(_)
                | Call::UnresolveThread(_)
                | Call::RequestReview(_)
                | Call::SubmitReview { .. }
                | Call::DeleteReview(_)
                | Call::DeleteComment(_)
                | Call::Reply { .. }
                | Call::Comment(_)
                | Call::ReplaceDescription(_)
        )
    }
}

/// A `gh api graphql` call, told apart by the operation its query names, its variables
/// read by the names `reviewing-pull-requests` gives them.
fn graphql(args: &[String]) -> Result<Call, Refusal> {
    let query = field(args, "query").ok_or(Refusal::Unhandled)?;
    // Before `resolveReviewThread`, whose name this one holds.
    if query.contains("unresolveReviewThread(") {
        return Ok(Call::UnresolveThread(required(field(args, "t"))?));
    }
    if query.contains("resolveReviewThread(") {
        return Ok(Call::ResolveThread(required(field(args, "t"))?));
    }
    if query.contains("addPullRequestReviewThreadReply(") {
        return Ok(Call::AddReply {
            thread: required(field(args, "t"))?,
            review: required(field(args, "r"))?,
            body: required(field(args, "b"))?,
        });
    }
    if query.contains("updatePullRequestReview(") {
        return Ok(Call::UpdateReview {
            review: required(field(args, "rid"))?,
            body: required(field(args, "body"))?,
        });
    }
    if query.contains("addPullRequestReviewThread(") {
        return Ok(Call::AddThread {
            review: required(field(args, "rid"))?,
            path: required(field(args, "path"))?,
            line: required(field(args, "line"))?
                .parse()
                .map_err(|_| Refusal::Unhandled)?,
            body: required(field(args, "body"))?,
        });
    }
    if query.contains("reviewThreads(") {
        return Ok(Call::ReviewThreads);
    }
    Err(Refusal::Unhandled)
}

/// A `gh api` call on a REST path, which reaches only the pull request the workspace
/// describes.
fn rest(args: &[String], pull_request: &PullRequest) -> Result<Call, Refusal> {
    let path = args
        .iter()
        .find_map(|arg| arg.strip_prefix("repos/"))
        .ok_or(Refusal::Unhandled)?;
    let repository = format!("{}/", pull_request.repository);
    // `gh` fills these in from the repo it runs in, the one the workspace describes.
    let path = path.replace("{owner}/{repo}/", &repository);
    let segments: Vec<&str> = path
        .strip_prefix(&repository)
        .ok_or(Refusal::NotFound)?
        .split('/')
        .collect();
    let method = option(args, "--method")
        .or_else(|| option(args, "-X"))
        .unwrap_or("GET");
    // A review comment is reached by its id alone, with no pull request in its path.
    if let ("DELETE", ["pulls", "comments", id]) = (method, segments.as_slice()) {
        return Ok(Call::DeleteComment(id_in(id)?));
    }
    let number = pull_request.number.to_string();
    let (kind, rest) = match segments.as_slice() {
        [kind, found, rest @ ..] if *found == number => (*kind, rest),
        _ => return Err(Refusal::NotFound),
    };
    let call = match (method, kind, rest) {
        ("GET", "pulls", []) => Call::PullRequestItself,
        ("GET", "pulls", ["files"]) => Call::ChangedFiles,
        ("GET", "issues", ["comments"]) => Call::IssueComments,
        ("GET", "pulls", ["comments"]) => Call::SubmittedComments,
        ("GET", "pulls", ["reviews"]) => Call::Reviews,
        ("GET", "pulls", ["reviews", id]) => Call::Review(id_in(id)?),
        ("GET", "pulls", ["reviews", id, "comments"]) => Call::CommentsOfReview(id_in(id)?),
        ("POST", "pulls", ["reviews"]) => Call::CreateReview(required(option(args, "--input"))?),
        ("DELETE", "pulls", ["reviews", id]) => Call::DeleteReview(id_in(id)?),
        ("POST", "pulls", ["reviews", id, "events"]) => Call::SubmitReview {
            review: id_in(id)?,
            event: serde_json::from_value(json!(required(field(args, "event"))?))
                .map_err(|_| Refusal::Unhandled)?,
        },
        ("POST", "pulls", ["requested_reviewers"]) => {
            Call::RequestReview(required(field(args, "reviewers[]"))?)
        }
        ("POST", "pulls", ["comments", id, "replies"]) => Call::Reply {
            answered: id_in(id)?,
            body: required(field(args, "body"))?,
        },
        _ => return Err(Refusal::Unhandled),
    };
    Ok(call)
}

/// An id in a path, where anything else names nothing GitHub has.
fn id_in(segment: &str) -> Result<u64, Refusal> {
    segment.parse().map_err(|_| Refusal::NotFound)
}

/// A call missing what it needs is one the stand-in cannot answer as `gh` would.
fn required(value: Option<&str>) -> Result<String, Refusal> {
    value.map(str::to_string).ok_or(Refusal::Unhandled)
}

/// The value given to a command-line option, as in `--body TEXT`.
pub fn option<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    let at = args.iter().position(|arg| arg == name)?;
    args.get(at + 1).map(String::as_str)
}

/// The value of a field `gh api` sends, as in `-f body=TEXT`.
fn field<'a>(args: &'a [String], name: &str) -> Option<&'a str> {
    args.iter()
        .find_map(|arg| arg.strip_prefix(name)?.strip_prefix('='))
}
