#!/bin/sh
# Commits, on the `follow-up` branch checked out in the current directory, the author's
# answer to the submitted review: the loop becomes `sum()`, VAT applies to the whole
# amount, and a currency module no thread asked for. `seed-submitted-review.sh` pushes it
# to GitHub, and `workspace.sh` writes it into a case's workspace, so both hold the same
# commit.
set -eu

perl -0pi -e 's{    total = 0\.0\n    for price in prices:\n        total = total \+ price\n    return total\n}{    return sum(prices)\n}; s{amount \* 1 \+ VAT}{amount * (1 + VAT)}' app/pricing.py
cat > app/currency.py <<'PY'
"""The currency totals are printed in. A fixture, not a real converter."""

CURRENCY = "EUR"
PY
git add -A
git commit -qm "sum the prices, apply VAT to the whole amount, name the currency"
