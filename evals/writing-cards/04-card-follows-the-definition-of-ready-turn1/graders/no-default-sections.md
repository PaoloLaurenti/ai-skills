---
type: regex
target: last_message
# The default sections stay out when the Definition of Ready sets its own. Only a heading
# or a bold label naming one counts: a note such as "**Why it's in To refine:**" is not.
pattern: '(?:^|\n)\s*(?:#+ *(?:What|Why) *(?:\n|$)|\*\*(?:What|Why):?\*\*)'
match: not_contains
---
