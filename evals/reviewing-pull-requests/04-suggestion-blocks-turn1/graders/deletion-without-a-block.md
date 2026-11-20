---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The debug line on line 4 should just go: a pure deletion gets no suggestion block.
pattern: '\{"body":"(?:[^"\\]|\\.)*```suggestion(?:[^"\\]|\\.)*","line":4,"path":"docker-entrypoint\.sh"'
match: not_contains
---
