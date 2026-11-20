//! What the stand-in for `gh` answers to the calls of `reviewing-pull-requests`: a review
//! kept pending, appended to, deleted and submitted, and the threads it opens.

mod common;

use std::fs;

use serde_json::json;

use common::{gh, gh_fed, gh_text, recorded_calls, run, run_fed, workspace_impersonating};

/// The query `reviewing-pull-requests` runs to find comments a deleted draft left behind.
const LEAK_CHECK_QUERY: &str = "query=query($owner:String!,$name:String!,$n:Int!){ repository(owner:$owner,name:$name){ pullRequest(number:$n){ reviewThreads(first:100){ nodes{ comments(first:100){ nodes{ databaseId state pullRequestReview{ databaseId } } } } } } } }";

/// The mutation `reviewing-pull-requests` appends an inline comment to an open draft with.
const ADD_THREAD_MUTATION: &str = "query=mutation($rid:ID!,$body:String!,$path:String!,$line:Int!){ addPullRequestReviewThread(input:{pullRequestReviewId:$rid, body:$body, path:$path, line:$line, side:RIGHT}){ thread { id } } }";

/// The mutation `reviewing-pull-requests` rewrites the general comment of an open draft with.
const UPDATE_REVIEW_MUTATION: &str = "query=mutation($rid:ID!,$body:String!){ updatePullRequestReview(input:{pullRequestReviewId:$rid, body:$body}){ pullRequestReview { state } } }";

/// The mutation `reviewing-pull-requests` puts a pending reply into a thread with.
const ADD_REPLY_MUTATION: &str = "query=mutation($t:ID!,$r:ID!,$b:String!){ addPullRequestReviewThreadReply(input:{pullRequestReviewThreadId:$t, pullRequestReviewId:$r, body:$b}){ comment{ databaseId state } } }";

/// The mutation `reviewing-pull-requests` resolves a thread with.
const RESOLVE_MUTATION: &str =
    "query=mutation($t:ID!){ resolveReviewThread(input:{threadId:$t}){ thread{ isResolved } } }";

/// The mutation `reviewing-pull-requests` takes a resolution back with.
const UNRESOLVE_MUTATION: &str =
    "query=mutation($t:ID!){ unresolveReviewThread(input:{threadId:$t}){ thread{ isResolved } } }";

/// The query `reviewing-pull-requests` reads the threads of a follow-up with.
const REVIEW_THREADS: [&str; 10] = [
    "api",
    "graphql",
    "-f",
    "query=query($owner:String!,$name:String!,$n:Int!){ repository(owner:$owner,name:$name){ pullRequest(number:$n){ reviewThreads(first:100){ nodes{ id isResolved isOutdated path line originalLine comments(first:100){ nodes{ url body state author{ login } } } } } } } }",
    "-f",
    "owner=acme",
    "-f",
    "name=shop",
    "-F",
    "n=1",
];

const THREADS_RESOLVED: &str =
    "[.data.repository.pullRequest.reviewThreads.nodes[] | {id, isResolved}]";

#[test]
fn shows_the_head_commit_of_a_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "head_sha": "9fceb02"
    }));

    let answer = gh(&workspace, &["pr", "view", "1", "--json", "headRefOid"]);

    assert_eq!(answer, json!({"headRefOid": "9fceb02"}));
}

#[test]
fn lists_the_reviews_of_a_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {
                "id": 31, "node_id": "PRR_31", "user": {"login": "reviewer"},
                "state": "COMMENTED", "body": "Two things to fix.", "commit_id": "9fceb02"
            }
        ]
    }));

    let answer = gh(&workspace, &["api", "repos/acme/shop/pulls/1/reviews"]);

    assert_eq!(
        answer,
        json!([
            {
                "id": 31, "node_id": "PRR_31", "user": {"login": "reviewer"},
                "state": "COMMENTED", "body": "Two things to fix.", "commit_id": "9fceb02"
            }
        ])
    );
}

