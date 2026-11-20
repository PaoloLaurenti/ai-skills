---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# README.md is outside the diff, so no inline comment can sit on it.
pattern: '"path":"README\.md"|"path=README\.md"'
match: not_contains
---
