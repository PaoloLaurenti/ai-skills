#!/bin/sh
# Puts every pull request of the fixture back as the cases start from it, seeds the
# follow-up's submitted review, then photographs each one into `pull-requests/`, named
# after its branch, which `workspace.sh` hands to the stand-in for `gh`.
#
# The seed posts the follow-up's review and reply under new ids, and the recorded
# histories of the follow-up's later turns name them, so running this again means
# recording those histories again.
set -eu

here="$(cd "$(dirname "$0")" && pwd -P)"
repo="${EVAL_FIXTURE_REPO:-$(gh api user -q .login)/ai-skills-eval-fixture}"
export EVAL_FIXTURE_REPO="$repo"

"$here/reset.sh"
"$here/seed-submitted-review.sh"
mkdir -p "$here/pull-requests"
gh pr list --repo "$repo" --state open --json number,headRefName \
    -q '.[] | "\(.number) \(.headRefName)"' |
    while read -r number branch; do
        "$here/../../../tools/fake-gh/photograph.sh" "$repo" "$number" \
            "$here/pull-requests/$branch.json"
        echo "photographed #$number ($branch) into pull-requests/$branch.json"
    done
