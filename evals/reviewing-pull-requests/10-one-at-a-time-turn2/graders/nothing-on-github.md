---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# Nothing reaches GitHub before the last point.
pattern: '"(?:POST|PATCH|PUT|DELETE)"|(?:query=|"query":")(?:\s|\\n)*mutation|"pr","(?:comment|edit|review)"'
match: not_contains
---
