---
type: regex
target: last_message
# A card fits on one screen: no 41 lines in a row without a horizontal rule, which sets
# the card apart from what frames it, the summary of the settings and the notes after
# it. Three dashes are written `-{3}`, since the frontmatter ends at the first three
# dashes anywhere in this file, a comment's included.
pattern: '(?:(?!-{3}(?:\n|$))[^\n]*\n){41}'
match: not_contains
---
