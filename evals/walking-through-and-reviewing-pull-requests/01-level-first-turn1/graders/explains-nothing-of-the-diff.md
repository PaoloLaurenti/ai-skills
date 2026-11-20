---
type: regex
target: trace
flags: m
# No reply of the main session names what the diff changes. The reviewer's messages, which
# carry a parent tool use, may.
pattern: '^(?=[^\n]*"parent_tool_use_id":null)(?=[^\n]*"type":"text")\{"type":"assistant"[^\n]*(?:is_low|LOW_STOCK|\(low\))'
match: not_contains
---
