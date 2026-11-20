---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# No submit: no `event` sent with the review or after it, no `gh pr review`.
pattern: '"event"|"event=|/events"|"pr","review"'
match: not_contains
---
