---
type: regex
target: trace
# A dropped point adds nothing to the collected review.
pattern: '"name":"(?:Write|Edit)"[^\n]*state/reviews'
match: not_contains
---
