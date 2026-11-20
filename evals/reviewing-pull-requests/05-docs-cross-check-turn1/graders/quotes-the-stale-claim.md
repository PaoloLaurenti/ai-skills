---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The stale claim is quoted rather than paraphrased, so its port appears.
pattern: '"input":\{"body":"(?:[^"\\]|\\.)*8080'
---
