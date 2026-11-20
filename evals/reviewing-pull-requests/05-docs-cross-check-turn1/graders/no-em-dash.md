---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# Non-negotiable 4, over what the run sent to GitHub.
pattern: '—'
match: not_contains
---
