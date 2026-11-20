---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# Every permalink to the fixture pins the head commit, 9f2a142, the workspace's own.
pattern: '^(?=[^\n]*(?:"input":|"(?:body|b)=))[^\n]*ai-skills-eval-fixture/blob/(?!9f2a142)'
match: not_contains
---
