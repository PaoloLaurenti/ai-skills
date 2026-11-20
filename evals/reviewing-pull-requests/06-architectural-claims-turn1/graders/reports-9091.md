---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The port disagreement inside the diff: the config sets 9091.
pattern: '"input":[^\n]*\b9091\b'
---
