//! The one pull request a workspace describes, in the shape GitHub's REST API gives it.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

#[derive(Deserialize, Serialize)]
pub struct PullRequest {
    pub repository: String,
    pub number: u64,
    pub author: Option<String>,
    /// The login `gh` is authenticated as, the requester of a run.
    pub viewer: Option<String>,
    pub body: Option<String>,
    pub diff: Option<String>,
    pub head_sha: Option<String>,
    title: Option<String>,
    /// As REST gives it, `open` or `closed`.
    state: Option<String>,
    head_ref: Option<String>,
    base_ref: Option<String>,
    #[serde(default)]
    pub review_comments: Vec<ReviewComment>,
    #[serde(default)]
    pub issue_comments: Vec<IssueComment>,
    #[serde(default)]
    pub reviews: Vec<Review>,
    /// Each named by the id of the comment that opened it.
    #[serde(default)]
    resolved_threads: Vec<u64>,
}

/// A comment on a line of the diff. A reply names the comment that opened its thread.
#[derive(Deserialize, Serialize)]
pub struct ReviewComment {
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pull_request_review_id: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    in_reply_to_id: Option<u64>,
    path: String,
    line: Option<u64>,
    user: User,
    body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    html_url: Option<String>,
    /// Whatever else a recorded response carries, given back as it came.
    #[serde(flatten)]
    rest: Map<String, Value>,
}

/// A comment in the general conversation, attached to no line.
#[derive(Deserialize, Serialize)]
pub struct IssueComment {
    id: u64,
    user: User,
    body: String,
    #[serde(flatten)]
    rest: Map<String, Value>,
}

#[derive(Deserialize, Serialize)]
pub struct Review {
    id: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    node_id: Option<String>,
    user: User,
    state: ReviewState,
    body: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    commit_id: Option<String>,
    #[serde(flatten)]
    rest: Map<String, Value>,
}

/// What `POST …/reviews` sends: with no `event`, the review stays pending.
#[derive(Deserialize)]
pub struct ReviewDraft {
    commit_id: Option<String>,
    /// `COMMENT`, `APPROVE` or `REQUEST_CHANGES`, which submits the review at once.
    event: Option<ReviewEvent>,
    #[serde(default)]
    body: String,
    #[serde(default)]
    comments: Vec<DraftComment>,
}

#[derive(Deserialize)]
struct DraftComment {
    path: String,
    line: Option<u64>,
    body: String,
    /// The anchor's side and range, kept as sent.
    #[serde(flatten)]
    rest: Map<String, Value>,
}

#[derive(Deserialize, Serialize)]
struct User {
    login: String,
    #[serde(flatten)]
    rest: Map<String, Value>,
}

impl User {
    fn named(login: &str) -> Self {
        User {
            login: login.to_string(),
            rest: Map::new(),
        }
    }
}

/// What GitHub turns down.
pub enum Rejection {
    ReviewFromTheAuthor,
    OnePendingReview,
    /// A draft created with an empty general comment can never be given one.
    MissingBody,
}

impl Rejection {
    /// What GitHub answers with it, word for word, as `record.sh` recorded it.
    pub fn answer(&self) -> Value {
        match self {
            Rejection::ReviewFromTheAuthor => json!({
                "message": "Review cannot be requested from pull request author.",
                "status": "422",
            }),
            Rejection::OnePendingReview => json!({
                "message": "Unprocessable Entity",
                "errors": ["User can only have one pending review per pull request"],
                "status": "422",
            }),
            Rejection::MissingBody => json!({
                "errors": [{"message": "Could not edit a review with a missing body."}],
            }),
        }
    }
}

impl PullRequest {
    pub fn load(described: &Path) -> Self {
        let described = fs::read_to_string(described).expect("the pull request");
        serde_json::from_str(&described).expect("the pull request as JSON")
    }

    pub fn save(&self, described: &Path) {
        let saved = serde_json::to_string(self).expect("the pull request as JSON");
        fs::write(described, saved).expect("the pull request is saved");
    }

    /// What REST lists for the pull request: a comment of a pending review stays out.
    pub fn submitted_comments(&self) -> Vec<&ReviewComment> {
        self.review_comments
            .iter()
            .filter(|comment| !self.is_pending(comment))
            .collect()
    }

    /// A comment of a pending review, or one its deleted review left behind.
    fn is_pending(&self, comment: &ReviewComment) -> bool {
        match comment.pull_request_review_id {
            Some(id) => self
                .review(id)
                .is_none_or(|review| review.state == ReviewState::Pending),
            None => false,
        }
    }

