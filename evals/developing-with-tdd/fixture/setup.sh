#!/bin/sh
# Creates a fresh fixture for one run: a small Rust crate, `shop`, in a git repo of its
# own, committed so that `git status` after a turn shows only what the run changed.
#
#   setup.sh <state> <dir>
#
# The state picks what the crate holds on top of `base/`, a cart whose two tests pass:
#
#   feature      `FEATURE.md` describes promo codes, which the crate does not have yet
#   implemented  promo codes implemented and tested, with one bug no test covers: a code
#                is rejected on its expiry date, which the feature says is still valid
#   halfway      a plan halfway through: the single-item test green, the cart total
#                faked as the first item's price, two tests still pending
#
# Rebuilt for every run rather than reset, since a local directory costs nothing and a
# reset would have to know everything a run might leave behind.
set -eu

[ "$#" -eq 2 ] || {
    echo "usage: setup.sh <feature|implemented|halfway> <dir>" >&2
    exit 2
}
state="$1"
dir="$2"
here="$(cd "$(dirname "$0")" && pwd)"
repo_root="$(cd "$here/../../.." && pwd)"

case "$state" in
    feature | implemented | halfway) ;;
    *)
        echo "setup: unknown state '$state'" >&2
        exit 2
        ;;
esac

mkdir -p "$dir"
cp -R "$here/base/." "$dir/"
mv "$dir/gitignore" "$dir/.gitignore"
case "$state" in
    implemented) cp -R "$here/implemented/." "$dir/" ;;
    halfway)
        cp -R "$here/halfway/." "$dir/"
        rm "$dir/FEATURE.md"
        ;;
esac

# asdf resolves the toolchain from the nearest `.tool-versions`, and a temporary directory
# has none above it.
cp "$repo_root/.tool-versions" "$dir/"

cd "$dir"

# Built before the commit, so the first `cargo test` of a run is not where the build time
# goes, and so the `Cargo.lock` it writes is committed rather than counted as a change.
cargo build -q --tests 2>/dev/null

git init -q
git config user.name "eval fixture"
git config user.email "eval-fixture@example.invalid"
git add -A
git commit -q -m "fixture: $state"
