---
type: tool_order
# Nothing is written before the reviewer returns. The append is a mutation named on the
# command line, or a GraphQL request handed over in a file.
before: Agent
after: { tool: Bash, input_match: 'addPullRequestReviewThread|graphql --input' }
---
