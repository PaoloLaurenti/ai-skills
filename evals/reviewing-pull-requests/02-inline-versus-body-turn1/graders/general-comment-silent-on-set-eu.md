---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The general comment does not raise the entrypoint finding a second time.
pattern: '"input":\{"body":"(?:[^"\\]|\\.)*set -eu'
match: not_contains
---
