---
type: regex
target: { source: file, path: origin.git/logs/refs/heads/walkthrough }
# The scaffold's own push is the first; the run's is the second.
pattern: '\tpush'
match: "count:2"
---
