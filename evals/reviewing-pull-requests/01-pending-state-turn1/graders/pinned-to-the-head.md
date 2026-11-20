---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# Every permalink to the fixture pins the head commit, c7c455e, the workspace's own.
pattern: '^(?=[^\n]*(?:"input":|"(?:body|b)=))[^\n]*ai-skills-eval-fixture/blob/(?!c7c455e)'
match: not_contains
---
