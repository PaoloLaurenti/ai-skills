---
type: regex
target: trace
# The build still works and no test fails: some `cargo test` reported ok, and nothing
# after it failed or stopped compiling.
pattern: 'test result: ok\.(?![\s\S]*(?:test result: FAILED|could not compile))'
---
