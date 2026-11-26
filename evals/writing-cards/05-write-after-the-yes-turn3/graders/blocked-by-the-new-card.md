---
type: regex
target: mock_calls
# The "blocked by" relation names the first new card by the id its create returned.
pattern: '"tool":"mcp__linear__save_issue","input":\{(?:(?!"output":)[^\n])*"blockedBy":\[[^\]]*"MOB-20"'
---
