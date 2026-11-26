---
type: regex
target: mock_calls
# A link between new cards is added once the card it points to exists: MOB-20 is known
# only from the answer to its create, and the link's text is that card's title.
pattern: '"tool":"mcp__linear__save_issue","input":\{(?:(?!"output":)[^\n])*\[Request a reset link\]\([^)]*MOB-20\b'
---
