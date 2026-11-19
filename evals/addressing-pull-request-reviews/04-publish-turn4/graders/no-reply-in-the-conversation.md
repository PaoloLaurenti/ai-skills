---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# A reply posted with `gh pr comment` lands in the general conversation, detached from
# the line it answers.
pattern: '"pr","comment"'
match: not_contains
---
