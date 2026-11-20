---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# The last point decided, one draft is created from the collected review.
pattern: '^(?=[^\n]*"POST")[^\n]*/pulls/4/reviews"'
match: "count:1"
---
