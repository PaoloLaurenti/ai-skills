---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# A line anchor into README.md points at lines 8 and 9, which say there are no
# environment variables.
pattern: 'blob/[0-9a-f]+/README\.md#L(?![89](?!\d))'
match: not_contains
---
