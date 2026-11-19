//! What the stand-in for `gh` answers about the pull request it impersonates.
//!
//! Each test runs the built `gh` the way Claude does, in a throwaway workspace whose
//! `.git/fake-gh/pull-request.json` describes the pull request.

mod common;

use std::fs;

use serde_json::json;
use tempfile::TempDir;

use common::{gh, gh_text, recorded_calls, run, workspace_impersonating};

/// The call `addressing-pull-request-reviews` makes for the review threads of acme/shop#1,
/// its query word for word.
const REVIEW_THREADS_OF_ACME_SHOP_1: [&str; 10] = [
    "api",
    "graphql",
    "-f",
    "query=query($owner:String!,$name:String!,$number:Int!){ repository(owner:$owner,name:$name){ pullRequest(number:$number){ author{login} reviewThreads(first:100){ nodes{ isResolved path line comments(first:50){ nodes{ databaseId author{login} body url } } } } reviews(first:100){ nodes{ author{login} state body } } } } }",
    "-f",
    "owner=acme",
    "-f",
    "name=shop",
    "-F",
    "number=1",
];

#[test]
fn lists_the_review_comment_of_a_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [
            {"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."}
        ]
    }));

    let answer = gh(&workspace, &["api", "repos/acme/shop/pulls/1/comments"]);

    assert_eq!(
        answer,
        json!([
            {"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."}
        ])
    );
}

#[test]
fn lists_every_review_comment_of_a_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [
            {"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."},
            {"id": 12, "path": "src/cart.rs", "line": 3, "user": {"login": "reviewer"}, "body": "Never called."}
        ]
    }));

    let answer = gh(&workspace, &["api", "repos/acme/shop/pulls/1/comments"]);

    assert_eq!(
        answer,
        json!([
            {"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."},
            {"id": 12, "path": "src/cart.rs", "line": 3, "user": {"login": "reviewer"}, "body": "Never called."}
        ])
    );
}

#[test]
fn filters_an_answer_with_its_jq_expression() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [
            {"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."},
            {"id": 12, "path": "src/cart.rs", "line": 3, "user": {"login": "reviewer"}, "body": "Never called."}
        ]
    }));

    let answer = gh_text(
        &workspace,
        &["api", "repos/acme/shop/pulls/1/comments", "-q", ".[].body"],
    );

    assert_eq!(answer, "Stale example.\nNever called.\n");
}

#[test]
fn records_every_call_for_the_graders() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": []
    }));

    gh(&workspace, &["api", "repos/acme/shop/pulls/1/comments"]);
    gh_text(
        &workspace,
        &["api", "repos/acme/shop/pulls/1/comments", "-q", "length"],
    );

    assert_eq!(
        recorded_calls(&workspace),
        vec![
            json!({"args": ["api", "repos/acme/shop/pulls/1/comments"]}),
            json!({"args": ["api", "repos/acme/shop/pulls/1/comments", "-q", "length"]}),
        ]
    );
}

#[test]
fn shows_the_description_of_a_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "body": "Rounds every total to the cent.",
        "review_comments": []
    }));

    let answer = gh(&workspace, &["pr", "view", "1", "--json", "body"]);

    assert_eq!(answer, json!({"body": "Rounds every total to the cent."}));
}

#[test]
fn lists_the_comments_of_the_general_conversation() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [],
        "issue_comments": [
            {"id": 21, "user": {"login": "reviewer"}, "body": "Looks good overall."}
        ]
    }));

    let answer = gh(
        &workspace,
        &["api", "repos/acme/shop/issues/1/comments", "--paginate"],
    );

    assert_eq!(
        answer,
        json!([{"id": 21, "user": {"login": "reviewer"}, "body": "Looks good overall."}])
    );
}

#[test]
fn lists_the_review_threads_with_their_comments() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "author": "developer",
        "review_comments": [
            {
                "id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"},
                "body": "Stale example.", "html_url": "https://github.com/acme/shop/pull/1#discussion_r11"
            }
        ],
        "reviews": []
    }));

    let answer = gh(&workspace, &REVIEW_THREADS_OF_ACME_SHOP_1);

    assert_eq!(
        answer,
        json!({"data": {"repository": {"pullRequest": {
            "author": {"login": "developer"},
            "reviewThreads": {"nodes": [{
                "id": "PRRT_11", "isResolved": false, "isOutdated": false,
                "path": "README.md", "line": 10, "originalLine": 10,
                "comments": {"nodes": [{
                    "databaseId": 11, "author": {"login": "reviewer"},
                    "body": "Stale example.", "url": "https://github.com/acme/shop/pull/1#discussion_r11",
                    "state": "SUBMITTED", "pullRequestReview": null
                }]}
            }]},
            "reviews": {"nodes": []}
        }}}})
    );
}

