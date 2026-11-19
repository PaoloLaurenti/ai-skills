#!/bin/sh
# Checks the stand-in for `gh` against the real one: records GitHub's answers again,
# into a temporary directory, and replays them against the stand-in with
# `tests/recorded.rs`. A failure means GitHub or `gh` now answers otherwise than the
# stand-in does: run `record.sh`, and the same tests show what to change.
#
# The pre-commit hook runs it after `tools/check.sh`, at most once a day per clone: the
# last day it passed is kept in `.git/`, which git never versions. It skips with a
# warning, never blocking the commit, where it cannot reach the owner's fixture repo: no
# `gh` logged in, no network, or a clone of someone without access to it. It stays out
# of `tools/check.sh`, which CI runs with no credentials to write there.
set -eu

here="$(cd "$(dirname "$0")" && pwd -P)"
passed="$(git rev-parse --absolute-git-dir)/fake-gh-checked"
today="$(date +%Y-%m-%d)"

if [ "$(cat "$passed" 2>/dev/null || true)" = "$today" ]; then
    exit 0
fi

skip() {
    echo "fake-gh: not checked against GitHub today: $1" >&2
    exit 0
}
command -v gh >/dev/null 2>&1 || skip "gh is not installed"
login="$(gh api user -q .login 2>/dev/null)" || skip "gh cannot reach GitHub as a logged-in user"
repo="${EVAL_FIXTURE_REPO:-$login/ai-skills-eval-walkthrough}"
gh repo view "$repo" >/dev/null 2>&1 || skip "$repo, the fixture repo, is out of reach"

echo "==> fake-gh: checking against GitHub, once today"
fresh="$(mktemp -d)"
trap 'rm -rf "$fresh"' EXIT
"$here/record.sh" --into "$fresh" >/dev/null 2>&1 || {
    echo "fake-gh: record.sh failed; run it on its own to see why." >&2
    exit 1
}
FAKE_GH_RECORDING="$fresh" cargo test --quiet --locked \
    --manifest-path "$here/Cargo.toml" --test recorded || {
    echo "fake-gh: GitHub now answers otherwise than the stand-in. Run" >&2
    echo "tools/fake-gh/record.sh, then make tests/recorded.rs pass again." >&2
    exit 1
}
echo "$today" >"$passed"
