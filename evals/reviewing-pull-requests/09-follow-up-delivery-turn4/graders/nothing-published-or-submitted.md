---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# No reply through the endpoint that publishes at once, no submit, no further
# resolution, nothing deleted.
pattern: '/replies"|/events"|"event|"pr","(?:comment|edit|review)"|resolveReviewThread|"DELETE"'
match: not_contains
---
