---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# One point per comment: no inline comment's prose, before any fenced block, runs past
# 80 words.
pattern: '(?:[\[,]\{"body":"|addPullRequestReviewThread[^\n]*?(?:"body=|"variables":\{"body":"))(?:(?!```)(?:[^"\\\s]|\\[^n]))+(?:(?:\s|\\n)+(?:(?!```)(?:[^"\\\s]|\\[^n]))+){80}'
match: not_contains
---
