---
type: regex
target: { source: file, path: .git/fake-gh/pull-request.json }
# The state the stand-in saved: the inline comment belongs to a review still `PENDING`,
# whose body is the general comment made from the finding held on docs/USAGE.md.
pattern: '"pull_request_review_id":(\d+),"path":"tests/test_levels\.py"[^\n]*\{"id":\1,"node_id":"[^"]*","user":\{[^{}]*\},"state":"PENDING","body":"(?:[^"\\]|\\.)*docs/USAGE\.md'
---
