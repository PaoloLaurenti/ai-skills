#!/bin/sh
# Creates the sandbox the walkthrough scenarios run against: one repo, one pull request,
# and the two review comments that `reset.sh` puts back before every run.
#
# The repo is separate from the one `reviewing-pull-requests` uses on purpose. That suite's
# reset deletes every published comment on every open pull request, and published
# comments are exactly what these scenarios need to find.
#
# File contents are written out rather than generated, because `review.json` anchors
# each comment to a line number and a fixture that drifts moves the comment off the line
# it talks about. The owner comes from the authenticated login at run time, so no name is
# written down.
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

mkdir -p temperature

cat > README.md <<'EOF'
# temperature-cli

A small command line tool kept as a fixture for walkthrough evaluations. It
converts a temperature between Celsius and Fahrenheit.

    python -m temperature 21 C

prints `69.8 F`.
EOF

cat > AGENTS.md <<'EOF'
# AGENTS.md

Rules this repo sets for itself.

- Replies on a pull request are written in English.
- Documentation moves in the same change as the behaviour it describes.
EOF

: > temperature/__init__.py

cat > temperature/convert.py <<'EOF'
def to_fahrenheit(celsius):
    return celsius * 9 / 5 + 32


def to_celsius(fahrenheit):
    return (fahrenheit - 32) * 5 / 9
EOF

cat > temperature/__main__.py <<'EOF'
import sys

from temperature.convert import to_celsius, to_fahrenheit


def main(argv):
    value, unit = float(argv[1]), argv[2].upper()
    if unit == "C":
        print(f"{to_fahrenheit(value)} F")
    else:
        print(f"{to_celsius(value)} C")


if __name__ == "__main__":
    main(sys.argv)
EOF

git add -A
git commit -q -m "convert temperatures between Celsius and Fahrenheit"

# The pull request. Two planted comments go on it, and `review.json` holds them:
#
# - `temperature/convert.py` line 9 claims `format_reading` is never called. It is false,
#   and the call sites sit in this same diff, in `temperature/__main__.py`. The scenario
#   is whether a run checks before agreeing.
# - `README.md` line 10 says the example no longer matches the output. It is true, and
#   fixing it makes the last sentence of `pr-body.md` false, so a run that publishes has
#   a description to correct.
git checkout -q -b "$branch"

cat > temperature/convert.py <<'EOF'
def to_fahrenheit(celsius):
    return celsius * 9 / 5 + 32


def to_celsius(fahrenheit):
    return (fahrenheit - 32) * 5 / 9


def format_reading(value, unit):
    return f"{value:.1f} °{unit}"
EOF

cat > temperature/__main__.py <<'EOF'
import sys

from temperature.convert import format_reading, to_celsius, to_fahrenheit


def main(argv):
    value, unit = float(argv[1]), argv[2].upper()
    if unit == "C":
        print(format_reading(to_fahrenheit(value), "F"))
    else:
        print(format_reading(to_celsius(value), "C"))


if __name__ == "__main__":
    main(sys.argv)
EOF

cat >> README.md <<'EOF'

Every reading carries one decimal and the symbol of its unit.
EOF

git add -A
git commit -q -m "print every reading with one decimal and its unit symbol"

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
