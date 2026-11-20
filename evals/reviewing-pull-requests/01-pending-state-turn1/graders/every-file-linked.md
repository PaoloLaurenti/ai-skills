---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# Code references: every repo file named in a comment sits inside a markdown link. Not
# a mention inside a link's URL, after `/`, nor inside a fenced block or a quoted
# replacement sentence, where a link would corrupt the text handed over: the mention is
# read only where the comment's text before it holds an even number of fences and of
# escaped quotes.
pattern: '(?<=(?:"body":"|"b(?:ody)?=)(?:(?:[^"\\`]|\\[^"]|`(?!``))|```(?:[^"\\`]|\\.|`(?!``))*```|\\"(?:[^"\\`]|\\[^"]|`(?!``))*\\")*)(?<![/\w.=-])(?:README\.md|AGENTS\.md|Dockerfile|(?:config/)?default\.toml|(?:\.github/workflows/)?deploy\.yml|(?:app/)?(?:server|migrate|limits|pricing|currency)\.py)(?![^\[\]\n]*\]\()'
match: not_contains
---
