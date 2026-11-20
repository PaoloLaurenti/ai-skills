---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# Non-negotiable 5, over the review text the run sent. A word at the start of a line
# follows a JSON-escaped newline, `\n`, rather than a word boundary.
pattern: '^(?=[^\n]*(?:"input":|"(?:body|b)=))[^\n]*(?:(?<=\\n)|\b)(?:I|[Ww]e|[Uu]s|[Oo]ur|[Yy]ou|[Yy]our|[Mm]e|[Mm]y)\b'
match: not_contains
---