#[test]
fn groups_a_reply_into_the_thread_of_the_comment_it_answers() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "author": "developer",
        "review_comments": [
            {
                "id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"},
                "body": "Stale example.", "html_url": "https://github.com/acme/shop/pull/1#discussion_r11"
            },
            {
                "id": 12, "in_reply_to_id": 11, "path": "README.md", "line": 10, "user": {"login": "developer"},
                "body": "Updated.", "html_url": "https://github.com/acme/shop/pull/1#discussion_r12"
            }
        ],
        "reviews": []
    }));

    let answer = gh(&workspace, &REVIEW_THREADS_OF_ACME_SHOP_1);

    assert_eq!(
        answer["data"]["repository"]["pullRequest"]["reviewThreads"],
        json!({"nodes": [{
            "id": "PRRT_11", "isResolved": false, "isOutdated": false,
            "path": "README.md", "line": 10, "originalLine": 10,
            "comments": {"nodes": [
                {
                    "databaseId": 11, "author": {"login": "reviewer"},
                    "body": "Stale example.", "url": "https://github.com/acme/shop/pull/1#discussion_r11",
                    "state": "SUBMITTED", "pullRequestReview": null
                },
                {
                    "databaseId": 12, "author": {"login": "developer"},
                    "body": "Updated.", "url": "https://github.com/acme/shop/pull/1#discussion_r12",
                    "state": "SUBMITTED", "pullRequestReview": null
                }
            ]}
        }]})
    );
}

#[test]
fn lists_the_reviews_beside_the_review_threads() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "author": "developer",
        "review_comments": [],
        "reviews": [
            {"id": 31, "user": {"login": "reviewer"}, "state": "CHANGES_REQUESTED", "body": "Two things to fix."}
        ]
    }));

    let answer = gh(&workspace, &REVIEW_THREADS_OF_ACME_SHOP_1);

    assert_eq!(
        answer["data"]["repository"]["pullRequest"]["reviews"],
        json!({"nodes": [
            {"author": {"login": "reviewer"}, "state": "CHANGES_REQUESTED", "body": "Two things to fix."}
        ]})
    );
}

#[test]
fn shows_the_diff_of_a_pull_request() {
    let diff = "--- a/README.md\n+++ b/README.md\n@@ -10 +10 @@\n-Shows 20 C.\n+Shows 68 F.\n";
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "diff": diff
    }));

    let answer = gh_text(&workspace, &["pr", "diff", "1"]);

    assert_eq!(answer, diff);
}

#[test]
fn tells_the_authenticated_login() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer"
    }));

    let answer = gh(&workspace, &["api", "user"]);

    assert_eq!(answer, json!({"login": "developer"}));
}

#[test]
fn lists_the_open_pull_requests_of_the_author() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "author": "developer",
        "viewer": "developer"
    }));

    let answer = gh(
        &workspace,
        &[
            "pr",
            "list",
            "--repo",
            "acme/shop",
            "--author",
            "@me",
            "--state",
            "open",
            "--json",
            "number",
        ],
    );

    assert_eq!(answer, json!([{"number": 1}]));
}

#[test]
fn posts_a_reply_inside_its_thread() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer",
        "review_comments": [
            {"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."}
        ]
    }));

    gh_text(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/comments/11/replies",
            "-f",
            "body=Updated.",
        ],
    );

    let comments = gh(
        &workspace,
        &[
            "api",
            "repos/acme/shop/pulls/1/comments",
            "-q",
            "[.[] | {in_reply_to_id, user: .user.login, body}]",
        ],
    );
    assert_eq!(
        comments,
        json!([
            {"in_reply_to_id": null, "user": "reviewer", "body": "Stale example."},
            {"in_reply_to_id": 11, "user": "developer", "body": "Updated."}
        ])
    );
}

#[test]
fn answers_a_posted_reply_with_the_comment_it_created() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer",
        "review_comments": [
            {
                "id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"},
                "body": "Stale example.", "html_url": "https://github.com/acme/shop/pull/1#discussion_r11"
            }
        ]
    }));

    let answer = gh(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/comments/11/replies",
            "-f",
            "body=Updated.",
        ],
    );

    assert_eq!(
        answer,
        json!({
            "id": 12, "in_reply_to_id": 11, "path": "README.md", "line": 10, "user": {"login": "developer"},
            "body": "Updated.", "html_url": "https://github.com/acme/shop/pull/1#discussion_r12"
        })
    );
}

