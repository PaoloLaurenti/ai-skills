---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# One pending draft, created by a `POST` with no event.
pattern: '^(?=[^\n]*"POST")[^\n]*/pulls/13/reviews"'
match: "count:1"
---
