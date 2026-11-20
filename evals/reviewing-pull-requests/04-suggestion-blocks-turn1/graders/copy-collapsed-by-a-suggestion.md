---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
# The COPY and the chmod collapse into one `COPY --chmod`, handed over as a suggestion
# whose anchor starts on the COPY, line 5, and covers the lines it replaces.
pattern: '\{"body":"(?:[^"\\]|\\.)*```suggestion\\n(?:[^"\\]|\\.)*--chmod(?:[^"\\]|\\.)*","line":[67],"path":"Dockerfile"[^}]*"start_line":5\b'
---
