---
type: llm
---

The planned tests are listed in the order they will be written, which the skill fixes as: happy paths, simplest first; then degenerate and empty cases, such as an empty cart, which close the happy paths; then domain errors, here an unknown and an expired code; then input validation; then edge cases, such as a code used on its expiry date. Rounding the discount down to the whole cent is a happy path, applying the discount with one more rule, not an edge case.

PASS if the listed order follows those groups, with any group allowed to be absent: no domain error before a happy path or an empty case, and no edge case before a domain error.

FAIL if the order breaks those groups, or if the reply shows no order.