#[test]
fn posts_a_comment_to_the_general_conversation() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "viewer": "developer",
        "issue_comments": []
    }));

    gh_text(
        &workspace,
        &[
            "pr",
            "comment",
            "1",
            "--repo",
            "acme/shop",
            "--body",
            "Both points are addressed.",
        ],
    );

    let comments = gh(
        &workspace,
        &[
            "api",
            "repos/acme/shop/issues/1/comments",
            "-q",
            "[.[] | {user: .user.login, body}]",
        ],
    );
    assert_eq!(
        comments,
        json!([{"user": "developer", "body": "Both points are addressed."}])
    );
}

#[test]
fn replaces_the_description_from_a_file() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "body": "Rounds every total."
    }));
    let description = "Rounds every total to the cent.\n";
    fs::write(workspace.path().join("description.md"), description).expect("the new description");

    gh_text(
        &workspace,
        &[
            "pr",
            "edit",
            "1",
            "--repo",
            "acme/shop",
            "--body-file",
            "description.md",
        ],
    );

    let shown = gh(&workspace, &["pr", "view", "1", "--json", "body"]);
    assert_eq!(shown, json!({"body": description}));
}

#[test]
fn lists_no_review_comment_on_a_pull_request_without_any() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1
    }));

    let answer = gh(&workspace, &["api", "repos/acme/shop/pulls/1/comments"]);

    assert_eq!(answer, json!([]));
}

#[test]
fn refuses_a_review_request_for_the_author_of_the_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "author": "developer"
    }));

    let refusal = run(
        workspace.path(),
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/requested_reviewers",
            "-f",
            "reviewers[]=developer",
        ],
    );

    assert_eq!(
        (
            refusal.status.code(),
            String::from_utf8_lossy(&refusal.stderr).into_owned()
        ),
        (
            Some(1),
            "gh: Review cannot be requested from pull request author. (HTTP 422)\n".to_string()
        )
    );
}

#[test]
fn requests_a_new_review_from_the_reviewer() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "author": "developer"
    }));

    let requested = gh(
        &workspace,
        &[
            "api",
            "--method",
            "POST",
            "repos/acme/shop/pulls/1/requested_reviewers",
            "-f",
            "reviewers[]=reviewer",
            "-q",
            "[.requested_reviewers[].login]",
        ],
    );

    assert_eq!(requested, json!(["reviewer"]));
}

#[test]
fn answers_not_found_for_another_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1
    }));

    let refusal = run(
        workspace.path(),
        &["api", "repos/acme/shop/pulls/2/comments"],
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
fn rejects_an_unknown_call_and_records_it_as_unhandled() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1
    }));

    let refusal = run(workspace.path(), &["pr", "merge", "1", "--squash"]);

    assert_eq!(
        (
            refusal.status.code(),
            String::from_utf8_lossy(&refusal.stderr).into_owned()
        ),
        (
            Some(1),
            "gh: `gh pr merge 1 --squash` is not available here\n".to_string()
        )
    );
    assert_eq!(
        recorded_calls(&workspace),
        vec![json!({"args": ["pr", "merge", "1", "--squash"], "unhandled": true})]
    );
}

#[test]
fn reports_a_workspace_with_no_pull_request_to_impersonate() {
    let workspace = TempDir::new().expect("temporary workspace");

    let refusal = run(workspace.path(), &["pr", "view", "1", "--json", "body"]);

    assert_eq!(
        (
            refusal.status.code(),
            String::from_utf8_lossy(&refusal.stderr).into_owned()
        ),
        (
            Some(1),
            "gh: no pull request to answer about: .git/fake-gh/pull-request.json is missing\n"
                .to_string()
        )
    );
}

#[test]
fn finds_the_pull_request_from_a_subdirectory_of_the_workspace() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "body": "Rounds every total to the cent."
    }));
    let subdirectory = workspace.path().join("src");
    fs::create_dir(&subdirectory).expect("a subdirectory");

    let answer = run(&subdirectory, &["pr", "view", "1", "--json", "body"]);

    assert_eq!(
        String::from_utf8_lossy(&answer.stdout),
        "{\"body\":\"Rounds every total to the cent.\"}\n"
    );
}

#[test]
fn rejects_a_call_too_short_to_name_a_command() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1
    }));

    let refusal = run(workspace.path(), &["status"]);

    assert_eq!(
        (
            refusal.status.code(),
            String::from_utf8_lossy(&refusal.stderr).into_owned()
        ),
        (
            Some(1),
            "gh: `gh status` is not available here\n".to_string()
        )
    );
    assert_eq!(
        recorded_calls(&workspace),
        vec![json!({"args": ["status"], "unhandled": true})]
    );
}

#[test]
fn tells_its_version() {
    let workspace = workspace_impersonating(json!({"repository": "acme/shop", "number": 1}));

    let answer = gh_text(&workspace, &["--version"]);

    assert_eq!(
        answer,
        "gh version 2.105.0 (2026-11-17)\nhttps://github.com/cli/cli/releases/tag/v2.105.0\n"
    );
}

