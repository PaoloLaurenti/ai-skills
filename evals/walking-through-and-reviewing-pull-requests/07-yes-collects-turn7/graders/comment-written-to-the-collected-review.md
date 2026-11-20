---
type: regex
target: trace
flags: m
# A Write of the collected review, `~/.local/state/reviews/OWNER-REPO-N.json`, holding the
# approved comment on the test. The file sits outside the workspace, where no grader
# reads, so its content is read from the call.
pattern: '^(?=[^\n]*"name":"Write")(?=[^\n]*state/reviews/[^"\\]*-1\.json)(?=[^\n]*tests/test_levels\.py)'
---