#[test]
fn shows_one_review() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {"id": 31, "user": {"login": "reviewer"}, "state": "COMMENTED", "body": "Two things to fix."},
            {"id": 32, "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ]
    }));

    let answer = gh(&workspace, &["api", "repos/acme/shop/pulls/1/reviews/32"]);

    assert_eq!(
        answer,
        json!({"id": 32, "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."})
    );
}

#[test]
fn lists_the_comments_of_one_review() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {"id": 31, "user": {"login": "reviewer"}, "state": "COMMENTED", "body": "Two things to fix."},
            {"id": 32, "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ],
        "review_comments": [
            {
                "id": 41, "pull_request_review_id": 31, "path": "README.md", "line": 10,
                "user": {"login": "reviewer"}, "body": "Stale example."
            },
            {
                "id": 42, "pull_request_review_id": 32, "path": "src/cart.rs", "line": 3,
                "user": {"login": "developer"}, "body": "Never called."
            }
        ]
    }));

    let answer = gh(
        &workspace,
        &["api", "repos/acme/shop/pulls/1/reviews/32/comments"],
    );

    assert_eq!(
        answer,
        json!([{
            "id": 42, "pull_request_review_id": 32, "path": "src/cart.rs", "line": 3,
            "user": {"login": "developer"}, "body": "Never called."
        }])
    );
}

#[test]
fn lists_only_the_submitted_comments_of_a_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {"id": 31, "user": {"login": "reviewer"}, "state": "COMMENTED", "body": "Two things to fix."},
            {"id": 32, "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ],
        "review_comments": [
            {
                "id": 41, "pull_request_review_id": 31, "path": "README.md", "line": 10,
                "user": {"login": "reviewer"}, "body": "Stale example."
            },
            {
                "id": 42, "pull_request_review_id": 32, "path": "src/cart.rs", "line": 3,
                "user": {"login": "developer"}, "body": "Never called."
            }
        ]
    }));

    let answer = gh(&workspace, &["api", "repos/acme/shop/pulls/1/comments"]);

    assert_eq!(
        answer,
        json!([{
            "id": 41, "pull_request_review_id": 31, "path": "README.md", "line": 10,
            "user": {"login": "reviewer"}, "body": "Stale example."
        }])
    );
}

#[test]
fn tells_the_state_and_review_of_each_comment_in_a_thread() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {"id": 31, "user": {"login": "reviewer"}, "state": "COMMENTED", "body": "Two things to fix."},
            {"id": 32, "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ],
        "review_comments": [
            {
                "id": 41, "pull_request_review_id": 31, "path": "README.md", "line": 10,
                "user": {"login": "reviewer"}, "body": "Stale example."
            },
            {
                "id": 43, "pull_request_review_id": 32, "in_reply_to_id": 41, "path": "README.md", "line": 10,
                "user": {"login": "developer"}, "body": "Updated."
            }
        ]
    }));

    let answer = gh(
        &workspace,
        &[
            "api",
            "graphql",
            "-f",
            LEAK_CHECK_QUERY,
            "-f",
            "owner=acme",
            "-f",
            "name=shop",
            "-F",
            "n=1",
        ],
    );

    assert_eq!(
        answer["data"]["repository"]["pullRequest"]["reviewThreads"],
        json!({"nodes": [{
            "id": "PRRT_41", "isResolved": false, "isOutdated": false,
            "path": "README.md", "line": 10, "originalLine": 10,
            "comments": {"nodes": [
                {
                    "databaseId": 41, "author": {"login": "reviewer"}, "body": "Stale example.", "url": null,
                    "state": "SUBMITTED", "pullRequestReview": {"databaseId": 31}
                },
                {
                    "databaseId": 43, "author": {"login": "developer"}, "body": "Updated.", "url": null,
                    "state": "PENDING", "pullRequestReview": {"databaseId": 32}
                }
            ]}
        }]})
    );
}

