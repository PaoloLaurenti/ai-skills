#!/bin/sh
# The one way to run a skill's eval cases through `claude plugin eval`.
#
#   tools/eval.sh <skill> [options]
#
# Options after the skill reach the command as they are, and a single-valued one given
# there overrides the default set here, such as `--model haiku --runs 1 --case <glob>`
# while a case is being written.
set -eu

if [ $# -lt 1 ]; then
    echo "usage: tools/eval.sh <skill> [options]" >&2
    exit 64
fi
skill="$1"
shift

repo="$(cd "$(dirname "$0")/.." && pwd -P)"

# Every run gets a temporary home, where the asdf shims cannot find their installs and
# cargo exits 126. Resolved at launch from the repo, where .tool-versions applies, so
# no machine-local path is written down.
if ! cargo="$(cd "$repo" && asdf which cargo 2>/dev/null)"; then
    echo "eval: asdf could not resolve cargo for this repo, so nothing was run." >&2
    echo "The Rust version comes from .tool-versions, which asdf has to have installed." >&2
    exit 1
fi
toolchain="$(dirname "$(dirname "$cargo")")"
# The sandbox Bash runs in cannot read the home, except the directories on PATH. rustc
# loads its own library from lib/, so lib/ goes on PATH too, or nothing compiles.
PATH="$toolchain/bin:$toolchain/lib:$PATH"
export PATH

# The target comes first: --allow-tools and --tag take several values, and would read
# a target placed after them as one more.
exec claude plugin eval "$repo" \
    --no-publish \
    --scaffold \
    --trust-plugin \
    --model opus \
    --judge-model sonnet \
    --max-cost-usd 10 \
    --allow-tools Edit Write 'Bash(cargo:*)' 'Bash(git:*)' \
    --tag "$skill" \
    "$@"
