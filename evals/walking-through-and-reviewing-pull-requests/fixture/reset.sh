#!/bin/sh
# Puts one pull request of the fixture back in the state the cases start from, before
# `photograph.sh` takes it, and once for each when `setup.sh` opens them.
#
#   reset.sh <branch>
#
# One branch at a time, as the hand-run scenarios once ran side by side, one per pull
# request. A conversation against the live pull request may change five things, and each
# is undone here:
#
# - the branch, when a run commits and pushes: it goes back to the `guided-base` tag,
#   forced;
# - a submitted review, when a run publishes one with a body: the pull request is replaced,
#   as explained below;
# - the description, when a run edits it: `pr-body.md` goes back in;
# - a pending review, which is what a run is expected to leave: deleted;
# - the published comments, the planted one included: every one is deleted, and
#   `review.json` posts the planted one again.
#
# The planted comment comes back with a new id on every reset, which is why photographing
# again means recording again every history of the suite. It is posted on its own rather than
# inside a review: GitHub never deletes a submitted review, and one posted on its own
# leaves behind a review with no body, which gives a run nothing to read into.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-guided-review}"
branch="${1:?usage: reset.sh <branch>}"

if ! gh repo view "$repo" >/dev/null 2>&1; then
    echo "reset: $repo does not exist. Run setup.sh first." >&2
    exit 1
fi

number=$(gh pr list --repo "$repo" --head "$branch" --state open --json number -q '.[0].number')
[ -n "$number" ] || {
    echo "reset: no open pull request on branch $branch." >&2
    exit 1
}

base=$(gh api "repos/$repo/git/ref/tags/guided-base" -q .object.sha)
gh api --method PATCH "repos/$repo/git/refs/heads/$branch" -f sha="$base" -F force=true >/dev/null

# A submitted review with a body can be neither deleted nor emptied, and the next run would
# read it as a review someone already wrote. So a pull request carrying one is closed and
# replaced by a fresh one on the same branch.
written=$(gh api --paginate "repos/$repo/pulls/$number/reviews" \
    -q '.[] | select(.state != "PENDING" and (.body | length) > 0) | .id' | wc -l | tr -d ' ')
if [ "$written" -gt 0 ]; then
    gh pr close "$number" --repo "$repo" >/dev/null 2>&1
    url=$(gh pr create --repo "$repo" --base main --head "$branch" \
        --title "mark items at or below five units as low in the report" \
        --body-file "$here/pr-body.md")
    echo "reset: #$number carried a written review, replaced by ${url##*/}"
    number="${url##*/}"
fi
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

echo "reset #$number ($branch) to $base, with the planted comment:"
gh api "repos/$repo/pulls/$number/comments" -q '.[] | "  \(.id)\t\(.path):\(.line)"'
