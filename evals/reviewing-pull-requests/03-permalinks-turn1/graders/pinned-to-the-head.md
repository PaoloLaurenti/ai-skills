---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# Every permalink to the fixture pins the head commit, ac706d1, the workspace's own.
pattern: '^(?=[^\n]*(?:"input":|"(?:body|b)=))[^\n]*ai-skills-eval-fixture/blob/(?!ac706d1)'
match: not_contains
---
