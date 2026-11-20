---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# Every permalink to the fixture pins the head commit, ba4c263, the workspace's own.
pattern: '^(?=[^\n]*(?:"input":|"(?:body|b)=))[^\n]*ai-skills-eval-fixture/blob/(?!ba4c263)'
match: not_contains
---
