---
type: regex
target: last_message
flags: i
# The feature leaves how long a link stays valid undecided: no duration is made up.
pattern: '(?:valid|expir)[^\n]{0,60}\b\d+\s*(?:h\b|hours?|minutes?|mins?\b|days?)'
match: not_contains
---
