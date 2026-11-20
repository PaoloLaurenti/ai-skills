---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# The finding needs README.md, outside the diff, so the review links it.
pattern: '^(?=[^\n]*(?:"input":|"(?:body|b)=))[^\n]*\]\(https://github\.com/fixture-owner/ai-skills-eval-fixture/blob/ac706d1[0-9a-f]*/README\.md'
---
