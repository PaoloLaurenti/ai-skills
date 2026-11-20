---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# One change reaches GitHub, the resolution: no reply, no draft, nothing published.
pattern: '(?:query=|"query":")(?:\s|\\n)*mutation'
match: "count:1"
---
