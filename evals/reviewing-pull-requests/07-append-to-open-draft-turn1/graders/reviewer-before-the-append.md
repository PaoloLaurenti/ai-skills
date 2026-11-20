---
type: tool_order
# Nothing is written before the reviewer returns.
before: Agent
after: { tool: Bash, input_match: 'addPullRequestReviewThread' }
---
