#!/bin/sh
# Clears the pending review every run leaves behind, on every fixture pull
# request.
#
# GitHub allows one pending review per user per pull request, so a second POST
# fails while a draft exists. That failure reads exactly like the skill getting the
# delivery wrong, which is why this runs before every run rather than after a
# failure.
set -eu

repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-fixture}"

if ! gh repo view "$repo" >/dev/null 2>&1; then
    echo "reset: $repo does not exist. Run setup.sh first." >&2
    exit 1
fi

cleared=0
for number in $(gh pr list --repo "$repo" --state open --json number -q '.[].number'); do
    for id in $(gh api "repos/$repo/pulls/$number/reviews" \
        -q '.[] | select(.state == "PENDING") | .id'); do
        gh api --method DELETE "repos/$repo/pulls/$number/reviews/$id" >/dev/null
        echo "cleared pending review $id on #$number"
        cleared=$((cleared + 1))
    done
    # A run that submitted its review, which a baseline without the skill is free to
    # do, leaves published comments behind. Those can be deleted one by one, so the
    # assertion that nothing leaked stays meaningful on the next run. The submitted
    # review itself cannot be deleted, only dismissed, and it does not matter: every
    # assertion here counts PENDING reviews.
    for comment in $(gh api "repos/$repo/pulls/$number/comments" -q '.[].id'); do
        gh api --method DELETE "repos/$repo/pulls/comments/$comment" >/dev/null
        echo "cleared published comment $comment on #$number"
        cleared=$((cleared + 1))
    done
done

if [ "$cleared" -eq 0 ]; then
    echo "no pending reviews to clear"
fi
