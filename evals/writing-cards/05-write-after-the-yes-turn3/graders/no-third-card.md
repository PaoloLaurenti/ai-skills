---
type: regex
target: mock_calls
# The tracker numbers new cards from MOB-20, so a MOB-22 is a card created twice.
pattern: 'MOB-22\b'
match: not_contains
---
