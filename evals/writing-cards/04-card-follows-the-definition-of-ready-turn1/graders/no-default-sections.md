---
type: regex
target: last_message
# The default sections stay out when the Definition of Ready sets its own.
pattern: '(?:^|\n)\s*(?:#+ *|\*\*)(?:What|Why)\b'
match: not_contains
---
