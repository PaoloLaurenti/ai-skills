---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The pending review created in one call, its comments and body together.
pattern: '"POST"[^\n]*"repos/[^"]*/pulls/1/reviews"'
match: "count:1"
---
