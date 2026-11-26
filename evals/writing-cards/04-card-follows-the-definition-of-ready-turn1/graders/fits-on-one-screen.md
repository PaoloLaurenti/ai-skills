---
type: regex
target: last_message
# A card fits on one screen: no 41 lines in a row without a `---` rule, which sets the
# card apart from what frames it, the summary of the settings and the notes after it.
pattern: '(?:(?!---(?:\n|$))[^\n]*\n){41}'
match: not_contains
---
