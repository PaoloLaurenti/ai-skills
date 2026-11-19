#!/bin/sh
# Creates the sandbox the walkthrough scenarios run against: one repo, one pull request,
# and the two review comments that `reset.sh` puts back before every run.
#
# The repo is separate from the one `reviewing-pull-requests` uses on purpose. That suite's
# reset deletes every published comment on every open pull request, and published
# comments are exactly what these scenarios need to find.
#
# The files and their two commits come from `files.sh`. The owner comes from the
# authenticated login at run time, so no name is written down.
#
# Run once. To rebuild, delete the repo and run it again.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
login="$(gh api user -q .login)"
repo="${EVAL_FIXTURE_REPO:-$login/ai-skills-eval-walkthrough}"
branch=walkthrough

if gh repo view "$repo" >/dev/null 2>&1; then
    echo "setup: $repo already exists." >&2
    echo "Delete it to rebuild, or set EVAL_FIXTURE_REPO to another name." >&2
    exit 1
fi

work="$(mktemp -d)"
trap 'rm -rf "$work"' EXIT
cd "$work"
git init -q -b main

# The fixture's commits carry an identity of their own. A machine's git config may scope
# its identity to a directory this temporary clone does not sit under.
git config user.name "$login"
git config user.email "$login@users.noreply.github.com"

"$here/files.sh" "$branch"

# `reset.sh` moves the branch back here before every run, so whatever a run committed and
# pushed is undone.
git tag walkthrough-base

# From `main`, because the first branch pushed becomes the default one.
git checkout -q main
gh repo create "$repo" --private --source . --remote origin --push >/dev/null
git push -q origin "$branch" walkthrough-base

gh pr create --repo "$repo" --base main --head "$branch" \
    --title "print every reading with one decimal and its unit symbol" \
    --body-file "$here/pr-body.md" >/dev/null

EVAL_FIXTURE_REPO="$repo" "$here/reset.sh"

echo
echo "Fixture ready at $repo:"
gh pr list --repo "$repo" --state open --json number,headRefName \
    -q '.[] | "  \(.headRefName)\t#\(.number)"'
