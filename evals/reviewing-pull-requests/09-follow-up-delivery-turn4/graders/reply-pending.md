---
type: regex
target: { source: file, path: .git/fake-gh/pull-request.json }
# The state the stand-in saved: the reply answers the two-decimals comment inside a
# review still pending.
pattern: '"pull_request_review_id":(\d+),"in_reply_to_id":4640020648,[^\n]*\{"id":\1,"node_id":"[^"]*","user":\{[^{}]*\},"state":"PENDING"'
---
