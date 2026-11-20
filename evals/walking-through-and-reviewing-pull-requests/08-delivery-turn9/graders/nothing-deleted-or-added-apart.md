---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# No draft deleted and recreated, no comment added to it on its own, nothing published
# to the conversation or the description.
pattern: '"DELETE"|query=mutation|"comment"|"edit"'
match: not_contains
---
