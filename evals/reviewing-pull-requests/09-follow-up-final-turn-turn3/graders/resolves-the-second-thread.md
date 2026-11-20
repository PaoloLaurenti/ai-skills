---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The yes resolves the line 14 thread, and only that one.
pattern: 'resolveReviewThread[^\n]*"t(?:=|":")PRRT_4640020638"'
match: "count:1"
---