#[test]
fn creates_a_pending_review_from_a_payload_file() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer"
    }));
    let payload = json!({
        "commit_id": "9fceb02",
        "body": "Worth a look.",
        "comments": [{"path": "src/cart.rs", "line": 3, "side": "RIGHT", "body": "Never called."}]
    });
    fs::write(workspace.path().join("review.json"), payload.to_string()).expect("the payload");

    let created = gh(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/reviews",
            "--input",
            "review.json",
        ],
    );

    assert_eq!(
        created,
        json!({
            "id": 1, "node_id": "PRR_1", "user": {"login": "developer"},
            "state": "PENDING", "body": "Worth a look.", "commit_id": "9fceb02"
        })
    );
    let comments = gh(
        &workspace,
        &[
            "api",
            "repos/acme/shop/pulls/1/reviews/1/comments",
            "-q",
            "[.[] | {path, line, side, body}]",
        ],
    );
    assert_eq!(
        comments,
        json!([{"path": "src/cart.rs", "line": 3, "side": "RIGHT", "body": "Never called."}])
    );
}

#[test]
fn creates_a_pending_review_when_the_method_is_given_as_its_short_option() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer"
    }));
    let payload = json!({"commit_id": "9fceb02", "body": "Worth a look.", "comments": []});
    fs::write(workspace.path().join("review.json"), payload.to_string()).expect("the payload");

    let created = gh(
        &workspace,
        &[
            "api",
            "-X",
            "POST",
            "repos/acme/shop/pulls/1/reviews",
            "--input",
            "review.json",
        ],
    );

    assert_eq!(
        created,
        json!({
            "id": 1, "node_id": "PRR_1", "user": {"login": "developer"},
            "state": "PENDING", "body": "Worth a look.", "commit_id": "9fceb02"
        })
    );
}

#[test]
fn reads_the_payload_of_a_review_from_standard_input() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer"
    }));
    let payload = json!({"commit_id": "9fceb02", "body": "Worth a look.", "comments": []});

    let created = gh_fed(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/reviews",
            "--input",
            "-",
        ],
        &payload.to_string(),
    );

    assert_eq!(
        created,
        json!({
            "id": 1, "node_id": "PRR_1", "user": {"login": "developer"},
            "state": "PENDING", "body": "Worth a look.", "commit_id": "9fceb02"
        })
    );
}

#[test]
fn publishes_a_review_created_with_an_event() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer"
    }));
    let payload = json!({"commit_id": "9fceb02", "body": "Worth a look.", "event": "COMMENT"});

    let created = gh_fed(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/reviews",
            "--input",
            "-",
        ],
        &payload.to_string(),
    );

    assert_eq!(
        created,
        json!({
            "id": 1, "node_id": "PRR_1", "user": {"login": "developer"},
            "state": "COMMENTED", "body": "Worth a look.", "commit_id": "9fceb02"
        })
    );
}

#[test]
fn adds_a_thread_to_a_pending_review() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer",
        "reviews": [
            {"id": 32, "node_id": "PRR_32", "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ]
    }));

    let added = gh(
        &workspace,
        &[
            "api",
            "graphql",
            "-f",
            ADD_THREAD_MUTATION,
            "-f",
            "rid=PRR_32",
            "-f",
            "body=Never called.",
            "-f",
            "path=src/cart.rs",
            "-F",
            "line=3",
        ],
    );

    assert_eq!(
        added,
        json!({"data": {"addPullRequestReviewThread": {"thread": {"id": "PRRT_33"}}}})
    );
    let comments = gh(
        &workspace,
        &[
            "api",
            "repos/acme/shop/pulls/1/reviews/32/comments",
            "-q",
            "[.[] | {id, path, line, body}]",
        ],
    );
    assert_eq!(
        comments,
        json!([{"id": 33, "path": "src/cart.rs", "line": 3, "body": "Never called."}])
    );
}

#[test]
fn replaces_the_general_comment_of_a_pending_review() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer",
        "reviews": [
            {"id": 32, "node_id": "PRR_32", "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ]
    }));

    let updated = gh(
        &workspace,
        &[
            "api",
            "graphql",
            "-f",
            UPDATE_REVIEW_MUTATION,
            "-f",
            "rid=PRR_32",
            "-f",
            "body=A draft.\nOne more point.",
        ],
    );

    assert_eq!(
        updated,
        json!({"data": {"updatePullRequestReview": {"pullRequestReview": {"state": "PENDING"}}}})
    );
    let body = gh(
        &workspace,
        &["api", "repos/acme/shop/pulls/1/reviews/32", "-q", "{body}"],
    );
    assert_eq!(body, json!({"body": "A draft.\nOne more point."}));
}

