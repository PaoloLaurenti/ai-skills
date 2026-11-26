---
type: regex
target: last_message
# The team's Definition of Ready sets the card's sections and their order, each a
# heading or a bold label, so a sentence merely naming them does not count.
pattern: '(?:#+ *|\*\*)Context\b[\s\S]*(?:#+ *|\*\*)User story\b[\s\S]*(?:#+ *|\*\*)Acceptance criteria\b[\s\S]*(?:#+ *|\*\*)Design\b'
---
