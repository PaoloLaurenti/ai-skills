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
    #[serde(default)]
    pub review_comments: Vec<ReviewComment>,
    #[serde(default)]
    pub issue_comments: Vec<IssueComment>,
    #[serde(default)]
    reviews: Vec<Review>,
}

/// A comment on a line of the diff. A reply names the comment that opened its thread.
#[derive(Deserialize, Serialize)]
pub struct ReviewComment {
    id: u64,
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
struct Review {
    user: User,
    state: String,
    body: String,
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

    /// A REST path reaches only this pull request.
    pub fn is_reached_by(&self, path: &str) -> bool {
        let Self {
            repository, number, ..
        } = self;
        path.starts_with(&format!("repos/{repository}/pulls/{number}/"))
            || path.starts_with(&format!("repos/{repository}/issues/{number}/"))
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
            .map(ReviewComment::as_graphql)
            .collect();
        json!({
            "isResolved": false,
            "path": first.path,
            "line": first.line,
            "comments": {"nodes": nodes},
        })
    }

    /// Adds the requester's reply to the thread the answered comment opened, and gives it
    /// back as GitHub does.
    pub fn reply_to(&mut self, answered: u64, body: &str) -> &ReviewComment {
        let first = self
            .review_comments
            .iter()
            .find(|comment| comment.id == answered)
            .expect("the answered comment exists");
        let id = self.next_comment_id();
        let reply = ReviewComment {
            id,
            in_reply_to_id: Some(answered),
            path: first.path.clone(),
            line: first.line,
            user: self.requester(),
            body: body.to_string(),
            html_url: Some(self.discussion_url(id)),
            rest: Map::new(),
        };
        self.review_comments.push(reply);
        self.review_comments.last().expect("the reply")
    }

    pub fn replace_the_description(&mut self, body: String) {
        self.body = Some(body);
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

    fn requester(&self) -> User {
        User::named(self.viewer.as_deref().expect("the viewer"))
    }

    fn next_comment_id(&self) -> u64 {
        let last = self.review_comments.iter().map(|comment| comment.id).max();
        last.unwrap_or(0) + 1
    }

    fn discussion_url(&self, id: u64) -> String {
        let Self {
            repository, number, ..
        } = self;
        format!("https://github.com/{repository}/pull/{number}#discussion_r{id}")
    }
}

impl ReviewComment {
    fn as_graphql(&self) -> Value {
        json!({
            "databaseId": self.id,
            "author": {"login": self.user.login},
            "body": self.body,
            "url": self.html_url,
        })
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
