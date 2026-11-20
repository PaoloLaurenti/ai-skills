---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The comment on AGENTS.md names the file that owns the values, so the repair ends the
# duplication instead of renewing it.
pattern: '\{"body":"(?:[^"\\]|\\.)*limits\.py(?:[^"\\]|\\.)*","line":\d+,"path":"AGENTS\.md"'
---
