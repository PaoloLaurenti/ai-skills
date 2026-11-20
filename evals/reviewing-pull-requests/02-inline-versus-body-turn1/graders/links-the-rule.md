---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The inline comment names the rule it breaks by linking the AGENTS.md that sets it.
pattern: '\{"body":"(?:[^"\\]|\\.)*\]\(https://github\.com/fixture-owner/ai-skills-eval-fixture/blob/[0-9a-f]+/AGENTS\.md(?:[^"\\]|\\.)*","line":\d+,"path":"docker-entrypoint\.sh"'
---
