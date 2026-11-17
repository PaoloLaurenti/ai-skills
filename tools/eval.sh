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

# The command refuses a plugin holding a file with two names, which cargo's build output
# is full of. So it evaluates a copy of what git sees, uncommitted edits included, and
# the results come back here.
copy="$(mktemp -d)"
trap 'rm -rf "$copy"' EXIT
(
    cd "$repo"
    git ls-files -z --cached --others --exclude-standard |
        xargs -0 sh -c 'for f; do [ ! -e "$f" ] || printf "%s\0" "$f"; done' sh |
        tar --null -T - -cf - |
        tar -xf - -C "$copy"
)
results="$repo/evals/results/$(date -u +%Y-%m-%dT%H-%M-%SZ)"

# The target comes first: --allow-tools and --tag take several values, and would read
# a target placed after them as one more. No exec, so the trap removes the copy.
claude plugin eval "$copy" \
    --output-dir "$results" \
    --report "$results/report.html" \
    --no-publish \
    --scaffold \
    --trust-plugin \
    --model opus \
    --judge-model sonnet \
    --max-cost-usd 10 \
    --allow-tools Edit Write 'Bash(cargo:*)' 'Bash(git:*)' \
    --tag "$skill" \
    "$@"
