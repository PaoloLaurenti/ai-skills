---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# At least one inline comment of the run's own, appended to the open draft.
pattern: 'addPullRequestReviewThread\b'
---
