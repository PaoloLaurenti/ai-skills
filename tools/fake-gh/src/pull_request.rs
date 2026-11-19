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
    /// GitHub's own words for it.
    pub fn message(&self) -> &'static str {
        match self {
            Rejection::ReviewFromTheAuthor => {
                "Review cannot be requested from pull request author."
            }
            Rejection::OnePendingReview => "User can only have one pending review per pull request",
            Rejection::MissingBody => "Could not edit a review with a missing body",
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

    /// The fields `gh pr view --json` names, comma-separated.
    pub fn viewed(&self, fields: &str) -> Option<Value> {
        let mut viewed = Map::new();
        for field in fields.split(',') {
            let value = match field {
                "body" => json!(self.body),
                "headRefOid" => json!(self.head_sha),
                _ => return None,
            };
            viewed.insert(field.to_string(), value);
        }
        Some(Value::Object(viewed))
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

    pub fn comment(&mut self, body: &str) {
        let comment = IssueComment {
            user: self.requester(),
            body: body.to_string(),
            rest: Map::new(),
        };
        self.issue_comments.push(comment);
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
            .max();
        last.unwrap_or(0) + 1
    }

    fn discussion_url(&self, id: u64) -> String {
        let Self {
            repository, number, ..
        } = self;
        format!("https://github.com/{repository}/pull/{number}#discussion_r{id}")
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
    fn as_graphql(&self) -> Value {
        json!({
            "author": {"login": self.user.login},
            "state": self.state,
            "body": self.body,
        })
    }
}
