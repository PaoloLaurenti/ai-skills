---
type: regex
target: trace
# Nothing is written before the requester answers: no card saved, no setting noted.
pattern: '"name":"(?:Write|Edit)"'
match: not_contains
---
