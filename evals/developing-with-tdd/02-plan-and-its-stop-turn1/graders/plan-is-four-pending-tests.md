---
type: regex
target: trace
# One Write or Edit holding at least four `#[ignore`, one per rule in FEATURE.md. Kept to
# a single call's line so the skill's own text, which names `#[ignore]`, is not counted;
# the `feature` fixture has no pending test, so an Edit cannot bring one in either.
pattern: '"name":"(?:Write|Edit)","input":\{[^\n]*?(?:#\[ignore[^\n]*?){4}'
---
