#!/bin/sh
# Writes the fixture's two commits into the git repo in the current directory: `main`,
# the tool as it was, and the pull request's branch, named by the one argument. `setup.sh`
# pushes them to GitHub, and `workspace.sh` builds a case's workspace from them, so both
# hold the same files.
#
# File contents are written out rather than generated, because `review.json` anchors the
# planted comment to a line number and a fixture that drifts moves the comment off the
# line it talks about.
set -eu

branch="$1"

mkdir -p stock tests docs

cat > README.md <<'EOF'
# stock-cli

A small command line tool kept as a fixture for review evaluations. It keeps stock
levels in `stock.json` and prints a report.

    python -m stock report

The report format is in `docs/USAGE.md`.
EOF

cat > AGENTS.md <<'EOF'
# AGENTS.md

Rules this repo sets for itself.

- Documentation moves in the same change as the behaviour it describes.
- A test covers the boundary of every threshold it checks.
EOF

cat > docs/USAGE.md <<'EOF'
# Usage

`python -m stock report` prints one line per item, sorted by name:

    apples: 12
    pears: 3

Nothing else appears on a line.
EOF

: > stock/__init__.py

cat > stock/levels.py <<'EOF'
class Stock:
    def __init__(self, levels):
        self.levels = dict(levels)

    def add(self, item, quantity):
        self.levels[item] = self.levels.get(item, 0) + quantity

    def remove(self, item, quantity):
        self.levels[item] = max(0, self.levels.get(item, 0) - quantity)
EOF

cat > stock/report.py <<'EOF'
def report(stock):
    lines = []
    for item in sorted(stock.levels):
        lines.append(f"{item}: {stock.levels[item]}")
    return "\n".join(lines)
EOF

cat > stock/__main__.py <<'EOF'
import json
import sys

from stock.levels import Stock
from stock.report import report


def main(argv):
    with open("stock.json") as f:
        stock = Stock(json.load(f))
    if argv[1] == "report":
        print(report(stock))


if __name__ == "__main__":
    main(sys.argv)
EOF

cat > tests/test_levels.py <<'EOF'
from stock.levels import Stock


def test_removing_more_than_there_is_leaves_zero():
    stock = Stock({"apples": 2})
    stock.remove("apples", 5)
    assert stock.levels["apples"] == 0
EOF

git add -A
git commit -q -m "keep stock levels and print a report"

# The pull request, in two concepts, each carrying something for a review to find:
#
# - `stock/levels.py` adds the threshold. `is_low` compares with `<` where the comment and
#   the description say "at or below", and `review.json` plants a published comment on
#   that line saying so. A run is graded on whether it cites it instead of proposing it
#   again. The new test checks three units only, never the boundary AGENTS.md asks for.
# - `stock/report.py` marks low items. `docs/USAGE.md`, outside the diff, still says
#   nothing else appears on a line, which only the general comment can carry.
git checkout -q -b "$branch"

cat > stock/levels.py <<'EOF'
# An item at or below this many units is low.
LOW_STOCK = 5


class Stock:
    def __init__(self, levels):
        self.levels = dict(levels)

    def add(self, item, quantity):
        self.levels[item] = self.levels.get(item, 0) + quantity

    def remove(self, item, quantity):
        self.levels[item] = max(0, self.levels.get(item, 0) - quantity)

    def is_low(self, item):
        return self.levels.get(item, 0) < LOW_STOCK
EOF

cat > stock/report.py <<'EOF'
def report(stock):
    lines = []
    for item in sorted(stock.levels):
        marker = " (low)" if stock.is_low(item) else ""
        lines.append(f"{item}: {stock.levels[item]}{marker}")
    return "\n".join(lines)
EOF

cat >> tests/test_levels.py <<'EOF'


def test_an_item_under_the_threshold_is_low():
    stock = Stock({"pears": 3})
    assert stock.is_low("pears")
EOF

git add -A
git commit -q -m "mark items at or below five units as low in the report"
