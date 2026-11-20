---
type: tool_order
# Nothing is written before the reviewer returns.
before: Agent
after: { tool: Bash, input_match: 'POST[^"]*/pulls/5/reviews' }
---
