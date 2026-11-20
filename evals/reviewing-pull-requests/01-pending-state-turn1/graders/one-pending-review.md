---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# The review created in one call, its comments and general comment together.
pattern: '^(?=[^\n]*"POST")[^\n]*/pulls/14/reviews"'
match: "count:1"
---
