---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# A suggestion block holds the final code, with no line opening with `+` or `-`.
pattern: '```suggestion(?:\\n(?:(?!```)(?:[^"\\]|\\[^n]))*)*?\\n[+-]'
match: not_contains
---
