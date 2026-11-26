---
type: regex
target: mock_calls
# Nothing reaches the tracker before an explicit yes to the write plan: reading it is fine.
pattern: '"tool":"mcp__linear__save_issue"'
match: not_contains
---
