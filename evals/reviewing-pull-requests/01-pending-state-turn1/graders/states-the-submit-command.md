---
type: regex
target: last_message
# The reply names the command that submits the review, so the pending review is no
# dead end.
pattern: '/reviews/[^/\s`]+/events'
---
