---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The finding sits in the general comment, README.md not being in the diff.
pattern: '"input":\{"body":"(?:[^"\\]|\\.)*README\.md'
---
