---
type: regex
target: mock_calls
# After the yes, the second new card is created.
pattern: '"tool":"mcp__linear__save_issue","input":\{(?:(?!"output":)[^\n])*"title":"Set a new password"'
---
