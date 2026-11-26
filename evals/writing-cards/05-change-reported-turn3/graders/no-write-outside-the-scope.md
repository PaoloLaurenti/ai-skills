---
type: regex
target: mock_calls
# No write names MOB-7 or MOB-9, by id or by url: some trackers turn either into a change
# on the card named, and both are outside the scope.
pattern: '"tool":"mcp__linear__save_issue","input":\{(?:(?!"output":)[^\n])*MOB-(?:7|9)\b'
match: not_contains
---
