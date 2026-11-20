---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The README is linked at the head commit.
pattern: '"input":\{"body":"(?:[^"\\]|\\.)*\]\(https://github\.com/fixture-owner/ai-skills-eval-fixture/blob/9f2a142[0-9a-f]*/README\.md'
---