#[test]
fn adds_a_pending_reply_to_a_thread() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer",
        "reviews": [
            {"id": 31, "user": {"login": "developer"}, "state": "COMMENTED", "body": "Two things to fix."},
            {"id": 32, "node_id": "PRR_32", "user": {"login": "developer"}, "state": "PENDING", "body": ""}
        ],
        "review_comments": [
            {
                "id": 41, "pull_request_review_id": 31, "path": "README.md", "line": 10,
                "user": {"login": "developer"}, "body": "Stale example."
            }
        ]
    }));

    let replied = gh(
        &workspace,
        &[
            "api",
            "graphql",
            "-f",
            ADD_REPLY_MUTATION,
            "-f",
            "t=PRRT_41",
            "-f",
            "r=PRR_32",
            "-f",
            "b=Fixed, thanks.",
        ],
    );

    assert_eq!(
        replied,
        json!({"data": {"addPullRequestReviewThreadReply": {"comment": {"databaseId": 42, "state": "PENDING"}}}})
    );
    let comments = gh(
        &workspace,
        &[
            "api",
            "repos/acme/shop/pulls/1/reviews/32/comments",
            "-q",
            "[.[] | {id, in_reply_to_id, body}]",
        ],
    );
    assert_eq!(
        comments,
        json!([{"id": 42, "in_reply_to_id": 41, "body": "Fixed, thanks."}])
    );
}

#[test]
fn resolves_a_thread() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [
            {"id": 41, "path": "README.md", "line": 10, "user": {"login": "developer"}, "body": "Stale example."},
            {"id": 42, "path": "src/cart.rs", "line": 3, "user": {"login": "developer"}, "body": "Never called."}
        ]
    }));

    let resolved = gh(
        &workspace,
        &["api", "graphql", "-f", RESOLVE_MUTATION, "-f", "t=PRRT_41"],
    );

    assert_eq!(
        resolved,
        json!({"data": {"resolveReviewThread": {"thread": {"isResolved": true}}}})
    );
    let threads = gh(
        &workspace,
        &[&REVIEW_THREADS[..], &["-q", THREADS_RESOLVED]].concat(),
    );
    assert_eq!(
        threads,
        json!([{"id": "PRRT_41", "isResolved": true}, {"id": "PRRT_42", "isResolved": false}])
    );
}

#[test]
fn unresolves_a_thread() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [
            {"id": 41, "path": "README.md", "line": 10, "user": {"login": "developer"}, "body": "Stale example."}
        ],
        "resolved_threads": [41]
    }));

    let unresolved = gh(
        &workspace,
        &[
            "api",
            "graphql",
            "-f",
            UNRESOLVE_MUTATION,
            "-f",
            "t=PRRT_41",
        ],
    );

    assert_eq!(
        unresolved,
        json!({"data": {"unresolveReviewThread": {"thread": {"isResolved": false}}}})
    );
    let threads = gh(
        &workspace,
        &[&REVIEW_THREADS[..], &["-q", THREADS_RESOLVED]].concat(),
    );
    assert_eq!(threads, json!([{"id": "PRRT_41", "isResolved": false}]));
}

#[test]
fn submits_a_pending_review() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {"id": 32, "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ],
        "review_comments": [
            {
                "id": 42, "pull_request_review_id": 32, "path": "src/cart.rs", "line": 3,
                "user": {"login": "developer"}, "body": "Never called."
            }
        ]
    }));

    let submitted = gh(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/reviews/32/events",
            "-f",
            "event=COMMENT",
        ],
    );

    assert_eq!(
        submitted,
        json!({"id": 32, "user": {"login": "developer"}, "state": "COMMENTED", "body": "A draft."})
    );
    let published = gh(
        &workspace,
        &["api", "repos/acme/shop/pulls/1/comments", "-q", "[.[].id]"],
    );
    assert_eq!(published, json!([42]));
}

