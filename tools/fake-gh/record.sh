#!/bin/sh
# Records what the real `gh` answers, so the stand-in is built from GitHub's own words
# rather than from memory. Two things land in the repo:
#
# - `evals/addressing-pull-request-reviews/fixture/pull-request.json`, the fixture pull
#   request as the stand-in reads it, taken by `photograph.sh`, for that suite's cases to
#   start from;
# - `tools/fake-gh/recorded/<name>.json`, one per answer the stand-in must give as GitHub
#   does, each the calls in order with their exit status and output, which
#   `tests/recorded.rs` replays against the stand-in.
#
# It runs against that suite's fixture repo, built by its `setup.sh`, and puts it back
# with its `reset.sh` before and after: some of the calls below publish at once, a reply,
# a comment, a description, a review request. Run it again when `gh` or a skill's calls
# change, never edit what it writes.
#
#   record.sh             writes both into the repo
#   record.sh --into DIR  writes them into DIR instead, as `pull-request.json` and
#                         `recorded/`, for `check-against-github.sh` to compare
#
# The owner's login is written as `fixture-owner`, since nothing in this repo names the
# owner; `tools/check.sh` fails on any that slips through.
set -eu

here="$(cd "$(dirname "$0")" && pwd -P)"
root="$(cd "$here/../.." && pwd -P)"
fixture="$root/evals/addressing-pull-request-reviews/fixture"
snapshot="$fixture/pull-request.json"
recordings="$here/recorded"
if [ "${1:-}" = --into ]; then
    mkdir -p "$2"
    into="$(cd "$2" && pwd -P)"
    snapshot="$into/pull-request.json"
    recordings="$into/recorded"
fi
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

"$here/photograph.sh" "$repo" "$number" "$snapshot"

# A probe is a list of calls run in order. `call` runs one and appends it to the probe;
# a node id a call answers with is remembered, and written as `{{node_id}}` wherever a
# later call sends it back, since the stand-in gives its own.
mkdir -p "$recordings" unscrubbed
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
        if [ -n "$node_id" ]; then sed "s/$node_id/{{node_id}}/g"; else cat; fi \
            >"unscrubbed/$probe.json"
    node_id=""
}
clear_pending() {
    gh api "repos/$repo/pulls/$number/reviews" -q '.[] | select(.state == "PENDING") | .id' |
        while read -r id; do
            gh api --method DELETE "repos/$repo/pulls/$number/reviews/$id" >/dev/null
        done
}

# What a run asks first, finding its bearings.
start version
call --version
finish

start repository-name
call repo view "$repo" --json nameWithOwner
finish

start pull-request-view
call pr view "$number" --repo "$repo" --json number,title,state,headRefName,baseRefName,author,url,reviews,comments
finish

start pull-request-itself
call api "repos/$repo/pulls/$number" --jq '{number, title, state, head: .head.ref, base: .base.ref}'
finish

start comments-as-text
call pr view "$number" --repo "$repo" --comments
finish

# `gh` fills `{owner}` and `{repo}` in from the repo it runs in, here named by GH_REPO.
start placeholders-in-a-path
GH_REPO="$repo"
export GH_REPO
call api "repos/{owner}/{repo}/pulls/$number/comments" -q length
unset GH_REPO
finish

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

start review-from-fields
call api --method POST "repos/$repo/pulls/$number/reviews" -f "commit_id=$head" -f "body=A recorded draft."
call api "repos/$repo/pulls/$number/reviews" -q '.[] | select(.state == "PENDING") | .body'
finish
clear_pending

# The request file names the draft the call before it created, so it is written after.
start thread-from-a-request-file
file review.json "{\"commit_id\": \"$head\", \"body\": \"A recorded draft.\"}"
call api --method POST "repos/$repo/pulls/$number/reviews" --input review.json
# Built outside double quotes: inside "$(...)", /bin/sh on macOS split the query's
# input object into one request per field.
# shellcheck disable=SC2016 # GraphQL's variables, not the shell's.
request=$(jq -n --arg rid "$node_id" \
    --arg query 'mutation($rid:ID!,$body:String!,$path:String!,$line:Int!){ addPullRequestReviewThread(input:{pullRequestReviewId:$rid, body:$body, path:$path, line:$line, side:RIGHT}){ thread { id } } }' \
    '{query: $query, variables: {rid: $rid, body: "A recorded comment.", path: "README.md", line: 10}}')
file thread.json "$request"
call api graphql --input thread.json
finish
clear_pending

start general-comment-from-a-file
file review.json "{\"commit_id\": \"$head\", \"body\": \"A recorded draft.\"}"
file body.md "A recorded draft.
One more point.
"
call api --method POST "repos/$repo/pulls/$number/reviews" --input review.json
# shellcheck disable=SC2016 # GraphQL's variables, not the shell's.
call api graphql \
    -f 'query=mutation($rid:ID!,$body:String!){ updatePullRequestReview(input:{pullRequestReviewId:$rid, body:$body}){ pullRequestReview { state } } }' \
    -f "rid=$node_id" -F "body=@body.md"
call api "repos/$repo/pulls/$number/reviews" -q '.[] | select(.state == "PENDING") | .body'
finish
clear_pending

start comment-on-the-conversation
call pr comment "$number" --repo "$repo" --body "A recorded comment."
finish

# Scrubbed in one pass, before anything lands in the repo, so a failure halfway leaves
# no login behind.
"$here/scrub.sh" "$owner" unscrubbed/*.json
cp unscrubbed/*.json "$recordings/"

echo "recorded #$number of $repo into $snapshot and $recordings/"
