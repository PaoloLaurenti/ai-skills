---
type: regex
target: mock_calls
# After the yes, the first new card is created.
pattern: '"tool":"mcp__linear__save_issue","input":\{(?:(?!"output":)[^\n])*"title":"Request a reset link"'
---
