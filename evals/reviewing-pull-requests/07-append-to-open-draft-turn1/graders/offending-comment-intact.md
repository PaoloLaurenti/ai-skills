---
type: regex
target: { source: file, path: .git/fake-gh/pull-request.json }
# The seeded comment breaking the conventions survives word for word: never edited to fit
# them.
pattern: '"path":"Dockerfile","line":10,[^\n]*?"body":"I think we should double-check this one — the base image is slim, so I''m not sure curl is even in there\."'
---
