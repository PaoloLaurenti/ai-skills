---
type: regex
target: trace
# Step 3 follows `grilling` where it is installed. Only a call that loaded it prints
# this, a refused or misnamed one does not.
pattern: 'Launching skill: (?:[\w-]+:)?grilling'
---
