---
type: regex
target: mock_calls
# After a write, the cards outside the scope are read again, to compare with the record.
pattern: '"tool":"mcp__linear__save_issue"[\s\S]*\n\{"tool":"mcp__linear__(?:get_issue|list_issues)"[^\n]*MOB-9\b'
---