#[test]
fn deletes_a_pending_review_leaving_its_comments_behind() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {"id": 32, "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ],
        "review_comments": [
            {
                "id": 42, "pull_request_review_id": 32, "path": "src/cart.rs", "line": 3,
                "user": {"login": "developer"}, "body": "Never called."
            }
        ]
    }));

    gh_text(
        &workspace,
        &[
            "api",
            "--method",
            "DELETE",
            "repos/acme/shop/pulls/1/reviews/32",
        ],
    );

    let reviews = gh(&workspace, &["api", "repos/acme/shop/pulls/1/reviews"]);
    assert_eq!(reviews, json!([]));
    let left_behind = gh(
        &workspace,
        &[
            "api",
            "graphql",
            "-f",
            LEAK_CHECK_QUERY,
            "-f",
            "owner=acme",
            "-f",
            "name=shop",
            "-F",
            "n=1",
            "-q",
            "[.data.repository.pullRequest.reviewThreads.nodes[].comments.nodes[] | {databaseId, state, pullRequestReview}]",
        ],
    );
    assert_eq!(
        left_behind,
        json!([{"databaseId": 42, "state": "PENDING", "pullRequestReview": null}])
    );
}

#[test]
fn deletes_a_comment_by_its_id() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [
            {
                "id": 42, "pull_request_review_id": 32, "path": "src/cart.rs", "line": 3,
                "user": {"login": "developer"}, "body": "Never called."
            },
            {
                "id": 43, "pull_request_review_id": 32, "path": "README.md", "line": 10,
                "user": {"login": "developer"}, "body": "Stale example."
            }
        ]
    }));

    gh_text(
        &workspace,
        &[
            "api",
            "--method",
            "DELETE",
            "repos/acme/shop/pulls/comments/42",
        ],
    );

    let left_behind = gh(
        &workspace,
        &[
            "api",
            "graphql",
            "-f",
            LEAK_CHECK_QUERY,
            "-f",
            "owner=acme",
            "-f",
            "name=shop",
            "-F",
            "n=1",
            "-q",
            "[.data.repository.pullRequest.reviewThreads.nodes[].comments.nodes[].databaseId]",
        ],
    );
    assert_eq!(left_behind, json!([43]));
}

#[test]
fn refuses_a_second_pending_review() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer",
        "reviews": [
            {"id": 32, "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ]
    }));
    let payload = json!({"commit_id": "9fceb02", "body": "Worth a look."});

    let refusal = run_fed(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/reviews",
            "--input",
            "-",
        ],
        &payload.to_string(),
    );

    assert_eq!(
        (
            refusal.status.code(),
            String::from_utf8_lossy(&refusal.stderr).into_owned()
        ),
        (Some(1), "gh: Unprocessable Entity (HTTP 422)\n".to_string())
    );
}

#[test]
fn refuses_a_general_comment_for_a_review_created_without_one() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {"id": 32, "node_id": "PRR_32", "user": {"login": "developer"}, "state": "PENDING", "body": ""}
        ]
    }));

    let refusal = run(
        workspace.path(),
        &[
            "api",
            "graphql",
            "-f",
            UPDATE_REVIEW_MUTATION,
            "-f",
            "rid=PRR_32",
            "-f",
            "body=One more point.",
        ],
    );

    assert_eq!(
        (
            refusal.status.code(),
            String::from_utf8_lossy(&refusal.stderr).into_owned()
        ),
        (
            Some(1),
            "gh: Could not edit a review with a missing body.\n".to_string()
        )
    );
}

#[test]
fn answers_not_found_for_a_comment_already_deleted() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": []
    }));

    let refusal = run(
        workspace.path(),
        &[
            "api",
            "--method",
            "DELETE",
            "repos/acme/shop/pulls/comments/42",
        ],
    );

    assert_eq!(
        (
            refusal.status.code(),
            String::from_utf8_lossy(&refusal.stderr).into_owned()
        ),
        (Some(1), "gh: Not Found (HTTP 404)\n".to_string())
    );
}

