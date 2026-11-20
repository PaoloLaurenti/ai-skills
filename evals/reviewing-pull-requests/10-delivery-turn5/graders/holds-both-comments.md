---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The draft holds the two comments added, the changed COPY one and the debug line, and
# nothing else.
pattern: '"comments":\[\{[^{}]*--chmod=755[^{}]*"path":"Dockerfile"[^{}]*\},\{[^{}]*"line":4,"path":"docker-entrypoint\.sh"[^{}]*\}\]'
---
