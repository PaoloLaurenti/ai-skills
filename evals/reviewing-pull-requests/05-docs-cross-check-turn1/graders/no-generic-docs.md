---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# Never "the docs": the file is named.
pattern: '^(?=[^\n]*(?:"input":|"(?:body|b)=))[^\n]*\b[Tt]he docs\b'
match: not_contains
---
