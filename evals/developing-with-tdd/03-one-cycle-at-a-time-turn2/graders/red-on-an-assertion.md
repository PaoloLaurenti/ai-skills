---
type: regex
target: trace
# The red comes from a failed assertion, not a "not implemented" panic, before the green.
pattern: 'assertion[^\n]{0,40}failed[\s\S]*test result: ok\. 3 passed; 0 failed; 5 ignored'
---