    pub fn comments_of_review(&self, id: u64) -> Vec<&ReviewComment> {
        self.review_comments
            .iter()
            .filter(|comment| comment.pull_request_review_id == Some(id))
            .collect()
    }

    pub fn review(&self, id: u64) -> Option<&Review> {
        self.reviews.iter().find(|review| review.id == id)
    }

    /// The pull request as `repos/OWNER/REPO/pulls/N` gives it, in the fields described.
    pub fn as_rest(&self) -> Value {
        json!({
            "number": self.number,
            "title": self.title,
            "state": self.state,
            "user": {"login": self.author},
            "body": self.body,
            "html_url": self.url(),
            "head": {"ref": self.head_ref, "sha": self.head_sha},
            "base": {"ref": self.base_ref},
        })
    }

    /// The fields `gh repo view --json` names, comma-separated.
    pub fn repository_viewed(&self, fields: &str) -> Option<Value> {
        picked(fields, |field| match field {
            "nameWithOwner" => Some(json!(self.repository)),
            _ => None,
        })
    }

    /// The fields `gh pr view --json` names, comma-separated.
    pub fn viewed(&self, fields: &str) -> Option<Value> {
        picked(fields, |field| match field {
            "body" => Some(json!(self.body)),
            "headRefOid" => Some(json!(self.head_sha)),
            "number" => Some(json!(self.number)),
            "title" => Some(json!(self.title)),
            "state" => Some(json!(self.state.as_deref().map(str::to_uppercase))),
            "headRefName" => Some(json!(self.head_ref)),
            "baseRefName" => Some(json!(self.base_ref)),
            "reviews" => Some(self.reviews.iter().map(Review::as_viewed).collect()),
            "comments" => Some(
                self.issue_comments
                    .iter()
                    .map(IssueComment::as_viewed)
                    .collect(),
            ),
            "author" => Some(json!({"login": self.author})),
            "url" => Some(json!(self.url())),
            _ => None,
        })
    }

    /// What `gh pr view --comments` prints: each review with a body, then the general
    /// conversation.
    pub fn comments_as_text(&self) -> String {
        let reviews = self
            .reviews
            .iter()
            .filter(|review| !review.body.is_empty())
            .map(|review| {
                as_text(
                    &review.user,
                    &review.rest,
                    review.state.as_text(),
                    &review.body,
                )
            });
        let conversation = self
            .issue_comments
            .iter()
            .map(|comment| as_text(&comment.user, &comment.rest, "none", &comment.body));
        reviews.chain(conversation).collect()
    }

    /// What the skill's GraphQL query asks for, whatever fields a query selects.
    pub fn review_threads(&self) -> Value {
        let threads: Vec<Value> = self
            .review_comments
            .iter()
            .filter(|comment| comment.in_reply_to_id.is_none())
            .map(|first| self.thread(first))
            .collect();
        let reviews: Vec<Value> = self.reviews.iter().map(Review::as_graphql).collect();
        json!({"data": {"repository": {"pullRequest": {
            "author": {"login": self.author},
            "reviewThreads": {"nodes": threads},
            "reviews": {"nodes": reviews},
        }}}})
    }

    /// REST lists replies flat, each naming the comment that opened its thread.
    fn thread(&self, first: &ReviewComment) -> Value {
        let nodes: Vec<Value> = self
            .review_comments
            .iter()
            .filter(|comment| comment.id == first.id || comment.in_reply_to_id == Some(first.id))
            .map(|comment| self.comment_as_graphql(comment))
            .collect();
        json!({
            "id": thread_id(first.id),
            "isResolved": self.resolved_threads.contains(&first.id),
            "isOutdated": false,
            "path": first.path,
            "line": first.line,
            "originalLine": first.line,
            "comments": {"nodes": nodes},
        })
    }

    fn comment_as_graphql(&self, comment: &ReviewComment) -> Value {
        let review = comment
            .pull_request_review_id
            .and_then(|id| self.review(id));
        json!({
            "databaseId": comment.id,
            "author": {"login": comment.user.login},
            "body": comment.body,
            "url": comment.html_url,
            "state": if self.is_pending(comment) { "PENDING" } else { "SUBMITTED" },
            "pullRequestReview": review.map(|review| json!({"databaseId": review.id})),
        })
    }

    /// Adds the requester's reply to the thread the answered comment opened, published at
    /// once, and gives it back as GitHub does.
    pub fn reply_to(&mut self, answered: u64, body: &str) -> &ReviewComment {
        self.add_reply(answered, body, None)
            .expect("the answered comment exists")
    }

