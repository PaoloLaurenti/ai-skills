#!/bin/sh
# Records what the real `gh` answers, so the stand-in is built from GitHub's own words
# rather than from memory. Two things land in the repo:
#
# - `evals/addressing-pull-request-reviews/fixture/pull-request.json`, the fixture pull
#   request as the stand-in reads it, for that suite's cases to start from;
# - `tools/fake-gh/recorded/<name>.json`, one per answer the stand-in had to guess, each
#   the calls in order with their exit status and output, which `tests/recorded.rs`
#   replays against the stand-in.
#
# It runs against that suite's fixture repo, built by its `setup.sh`, and puts it back
# with its `reset.sh` before and after: some of the calls below publish at once, a reply,
# a comment, a description, a review request. Run it again when `gh` or a skill's calls
# change, never edit what it writes.
#
# The owner's login is written as `fixture-owner`, since nothing in this repo names the
# owner; `tools/check.sh` fails on any that slips through.
set -eu

here="$(cd "$(dirname "$0")" && pwd -P)"
root="$(cd "$here/../.." && pwd -P)"
fixture="$root/evals/addressing-pull-request-reviews/fixture"
login="$(gh api user -q .login)"
EVAL_FIXTURE_REPO="${EVAL_FIXTURE_REPO:-$login/ai-skills-eval-walkthrough}"
export EVAL_FIXTURE_REPO
repo="$EVAL_FIXTURE_REPO"
owner="${repo%/*}"

work="$(mktemp -d)"
trap '"$fixture/reset.sh" >/dev/null; rm -rf "$work"' EXIT
"$fixture/reset.sh" >/dev/null
number=$(gh pr list --repo "$repo" --head walkthrough --state open --json number -q '.[0].number')
head=$(gh pr view "$number" --repo "$repo" --json headRefOid -q .headRefOid)
cd "$work"

# The account's number and its GraphQL id lead back to it as surely as its login, so
# they become those of account 1, `MDQ6VXNlcjE=` being `04:User1` in base64.
account="$(gh api user -q .id)"
account_node="$(gh api user -q .node_id)"
scrub() {
    sed -E \
        -e "s/$owner/fixture-owner/g" \
        -e "s/$login/fixture-owner/g" \
        -e "s|$account_node|MDQ6VXNlcjE=|g" \
        -e "s/(^|[^0-9])$account([^0-9]|$)/\\11\\2/g"
}

# The fixture pull request. Only the reviews holding a planted comment are kept: GitHub
# never deletes a submitted review, so every reset leaves one more empty review behind.
gh api "repos/$repo/pulls/$number" >pr.json
gh api "repos/$repo/pulls/$number/comments" >comments.json
gh api "repos/$repo/issues/$number/comments" >conversation.json
gh api "repos/$repo/pulls/$number/reviews" >reviews.json
gh pr diff "$number" --repo "$repo" >diff.txt
jq -n \
    --slurpfile pr pr.json \
    --slurpfile comments comments.json \
    --slurpfile conversation conversation.json \
    --slurpfile reviews reviews.json \
    --rawfile diff diff.txt \
    --arg viewer "$login" \
    '$pr[0] as $pr | $comments[0] as $comments | {
        repository: $pr.base.repo.full_name,
        number: $pr.number,
        author: $pr.user.login,
        viewer: $viewer,
        body: $pr.body,
        diff: $diff,
        head_sha: $pr.head.sha,
        review_comments: $comments,
        issue_comments: $conversation[0],
        reviews: [$reviews[0][] | select(.id as $id | $comments | any(.pull_request_review_id == $id))]
    }' | scrub >"$fixture/pull-request.json"

# A probe is a list of calls run in order. `call` runs one and appends it to the probe;
# a node id a call answers with is remembered, and written as `{{node_id}}` wherever a
# later call sends it back, since the stand-in gives its own.
mkdir -p "$here/recorded"
node_id=""
start() {
    probe="$1"
    : >calls.jsonl
    printf '{}' >files.json
}
file() {
    printf '%s' "$2" >"$1"
    jq --arg name "$1" --rawfile text "$1" '. + {($name): $text}' files.json >files.next
    mv files.next files.json
}
call() {
    status=0
    gh "$@" >stdout.txt 2>stderr.txt || status=$?
    # Through NUL-separated text, since jq would read an argument such as `--repo` as
    # one of its own options.
    printf '%s\0' "$@" >args.bin
    jq -n --rawfile args args.bin --argjson exit "$status" \
        --rawfile stdout stdout.txt --rawfile stderr stderr.txt \
        '{args: ($args | split("\u0000")[:-1]), exit: $exit, stdout: $stdout, stderr: $stderr}' >>calls.jsonl
    answered="$(jq -r '.node_id? // empty' stdout.txt 2>/dev/null || true)"
    [ -z "$answered" ] || node_id="$answered"
}
finish() {
    jq -s --slurpfile files files.json '{files: $files[0], calls: .}' calls.jsonl |
        if [ -n "$node_id" ]; then sed "s/$node_id/{{node_id}}/g"; else cat; fi |
        scrub >"$here/recorded/$probe.json"
    node_id=""
}
clear_pending() {
    gh api "repos/$repo/pulls/$number/reviews" -q '.[] | select(.state == "PENDING") | .id' |
        while read -r id; do
            gh api --method DELETE "repos/$repo/pulls/$number/reviews/$id" >/dev/null
        done
}

start description-from-a-file
file description.md "Prints every reading with one decimal and its unit symbol.
"
call pr edit "$number" --repo "$repo" --body-file description.md
call pr view "$number" --repo "$repo" --json body -q .body
finish

start review-request-for-the-author
call api --method POST "repos/$repo/pulls/$number/requested_reviewers" -f "reviewers[]=$login"
finish

start second-pending-review
file review.json "{\"commit_id\": \"$head\", \"body\": \"A recorded draft.\"}"
call api --method POST "repos/$repo/pulls/$number/reviews" --input review.json
call api --method POST "repos/$repo/pulls/$number/reviews" --input review.json
finish
clear_pending

start general-comment-for-a-draft-without-one
file review.json "{\"commit_id\": \"$head\", \"body\": \"\"}"
call api --method POST "repos/$repo/pulls/$number/reviews" --input review.json
# shellcheck disable=SC2016 # GraphQL's variables, not the shell's.
call api graphql \
    -f 'query=mutation($rid:ID!,$body:String!){ updatePullRequestReview(input:{pullRequestReviewId:$rid, body:$body}){ pullRequestReview { state } } }' \
    -f "rid=$node_id" -f "body=One more point."
finish
clear_pending

start comment-on-the-conversation
call pr comment "$number" --repo "$repo" --body "A recorded comment."
finish

echo "recorded #$number of $repo into $fixture/pull-request.json and $here/recorded/"
