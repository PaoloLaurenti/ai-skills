---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The port disagreement inside the diff: the Dockerfile exposes 9090.
pattern: '"input":[^\n]*\b9090\b'
---
