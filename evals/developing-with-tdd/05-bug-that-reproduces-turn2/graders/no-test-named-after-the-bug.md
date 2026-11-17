---
type: regex
target: trace
# Nowhere in the run a function whose name holds bug, issue, fix or regression as a word.
# Matched as a word, since a helper such as `stub_fixed_at` holds `fix`.
pattern: '^(?![\s\S]*\bfn (?:\w+_)?(?:bug|issue|fix|regression)(?:_\w*)?\s*\()'
---
