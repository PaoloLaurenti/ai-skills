#!/bin/sh
# Every mechanical gate this repo has, in one command. A working copy, the
# pre-commit hook and CI all run this same script, so none of the three can come
# to disagree about what passing means.
set -eu

manifest="$(dirname "$0")/skill-check/Cargo.toml"

if ! command -v cargo >/dev/null 2>&1; then
    echo "check: cargo is not on PATH, so nothing was checked." >&2
    echo "The version comes from .tool-versions, which asdf resolves in a shell" >&2
    echo "started in this directory. A git client launched outside such a shell" >&2
    echo "may not see it." >&2
    exit 1
fi

echo "==> formatting"
cargo fmt --manifest-path "$manifest" --check

echo "==> lints"
cargo clippy --locked --manifest-path "$manifest" --all-targets -- -D warnings

echo "==> skills and repo-wide rules"
cargo test --locked --manifest-path "$manifest"
