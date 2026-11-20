#!/bin/sh
# Puts every pull request of the fixture back in the state the cases start from, before
# `photograph.sh` takes them.
#
# - A pending review, which is what a run against the live pull request leaves behind:
#   deleted.
# - The published comments, which a run that submitted its review leaves behind: deleted
#   one by one.
# - A submitted review with a body, which can be neither deleted nor emptied, and which a
#   case would read as a review the requester already wrote: the pull request is closed
#   and replaced by a fresh one on the same branch, with the same title and description,
#   so its number changes.
#
# The follow-up's seeded review has no body, so the follow-up pull request is cleared like
# the others and seeded again by `seed-submitted-review.sh`.
set -eu

repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-fixture}"

if ! gh repo view "$repo" >/dev/null 2>&1; then
    echo "reset: $repo does not exist. Run setup.sh first." >&2
    exit 1
fi

for number in $(gh pr list --repo "$repo" --state open --json number -q '.[].number'); do
    written=$(gh api --paginate "repos/$repo/pulls/$number/reviews" \
        -q '.[] | select(.state != "PENDING" and (.body | length) > 0) | .id' | wc -l | tr -d ' ')
    if [ "$written" -gt 0 ]; then
        branch=$(gh pr view "$number" --repo "$repo" --json headRefName -q .headRefName)
        title=$(gh pr view "$number" --repo "$repo" --json title -q .title)
        body="$(mktemp)"
        gh pr view "$number" --repo "$repo" --json body -q .body >"$body"
        gh pr close "$number" --repo "$repo" >/dev/null
        url=$(gh pr create --repo "$repo" --base main --head "$branch" \
            --title "$title" --body-file "$body")
        rm -f "$body"
        echo "replaced #$number, which carried a written review, by #${url##*/}"
        continue
    fi
    for id in $(gh api "repos/$repo/pulls/$number/reviews" \
        -q '.[] | select(.state == "PENDING") | .id'); do
        gh api --method DELETE "repos/$repo/pulls/$number/reviews/$id" >/dev/null
        echo "cleared pending review $id on #$number"
    done
    for comment in $(gh api --paginate "repos/$repo/pulls/$number/comments" -q '.[].id'); do
        gh api --method DELETE "repos/$repo/pulls/comments/$comment" >/dev/null
        echo "cleared published comment $comment on #$number"
    done
done
