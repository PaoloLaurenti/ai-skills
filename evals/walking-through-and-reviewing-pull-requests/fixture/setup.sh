#!/bin/sh
# Creates the repo the suite's pull request lives in, with three pull requests carrying
# the same diff, one per hand-run scenario of old. The cases use `guided-1` alone, as
# `photograph.sh` takes it.
#
# The repo is separate from the ones the other suites use. `reviewing-pull-requests`
# resets by deleting every published comment on every open pull request, and the planted
# comment is what case `04` needs to find.
#
# The files and their two commits come from `files.sh`. The owner comes from the
# authenticated login at run time, so no name is written down.
#
# Run once. To rebuild, delete the repo and run it again.
set -eu

here="$(cd "$(dirname "$0")" && pwd)"
login="$(gh api user -q .login)"
repo="${EVAL_FIXTURE_REPO:-$login/ai-skills-eval-guided-review}"
branches="guided-1 guided-2 guided-3"

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

"$here/files.sh" guided-head

# `reset.sh` moves each pull request's branch back here.
git tag guided-base

# From `main`, because the first branch pushed becomes the default one.
git checkout -q main
gh repo create "$repo" --private --source . --remote origin --push >/dev/null
git push -q origin guided-base
for branch in $branches; do
    git push -q origin "guided-base:refs/heads/$branch"
    gh pr create --repo "$repo" --base main --head "$branch" \
        --title "mark items at or below five units as low in the report" \
        --body-file "$here/pr-body.md" >/dev/null
    EVAL_FIXTURE_REPO="$repo" "$here/reset.sh" "$branch"
done

echo
echo "Fixture ready at $repo:"
gh pr list --repo "$repo" --state open --json number,headRefName \
    -q '.[] | "  \(.headRefName)\t#\(.number)"'
