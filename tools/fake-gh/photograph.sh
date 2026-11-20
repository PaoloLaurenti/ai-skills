#!/bin/sh
# Photographs a pull request as it stands on GitHub into the file the stand-in for `gh`
# answers from, a case's `.git/fake-gh/pull-request.json`, with the account that took it
# scrubbed out.
#
#   photograph.sh <owner/repo> <number> <out.json>
#
# The comments keep their ids, and a recorded conversation that names one goes on
# naming it, so photographing a pull request again means recording again every history
# resumed over it.
set -eu

here="$(cd "$(dirname "$0")" && pwd -P)"
repo="$1"
number="$2"
out="$(cd "$(dirname "$3")" && pwd -P)/$(basename "$3")"
login="$(gh api user -q .login)"

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cd "$work"

# Reviews with no body and no planted comment are left out: GitHub never deletes a
# submitted review, so every reset of a fixture leaves one more behind.
gh api "repos/$repo/pulls/$number" >pr.json
gh api "repos/$repo/pulls/$number/comments" >comments.json
gh api "repos/$repo/issues/$number/comments" >conversation.json
gh api "repos/$repo/pulls/$number/reviews" >reviews.json
gh pr diff "$number" --repo "$repo" >diff.txt
# What `gh pr view` answers with every field it knows, and as text, for the stand-in to
# give back the fields it does not keep itself. The field list is the one this `gh`
# prints when asked for none.
fields="$(gh pr view "$number" --repo "$repo" --json 2>&1 | sed -n 's/^  *//p' | paste -sd, -)"
gh pr view "$number" --repo "$repo" --json "$fields" >viewed.json
gh pr view "$number" --repo "$repo" >viewed.txt
jq -n \
    --slurpfile pr pr.json \
    --slurpfile comments comments.json \
    --slurpfile conversation conversation.json \
    --slurpfile reviews reviews.json \
    --rawfile diff diff.txt \
    --slurpfile viewed viewed.json \
    --rawfile viewed_as_text viewed.txt \
    --arg viewer "$login" \
    '$pr[0] as $pr | $comments[0] as $comments | {
        repository: $pr.base.repo.full_name,
        number: $pr.number,
        title: $pr.title,
        state: $pr.state,
        head_ref: $pr.head.ref,
        base_ref: $pr.base.ref,
        author: $pr.user.login,
        viewer: $viewer,
        body: $pr.body,
        diff: $diff,
        head_sha: $pr.head.sha,
        review_comments: $comments,
        issue_comments: $conversation[0],
        reviews: [$reviews[0][] | select(.body != "" or (.id as $id | $comments | any(.pull_request_review_id == $id)))],
        viewed: $viewed[0],
        viewed_as_text: $viewed_as_text
    }' >pull-request.json
# Scrubbed before it lands, so a failure leaves no login behind in the repo.
"$here/scrub.sh" "${repo%/*}" pull-request.json
mv pull-request.json "$out"
