---
type: regex
target: trace
flags: mi
# A reply of the main session, not of the reviewer, offering the levels: one trace line
# per message part, the main session's with no parent tool use. The menu names the expert
# level and the repository, in whatever words.
pattern: '^(?=[^\n]*"parent_tool_use_id":null)(?=[^\n]*"type":"text")(?=[^\n]*(?:espert|expert))(?=[^\n]*repo)\{"type":"assistant"'
---
