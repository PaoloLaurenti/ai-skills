---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# No REST write, no comment, no edit.
pattern: '"(?:POST|PATCH|PUT|DELETE)"|"pr","(?:comment|edit|review)"'
match: not_contains
---
