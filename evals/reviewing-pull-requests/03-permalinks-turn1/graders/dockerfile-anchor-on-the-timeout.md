---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# A line anchor into the Dockerfile points at line 9, `ENV APP_TIMEOUT=60`, the only
# line of it the finding is about.
pattern: 'blob/[0-9a-f]+/Dockerfile#L(?!9(?!\d))'
match: not_contains
---
