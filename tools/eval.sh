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
# The stand-in for `gh` the pull request suites run against, first on PATH so no real
# `gh` answers in its place. It runs from the repo rather than from the copy below,
# since the sandbox skips an executable whose directory sits in a temporary one.
cargo build --quiet --locked --manifest-path "$repo/tools/fake-gh/Cargo.toml"
PATH="$repo/tools/fake-gh/target/debug:$PATH"
# Apple's /usr/bin/git is a shim that asks xcrun for the real git, and xcrun needs a
# cache the sandbox denies, so every git command exits 72. The real git runs, but the
# sandbox skips its directory, being outside the home: three scripts in a directory git
# ignores here hand each command to it. git push runs the other two by name.
if [ "$(command -v git)" = /usr/bin/git ] &&
    developer="$(xcode-select -p 2>/dev/null)" && [ -x "$developer/usr/bin/git" ]; then
    shims="$repo/evals/results/.git-bin"
    mkdir -p "$shims"
    for command in git git-receive-pack git-upload-pack; do
        printf '#!/bin/sh\nexec "%s" "$@"\n' "$developer/usr/bin/$command" >"$shims/$command"
        chmod +x "$shims/$command"
    done
    PATH="$shims:$PATH"
fi
export PATH

# The command refuses a plugin holding a file with two names, which cargo's build output
# is full of. So it evaluates a copy of what git sees, uncommitted edits included, and
# the results come back here.
copy="$(cd "$(mktemp -d)" && pwd -P)"
trap 'rm -rf "$copy"' EXIT
(
    cd "$repo"
    git ls-files -z --cached --others --exclude-standard |
        xargs -0 sh -c 'for f; do [ ! -e "$f" ] || printf "%s\0" "$f"; done' sh |
        tar --null -T - -cf - |
        tar -xf - -C "$copy"
)
# A later-turn case resumes a recorded conversation, where the loaded skill names its
# directory `<plugin-root>`, and that directory is now the copy.
find "$copy/evals" -name history.jsonl | while IFS= read -r history; do
    sed "s|<plugin-root>|$copy|g" "$history" >"$history.new"
    mv "$history.new" "$history"
done
results="$repo/evals/results/$(date -u +%Y-%m-%dT%H-%M-%SZ)"

# The target comes first: --allow-tools and --tag take several values, and would read
# a target placed after them as one more. No exec, so the trap removes the copy. Read
# is granted on the copy because a skill's reference files sit there, outside the
# workspace, where a run refuses it.
claude plugin eval "$copy" \
    --output-dir "$results" \
    --report "$results/report.html" \
    --no-publish \
    --scaffold \
    --trust-plugin \
    --model opus \
    --judge-model sonnet \
    --max-cost-usd 16 \
    --allow-tools Edit Write 'Bash(cargo:*)' 'Bash(git:*)' 'Bash(gh:*)' "Read(/$copy/**)" \
    --tag "$skill" \
    "$@"
