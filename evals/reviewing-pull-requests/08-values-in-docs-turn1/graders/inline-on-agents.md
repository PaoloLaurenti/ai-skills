---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# AGENTS.md is in the diff, so the finding on its new section sits inline there.
pattern: '"path":"AGENTS\.md"'
---