    /// Adds the requester's reply to a thread inside the pending review whose node id is
    /// given, and tells the reply's id.
    pub fn reply_in_draft(&mut self, thread: &str, review: &str, body: &str) -> Option<u64> {
        let review = self.review_by_node(review)?.id;
        let reply = self.add_reply(first_of_thread(thread)?, body, Some(review))?;
        Some(reply.id)
    }

    fn add_reply(
        &mut self,
        answered: u64,
        body: &str,
        review: Option<u64>,
    ) -> Option<&ReviewComment> {
        let first = self
            .review_comments
            .iter()
            .find(|comment| comment.id == answered)?;
        let id = self.next_id();
        let reply = ReviewComment {
            id,
            pull_request_review_id: review,
            in_reply_to_id: Some(answered),
            path: first.path.clone(),
            line: first.line,
            user: self.requester(),
            body: body.to_string(),
            html_url: Some(self.discussion_url(id)),
            rest: Map::new(),
        };
        self.review_comments.push(reply);
        self.review_comments.last()
    }

    pub fn resolve(&mut self, thread: &str) -> Option<()> {
        let first = first_of_thread(thread)?;
        self.resolved_threads.push(first);
        Some(())
    }

    pub fn unresolve(&mut self, thread: &str) -> Option<()> {
        let first = first_of_thread(thread)?;
        self.resolved_threads.retain(|resolved| *resolved != first);
        Some(())
    }

    /// Removes the review and nothing else: its inline comments stay, pending, with no review.
    pub fn delete_review(&mut self, review: u64) -> Option<Review> {
        let at = self.reviews.iter().position(|found| found.id == review)?;
        Some(self.reviews.remove(at))
    }

    pub fn delete_comment(&mut self, comment: u64) -> Option<ReviewComment> {
        let at = self
            .review_comments
            .iter()
            .position(|found| found.id == comment)?;
        Some(self.review_comments.remove(at))
    }

    pub fn submit(&mut self, review: u64, event: ReviewEvent) -> Option<&Review> {
        let review = self.reviews.iter_mut().find(|found| found.id == review)?;
        review.state = ReviewState::sent_with(Some(event));
        Some(review)
    }

    pub fn replace_the_description(&mut self, body: String) {
        self.body = Some(body);
    }

    /// Gives the review whose node id is given a new general comment, and tells its state.
    pub fn replace_the_general_comment(
        &mut self,
        review: &str,
        body: String,
    ) -> Option<Result<ReviewState, Rejection>> {
        let review = self
            .reviews
            .iter_mut()
            .find(|found| found.node_id.as_deref() == Some(review))?;
        if review.body.is_empty() {
            return Some(Err(Rejection::MissingBody));
        }
        review.body = body;
        Some(Ok(review.state))
    }

    fn review_by_node(&self, node: &str) -> Option<&Review> {
        self.reviews
            .iter()
            .find(|review| review.node_id.as_deref() == Some(node))
    }

    /// Opens a thread inside the pending review whose node id is given, and names it.
    pub fn add_thread(
        &mut self,
        review: &str,
        path: String,
        line: u64,
        body: String,
    ) -> Option<String> {
        let review = self.review_by_node(review)?.id;
        let comment = DraftComment {
            path,
            line: Some(line),
            body,
            rest: Map::new(),
        };
        Some(thread_id(self.open_thread(review, comment)))
    }

    /// Posts the requester's comment to the general conversation, and tells where it is.
    pub fn comment(&mut self, body: &str) -> String {
        let id = self.next_id();
        let comment = IssueComment {
            id,
            user: self.requester(),
            body: body.to_string(),
            rest: Map::new(),
        };
        self.issue_comments.push(comment);
        format!("{}#issuecomment-{id}", self.url())
    }

    pub fn request_review_from(&self, reviewer: &str) -> Result<Value, Rejection> {
        if self.author.as_deref() == Some(reviewer) {
            return Err(Rejection::ReviewFromTheAuthor);
        }
        Ok(json!({"requested_reviewers": [{"login": reviewer}]}))
    }

