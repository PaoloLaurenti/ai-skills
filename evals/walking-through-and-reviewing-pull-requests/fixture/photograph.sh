#!/bin/sh
# Puts the pull request on `guided-1` back as every case starts from it, then photographs
# it into `pull-request.json` beside this script, which `workspace.sh` hands to the
# stand-in for `gh`. One pull request is enough: each case runs in a workspace of its own.
#
# The reset posts the planted comment again under a new id, and the recorded histories
# name the id in the photograph, so running this again means recording again every
# history of the suite.
set -eu

here="$(cd "$(dirname "$0")" && pwd -P)"
repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-guided-review}"
branch=guided-1

EVAL_FIXTURE_REPO="$repo" "$here/reset.sh" "$branch" >/dev/null
number=$(gh pr list --repo "$repo" --head "$branch" --state open --json number -q '.[0].number')
"$here/../../../tools/fake-gh/photograph.sh" "$repo" "$number" "$here/pull-request.json"
echo "photographed #$number of $repo into $here/pull-request.json"
