---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# No submit, by REST or by `gh pr review`.
pattern: '/events"|"event=|"pr","review"'
match: not_contains
---
