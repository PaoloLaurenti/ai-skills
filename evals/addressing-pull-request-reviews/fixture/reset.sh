#!/bin/sh
# Puts the fixture pull request back in the state every scenario starts from, and runs
# before every run, the first one included.
#
# A run is allowed to change four things, and each is undone here:
#
# - the branch, when a run commits and pushes: it goes back to the `walkthrough-base`
#   tag, forced;
# - the description, when a run corrects it: `pr-body.md` goes back in;
# - the comments, when a run replies: every comment on the pull request is deleted,
#   the planted ones included, and `review.json` posts them again;
# - a pending review, when a run leaves one.
#
# The planted comments come back with new ids on every reset, which is why a scenario
# reads them from the API instead of writing them down. Each is posted on its own rather
# than inside one review: GitHub never deletes a submitted review, so a review body posted
# on every reset would pile up as identical text for a run to read. A comment posted on
# its own leaves behind a review with no body, which gives a run nothing to walk through.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-walkthrough}"
branch=walkthrough

if ! gh repo view "$repo" >/dev/null 2>&1; then
    echo "reset: $repo does not exist. Run setup.sh first." >&2
    exit 1
fi

number=$(gh pr list --repo "$repo" --head "$branch" --state open --json number -q '.[0].number')
[ -n "$number" ] || {
    echo "reset: no open pull request on branch $branch." >&2
    exit 1
}

base=$(gh api "repos/$repo/git/ref/tags/walkthrough-base" -q .object.sha)
gh api --method PATCH "repos/$repo/git/refs/heads/$branch" -f sha="$base" -F force=true >/dev/null
gh api --method PATCH "repos/$repo/pulls/$number" -F "body=@$here/pr-body.md" >/dev/null

for id in $(gh api "repos/$repo/pulls/$number/reviews" -q '.[] | select(.state == "PENDING") | .id'); do
    gh api --method DELETE "repos/$repo/pulls/$number/reviews/$id" >/dev/null
done
for id in $(gh api --paginate "repos/$repo/pulls/$number/comments" -q '.[].id'); do
    gh api --method DELETE "repos/$repo/pulls/comments/$id" >/dev/null
done
for id in $(gh api --paginate "repos/$repo/issues/$number/comments" -q '.[].id'); do
    gh api --method DELETE "repos/$repo/issues/comments/$id" >/dev/null
done

payload="$(mktemp)"
trap 'rm -f "$payload"' EXIT
count=$(jq length "$here/review.json")
i=0
while [ "$i" -lt "$count" ]; do
    jq --arg sha "$base" --argjson i "$i" '.[$i] + {commit_id: $sha}' "$here/review.json" > "$payload"
    gh api --method POST "repos/$repo/pulls/$number/comments" --input "$payload" >/dev/null
    i=$((i + 1))
done

echo "reset #$number to $base, with the planted comments:"
gh api "repos/$repo/pulls/$number/comments" -q '.[] | "  \(.id)\t\(.path):\(.line)"'