    /// Opens a review as the requester, with its inline comments in it.
    pub fn create_review(&mut self, draft: ReviewDraft) -> Result<&Review, Rejection> {
        let requester = self.requester();
        let pending = |review: &Review| {
            review.user.login == requester.login && review.state == ReviewState::Pending
        };
        if self.reviews.iter().any(pending) {
            return Err(Rejection::OnePendingReview);
        }
        let id = self.next_id();
        let review = Review {
            id,
            node_id: Some(format!("PRR_{id}")),
            user: self.requester(),
            state: ReviewState::sent_with(draft.event),
            body: draft.body,
            commit_id: draft.commit_id,
            rest: Map::new(),
        };
        self.reviews.push(review);
        for comment in draft.comments {
            self.open_thread(id, comment);
        }
        Ok(self.review(id).expect("the review"))
    }

    /// Opens a thread inside a review with the requester's comment, and tells its id.
    fn open_thread(&mut self, review: u64, comment: DraftComment) -> u64 {
        let id = self.next_id();
        let comment = ReviewComment {
            id,
            pull_request_review_id: Some(review),
            in_reply_to_id: None,
            path: comment.path,
            line: comment.line,
            user: self.requester(),
            body: comment.body,
            html_url: Some(self.discussion_url(id)),
            rest: comment.rest,
        };
        self.review_comments.push(comment);
        id
    }

    fn requester(&self) -> User {
        User::named(self.viewer.as_deref().expect("the viewer"))
    }

    fn next_id(&self) -> u64 {
        let comments = self.review_comments.iter().map(|comment| comment.id);
        let last = comments
            .chain(self.reviews.iter().map(|review| review.id))
            .chain(self.issue_comments.iter().map(|comment| comment.id))
            .max();
        last.unwrap_or(0) + 1
    }

    /// Where the pull request lives on GitHub, which `gh pr edit` prints.
    pub fn url(&self) -> String {
        let Self {
            repository, number, ..
        } = self;
        format!("https://github.com/{repository}/pull/{number}")
    }

    fn discussion_url(&self, id: u64) -> String {
        format!("{}#discussion_r{id}", self.url())
    }
}

/// GraphQL names a thread after the comment that opened it.
fn thread_id(first: u64) -> String {
    format!("PRRT_{first}")
}

fn first_of_thread(thread: &str) -> Option<u64> {
    thread.strip_prefix("PRRT_")?.parse().ok()
}

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewState {
    Pending,
    Commented,
    Approved,
    ChangesRequested,
    Dismissed,
}

/// What submits a review, sent with it or after it.
#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReviewEvent {
    Comment,
    Approve,
    RequestChanges,
}

impl ReviewState {
    /// As `gh pr view --comments` writes it.
    fn as_text(self) -> &'static str {
        match self {
            ReviewState::Pending => "pending",
            ReviewState::Commented => "commented",
            ReviewState::Approved => "approved",
            ReviewState::ChangesRequested => "changes requested",
            ReviewState::Dismissed => "dismissed",
        }
    }

    /// The state a review takes from the event it is sent with, pending without one.
    fn sent_with(event: Option<ReviewEvent>) -> Self {
        match event {
            None => ReviewState::Pending,
            Some(ReviewEvent::Comment) => ReviewState::Commented,
            Some(ReviewEvent::Approve) => ReviewState::Approved,
            Some(ReviewEvent::RequestChanges) => ReviewState::ChangesRequested,
        }
    }
}

impl Review {
    /// As `gh pr view --json reviews` lists it.
    fn as_viewed(&self) -> Value {
        json!({
            "id": self.node_id,
            "author": {"login": self.user.login},
            "authorAssociation": self.rest["author_association"],
            "body": self.body,
            "state": self.state,
        })
    }

    fn as_graphql(&self) -> Value {
        json!({
            "author": {"login": self.user.login},
            "state": self.state,
            "body": self.body,
        })
    }
}

impl IssueComment {
    /// As `gh pr view --json comments` lists it.
    fn as_viewed(&self) -> Value {
        json!({
            "author": {"login": self.user.login},
            "authorAssociation": self.rest["author_association"],
            "body": self.body,
        })
    }
}

/// One entry of `gh pr view --comments`.
fn as_text(user: &User, rest: &Map<String, Value>, status: &str, body: &str) -> String {
    let association = rest["author_association"].as_str().unwrap_or("NONE");
    format!(
        "author:\t{}\nassociation:\t{}\nedited:\tfalse\nstatus:\t{status}\n--\n{body}\n--\n",
        user.login,
        association.to_lowercase()
    )
}

/// The object `--json` prints, one key per field named, or nothing when one is unknown.
fn picked(fields: &str, value_of: impl Fn(&str) -> Option<Value>) -> Option<Value> {
    let mut picked = Map::new();
    for field in fields.split(',') {
        picked.insert(field.to_string(), value_of(field)?);
    }
    Some(Value::Object(picked))
}
