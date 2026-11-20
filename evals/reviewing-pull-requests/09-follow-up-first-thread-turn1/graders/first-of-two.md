---
type: regex
target: last_message
# Two threads counted, the line 8 and line 14 ones: the line 18 thread has neither reply
# nor change, and waits for the final turn.
pattern: '\b1 (?:of|di|su) 2\b'
---
