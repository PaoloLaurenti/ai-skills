---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# A line anchor into config/default.toml points at the `[limits]` section, lines 7 and 8.
pattern: 'blob/[0-9a-f]+/config/default\.toml#L(?![78](?!\d))'
match: not_contains
---
