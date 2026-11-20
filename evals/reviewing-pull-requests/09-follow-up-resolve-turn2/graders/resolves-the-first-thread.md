---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The yes resolves the line 8 thread at once, and only that one.
pattern: 'resolveReviewThread[^\n]*"t(?:=|":")PRRT_4640020626"'
match: "count:1"
---
