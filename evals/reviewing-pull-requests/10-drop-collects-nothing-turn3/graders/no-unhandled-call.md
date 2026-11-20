---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# A call the stand-in for `gh` does not know: the skill reached for something new.
pattern: '"unhandled"'
match: not_contains
---
