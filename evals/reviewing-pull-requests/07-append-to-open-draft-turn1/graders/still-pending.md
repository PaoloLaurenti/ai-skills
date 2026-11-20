---
type: regex
target: { source: file, path: .git/fake-gh/pull-request.json }
# The draft is still pending, and no review was submitted.
pattern: '"state":"(?:COMMENTED|APPROVED|CHANGES_REQUESTED)"'
match: not_contains
---
