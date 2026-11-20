---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The prose proposed for AGENTS.md carries neither the stale pair nor the right one:
# documentation does not carry values the code owns.
pattern: '\{"body":"(?:[^"\\]|\\.)*```suggestion(?:(?!```)(?:[^\"\\]|\\.))*(?<![\d.])(?:30|100|45|250)(?!\d)(?:[^"\\]|\\.)*","line":\d+,"path":"AGENTS\.md"'
match: not_contains
---
