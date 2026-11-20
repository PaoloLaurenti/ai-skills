---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# Nothing published or removed.
pattern: '"(?:PATCH|PUT|DELETE)"|/replies"|"pr","(?:comment|edit)"'
match: not_contains
---
