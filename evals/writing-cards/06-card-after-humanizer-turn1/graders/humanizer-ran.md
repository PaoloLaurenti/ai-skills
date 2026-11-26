---
type: regex
target: trace
# The skill passes the card through `humanizer` where it is installed. Only a call that
# loaded it prints this, a refused or misnamed one does not.
pattern: 'Launching skill: (?:[\w-]+:)?humanizer'
---
