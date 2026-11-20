---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# No call that changes the pull request: a REST write, `gh pr comment`, `gh pr edit`,
# `gh pr review`, or a GraphQL mutation.
pattern: '"(?:POST|PATCH|PUT|DELETE|comment|edit|review)"|(?:query=|"query":")(?:\s|\\n)*mutation'
match: not_contains
---
