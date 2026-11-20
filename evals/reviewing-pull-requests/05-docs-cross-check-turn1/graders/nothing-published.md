---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# Nothing that publishes or removes: no reply posted on its own, no comment on the
# conversation, no edit of the description, no PATCH, PUT or DELETE.
pattern: '"(?:PATCH|PUT|DELETE)"|/replies"|"pr","(?:comment|edit)"'
match: not_contains
---