#[test]
fn names_the_repository_it_impersonates() {
    let workspace = workspace_impersonating(json!({"repository": "acme/shop", "number": 1}));

    let answer = gh(&workspace, &["repo", "view", "--json", "nameWithOwner"]);

    assert_eq!(answer, json!({"nameWithOwner": "acme/shop"}));
}

#[test]
fn shows_the_fields_a_pull_request_view_names() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "title": "Round every total",
        "state": "open",
        "head_ref": "rounding",
        "author": "developer"
    }));

    let answer = gh(
        &workspace,
        &[
            "pr",
            "view",
            "1",
            "--json",
            "number,title,state,headRefName,author,url",
        ],
    );

    assert_eq!(
        answer,
        json!({
            "number": 1, "title": "Round every total", "state": "OPEN", "headRefName": "rounding",
            "author": {"login": "developer"}, "url": "https://github.com/acme/shop/pull/1"
        })
    );
}

#[test]
fn shows_the_reviews_and_the_conversation_as_text() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {"id": 31, "user": {"login": "reviewer"}, "state": "COMMENTED", "body": "Two notes.", "author_association": "MEMBER"},
            {"id": 32, "user": {"login": "reviewer"}, "state": "COMMENTED", "body": ""}
        ],
        "issue_comments": [
            {"id": 21, "user": {"login": "developer"}, "body": "Both fixed.", "author_association": "OWNER"}
        ]
    }));

    let answer = gh_text(&workspace, &["pr", "view", "1", "--comments"]);

    assert_eq!(
        answer,
        "author:\treviewer\nassociation:\tmember\nedited:\tfalse\nstatus:\tcommented\n--\nTwo notes.\n--\n\
         author:\tdeveloper\nassociation:\towner\nedited:\tfalse\nstatus:\tnone\n--\nBoth fixed.\n--\n"
    );
}

#[test]
fn fills_in_the_owner_and_repository_a_path_leaves_to_gh() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [
            {"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."}
        ]
    }));

    let answer = gh(
        &workspace,
        &["api", "repos/{owner}/{repo}/pulls/1/comments"],
    );

    assert_eq!(
        answer,
        json!([{"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."}])
    );
}

#[test]
fn shows_the_base_branch_of_a_pull_request() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "base_ref": "main"
    }));

    let answer = gh(&workspace, &["pr", "view", "1", "--json", "baseRefName"]);

    assert_eq!(answer, json!({"baseRefName": "main"}));
}

#[test]
fn shows_the_pull_request_itself() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "title": "Round every total",
        "state": "open",
        "author": "developer",
        "body": "Rounds every total to the cent.",
        "head_ref": "rounding",
        "head_sha": "9fceb02",
        "base_ref": "main"
    }));

    let answer = gh(&workspace, &["api", "repos/acme/shop/pulls/1"]);

    assert_eq!(
        answer,
        json!({
            "number": 1, "title": "Round every total", "state": "open", "user": {"login": "developer"},
            "body": "Rounds every total to the cent.", "html_url": "https://github.com/acme/shop/pull/1",
            "head": {"ref": "rounding", "sha": "9fceb02"}, "base": {"ref": "main"}
        })
    );
}

#[test]
fn filters_an_answer_with_its_long_jq_option() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "review_comments": [
            {"id": 11, "path": "README.md", "line": 10, "user": {"login": "reviewer"}, "body": "Stale example."}
        ]
    }));

    let answer = gh_text(
        &workspace,
        &[
            "api",
            "repos/acme/shop/pulls/1/comments",
            "--jq",
            ".[].body",
        ],
    );

    assert_eq!(answer, "Stale example.\n");
}

#[test]
fn shows_the_reviews_and_the_conversation_a_pull_request_view_names() {
    let workspace = workspace_impersonating(json!({
        "repository": "acme/shop",
        "number": 1,
        "reviews": [
            {
                "id": 31, "node_id": "PRR_31", "user": {"login": "reviewer"}, "state": "COMMENTED",
                "body": "Two notes.", "author_association": "MEMBER"
            }
        ],
        "issue_comments": [
            {"id": 21, "user": {"login": "developer"}, "body": "Both fixed.", "author_association": "OWNER"}
        ]
    }));

    let answer = gh(
        &workspace,
        &["pr", "view", "1", "--json", "reviews,comments"],
    );

    assert_eq!(
        answer,
        json!({
            "reviews": [{
                "id": "PRR_31", "author": {"login": "reviewer"}, "authorAssociation": "MEMBER",
                "body": "Two notes.", "state": "COMMENTED"
            }],
            "comments": [{"author": {"login": "developer"}, "authorAssociation": "OWNER", "body": "Both fixed."}]
        })
    );
}
