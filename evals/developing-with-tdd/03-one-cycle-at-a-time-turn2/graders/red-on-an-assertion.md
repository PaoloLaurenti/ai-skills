---
type: regex
target: trace
# The red comes from a failed assertion, not a "not implemented" panic, before the green.
# Either line proves it: Claude sometimes filters `cargo test` through grep and keeps only
# the `left:` and `right:` of a failed `assert_eq!`, dropping the "assertion failed" line.
pattern: '(?:assertion[^\n]{0,40}failed|\bleft:\s)[\s\S]*test result: ok\. 3 passed; 0 failed; 5 ignored'
---