#[test]
fn rejects_an_unknown_graphql_operation_as_unhandled() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1
    }));
    let mutation = "query=mutation($rid:ID!){ addPullRequestReviewComment(input:{pullRequestReviewId:$rid, body:\"x\"}){ comment{ id } } }";

    let refusal = run(
        workspace.path(),
        &["api", "graphql", "-f", mutation, "-f", "rid=PRR_32"],
    );

    assert_eq!(
        (
            refusal.status.code(),
            String::from_utf8_lossy(&refusal.stderr).into_owned()
        ),
        (
            Some(1),
            format!("gh: `gh api graphql -f {mutation} -f rid=PRR_32` is not available here\n")
        )
    );
    assert_eq!(
        recorded_calls(&workspace),
        vec![
            json!({"args": ["api", "graphql", "-f", mutation, "-f", "rid=PRR_32"], "unhandled": true})
        ]
    );
}

#[test]
fn logs_the_payload_a_review_is_created_from() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer"
    }));
    let payload = json!({
        "commit_id": "9fceb02",
        "body": "Worth a look.",
        "comments": [{"path": "src/cart.rs", "line": 3, "side": "RIGHT", "body": "Never called."}]
    });
    fs::write(workspace.path().join("review.json"), payload.to_string()).expect("the payload");
    let args = [
        "api",
        "--method",
        "POST",
        "repos/acme/shop/pulls/1/reviews",
        "--input",
        "review.json",
    ];

    run(workspace.path(), &args);

    assert_eq!(
        recorded_calls(&workspace),
        vec![json!({"args": args, "input": payload})]
    );
}

#[test]
fn logs_the_payload_a_review_is_created_from_on_standard_input() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer"
    }));
    let payload = json!({"commit_id": "9fceb02", "body": "Worth a look.", "comments": []});
    let args = [
        "api",
        "--method",
        "POST",
        "repos/acme/shop/pulls/1/reviews",
        "--input",
        "-",
    ];

    run_fed(&workspace, &args, &payload.to_string());

    assert_eq!(
        recorded_calls(&workspace),
        vec![json!({"args": args, "input": payload})]
    );
}

#[test]
fn tells_an_outdated_thread_and_the_line_it_was_written_on() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [{"id": 31, "user": {"login": "reviewer"}, "state": "COMMENTED", "body": ""}],
        "review_comments": [{
            "id": 41, "pull_request_review_id": 31, "path": "src/cart.rs",
            "line": null, "original_line": 8,
            "user": {"login": "reviewer"}, "body": "Worth a `sum()`."
        }]
    }));

    let threads = gh(
        &workspace,
        &[
            &REVIEW_THREADS[..],
            &["-q", ".data.repository.pullRequest.reviewThreads.nodes[] | {isOutdated, line, originalLine}"],
        ]
        .concat(),
    );

    assert_eq!(
        threads,
        json!({"isOutdated": true, "line": null, "originalLine": 8})
    );
}

#[test]
fn adds_a_thread_from_a_query_and_its_variables_given_as_a_file() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer",
        "reviews": [
            {"id": 32, "node_id": "PRR_32", "user": {"login": "developer"}, "state": "PENDING", "body": "A draft."}
        ]
    }));
    let request = json!({
        "query": ADD_THREAD_MUTATION.strip_prefix("query=").expect("the query"),
        "variables": {"rid": "PRR_32", "body": "Never called.", "path": "src/cart.rs", "line": 3}
    });
    fs::write(workspace.path().join("thread.json"), request.to_string()).expect("the request");

    let added = gh(&workspace, &["api", "graphql", "--input", "thread.json"]);

    assert_eq!(
        added,
        json!({"data": {"addPullRequestReviewThread": {"thread": {"id": "PRRT_33"}}}})
    );
}

#[test]
fn creates_a_pending_review_from_fields() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer"
    }));

    let created = gh(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/reviews",
            "-f",
            "commit_id=9fceb02",
            "-f",
            "body=Worth a look.",
        ],
    );

    assert_eq!(
        created,
        json!({
            "id": 1, "node_id": "PRR_1", "user": {"login": "developer"},
            "state": "PENDING", "body": "Worth a look.", "commit_id": "9fceb02"
        })
    );
}
