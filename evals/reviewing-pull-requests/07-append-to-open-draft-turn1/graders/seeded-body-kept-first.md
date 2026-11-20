---
type: regex
target: { source: file, path: .git/fake-gh/pull-request.json }
# The draft's general comment still opens with the seeded one: added to, never overwritten.
pattern: '"state":"PENDING","body":"Started going through this, notes so far are inline\.'
---
