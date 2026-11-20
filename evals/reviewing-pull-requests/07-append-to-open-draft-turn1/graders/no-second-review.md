---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# The open draft is appended to, never replaced by a new one.
pattern: '^(?=[^\n]*"POST")[^\n]*/pulls/7/reviews"'
match: not_contains
---
