#!/bin/sh
# Creates the repo the suite's pull requests live in: one branch and open pull request
# per branch `files.sh` knows, which writes their files and commits. The owner comes
# from the authenticated login at run time, so no name is written down.
#
# Run once. To rebuild, delete the repo and run it again.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
login="$(gh api user -q .login)"
repo="${EVAL_FIXTURE_REPO:-$login/ai-skills-eval-fixture}"

if gh repo view "$repo" >/dev/null 2>&1; then
    echo "setup: $repo already exists." >&2
    echo "Delete it to rebuild, or set EVAL_FIXTURE_REPO to another name." >&2
    exit 1
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cd "$work"
git init -q -b main

# The fixture's commits carry an identity of their own. A machine's git config may
# scope its identity to a directory this temporary clone does not sit under, and
# these commits belong to the account that will own the pull requests anyway.
git config user.name "$login"
git config user.email "$login@users.noreply.github.com"

# Every branch is cut from `main`, which `files.sh` writes on its first call, so the repo
# is created after that call, from `main`. A pull request's title is its branch commit's
# message, and its body never hints at the finding: a case where the diff explains
# itself tests nothing.
open_pr() {
    branch="$1"
    body="$2"
    git rev-parse -q --verify "$branch" >/dev/null || "$here/files.sh" "$branch"
    git checkout -q main
    git push -q -u origin "$branch"
    gh pr create --repo "$repo" --head "$branch" --base main \
        --title "$(git log -1 --format=%s "$branch")" --body "$body"
}

"$here/files.sh" pending-state
git checkout -q main
gh repo create "$repo" --private --source=. --push \
    --description "Fixture pull requests for the reviewing-pull-requests evaluations"

open_pr pending-state "Keeps the base image on the current 3.x release."
open_pr inline-versus-body "Adds a startup line to the entrypoint so the logs show it came up."
open_pr permalinks "Adds a timeout setting so it stops being hardcoded."
open_pr suggestions "Sets ownership and permissions on the entrypoint explicitly."
open_pr docs-cross-check "Moves the listening port off 8080, which collides with another service."
# The body carries the bait of the architectural claim: a deployed name the workflow
# contradicts.
open_pr architectural-claims "Adds a metrics listener so the example-service container can be scraped."
open_pr open-draft "Lets the orchestrator restart the container when it stops answering."
open_pr values-in-docs "Adds a timeout and a batch size to the migration step."
# `seed-submitted-review.sh` puts the branch back on this tag before every seeding.
open_pr follow-up "Adds the subtotal, the VAT and a printable total."
git tag follow-up-base follow-up
git push -q origin follow-up-base

echo
echo "Fixture ready at $repo. Pull request numbers, by scenario branch:"
gh pr list --repo "$repo" --state open --json number,headRefName \
    -q '.[] | "  \(.headRefName)\t#\(.number)"'
