---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# Every permalink to the fixture pins the head commit, b490640, the workspace's own.
pattern: '^(?=[^\n]*(?:"input":|"(?:body|b)=))[^\n]*ai-skills-eval-fixture/blob/(?!b490640)'
match: not_contains
---
