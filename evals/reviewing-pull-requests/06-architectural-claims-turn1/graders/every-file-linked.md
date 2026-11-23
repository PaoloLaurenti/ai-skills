---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# Code references: every repo file named in a comment sits inside a markdown link. Not
# a mention inside a link's URL, after `/`, nor inside a fenced block, a quoted
# replacement sentence or a `>` line quoting one, where a link would corrupt the text
# handed over: the mention is read only where the comment's text before it holds an even
# number of fences and of escaped quotes, and outside a line opening with `>`. Nor after
# "a", as in "a Dockerfile `HEALTHCHECK`", which names a kind of file rather than this
# repo's.
pattern: '(?<=(?:"body":"|"b(?:ody)?=)(?:(?!>)|>(?:[^"\\]|\\[^n])*(?=\\n|"))(?:(?:[^"\\`]|\\[^"n]|\\n(?!>)|`(?!``))|\\n>(?:[^"\\]|\\[^n])*(?=\\n|")|```(?:[^"\\`]|\\.|`(?!``))*```|\\"(?:[^"\\`]|\\[^"]|`(?!``))*\\")*)(?<![/\w.=-])(?<!\b[Aa] )(?:README\.md|AGENTS\.md|Dockerfile|(?:config/)?default\.toml|(?:\.github/workflows/)?deploy\.yml|(?:app/)?(?:server|migrate|limits|pricing|currency)\.py)(?![^\[\]\n]*\]\()'
match: not_contains
---
