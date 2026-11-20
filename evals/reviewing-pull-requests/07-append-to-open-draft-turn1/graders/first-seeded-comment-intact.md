---
type: regex
target: { source: file, path: .git/fake-gh/pull-request.json }
# The clean seeded comment survives, word for word, in the draft as the stand-in keeps it.
pattern: '"path":"Dockerfile","line":9,[^\n]*?"body":"Worth confirming the orchestrator actually consumes HEALTHCHECK here, since some schedulers ignore it and run their own probe\."'
---
