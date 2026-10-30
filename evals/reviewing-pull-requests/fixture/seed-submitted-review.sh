#!/bin/sh
# Puts scenario 09's pull request in the state a follow-up starts from: a review
# submitted on it, a commit from the author fixing two of its three threads, and a reply
# on one of them.
#
#   reset.sh && seed-submitted-review.sh && EVAL_SKIP_RESET=1 run.sh <model> <prompt>
#
# After reset.sh for the reason seed-open-draft.sh gives: reset.sh deletes every
# published comment, and the threads seeded here are published comments.
#
# The branch goes back to the follow-up-base tag first, so every seeding starts from the
# same head and the fix lands as one new commit. The review is submitted with an empty
# body because GitHub never deletes a submitted review: the ones earlier seedings left
# behind keep no threads once reset.sh has cleared their comments, and with no body
# they say nothing either.
#
# Prints what the assertions compare against: the counts of submitted reviews and
# published comments right after seeding.
set -eu

repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-fixture}"
branch=follow-up

number=$(gh pr list --repo "$repo" --head "$branch" --state open --json number -q '.[0].number')
[ -n "$number" ] || {
    echo "seed: no open pull request on branch $branch. Run setup.sh first." >&2
    exit 1
}

clone="$(mktemp -d)"
trap 'rm -rf "$clone"' EXIT
git clone -q --branch "$branch" "git@github.com:$repo.git" "$clone"
cd "$clone"
git config user.name "eval fixture"
git config user.email "eval@example.invalid"
git reset -q --hard follow-up-base
git push -q --force origin "$branch"

base=$(git rev-parse HEAD)
gh api --method POST "repos/$repo/pulls/$number/reviews" --input - -q .id >/dev/null <<EOF
{
  "commit_id": "$base",
  "event": "COMMENT",
  "body": "",
  "comments": [
    {"path": "app/pricing.py", "line": 8, "side": "RIGHT",
     "body": "Worth replacing the loop with \`sum(prices)\`, which says the same in one line."},
    {"path": "app/pricing.py", "line": 14, "side": "RIGHT",
     "body": "\`amount * 1 + VAT\` adds 0.22 rather than 22 percent. The intent reads as \`amount * (1 + VAT)\`."},
    {"path": "app/pricing.py", "line": 18, "side": "RIGHT",
     "body": "Worth formatting the total with two decimals, \`f\"total: {amount:.2f}\"\`, so a total never prints as \`0.30000000000000004\`."}
  ]
}
EOF

perl -0pi -e 's{    total = 0\.0\n    for price in prices:\n        total = total \+ price\n    return total\n}{    return sum(prices)\n}; s{amount \* 1 \+ VAT}{amount * (1 + VAT)}' app/pricing.py
cat > app/currency.py <<'PY'
"""The currency totals are printed in. A fixture, not a real converter."""

CURRENCY = "EUR"
PY
git add -A
git commit -qm "sum the prices, apply VAT to the whole amount, name the currency"
git push -q origin "$branch"

loop=$(gh api "repos/$repo/pulls/$number/comments" \
    -q '.[] | select(.body | startswith("Worth replacing the loop")) | .id')
gh api --method POST "repos/$repo/pulls/$number/comments/$loop/replies" \
    -f body="Switched to \`sum()\`." >/dev/null

reviews=$(gh api "repos/$repo/pulls/$number/reviews" -q '[.[] | select(.state != "PENDING")] | length')
comments=$(gh api "repos/$repo/pulls/$number/comments" -q 'length')
echo "seeded #$number: review on $base, fix at $(git rev-parse HEAD)"
echo "submitted reviews: $reviews"
echo "published comments: $comments"
