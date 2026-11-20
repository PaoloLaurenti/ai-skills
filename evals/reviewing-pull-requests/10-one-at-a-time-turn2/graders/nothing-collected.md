---
type: regex
target: trace
# No answer yet, so nothing is written to the collected review.
pattern: '"name":"(?:Write|Edit)"[^\n]*state/reviews'
match: not_contains
---
