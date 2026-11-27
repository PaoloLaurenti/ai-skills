---
type: regex
target: trace
# The build still works and no test fails: some `cargo test` reported ok, and nothing
# after it failed or stopped compiling. Both are read from tool results only, the trace
# lines opening with `{"type":"user"`, so Claude's own words quoting a failure count for
# nothing, and neither does a success it only claims.
pattern: '(?:^|\n)\{"type":"user"[^\n]*test result: ok\.(?![^\n]*(?:test result: FAILED|could not compile))(?![\s\S]*\n\{"type":"user"[^\n]*(?:test result: FAILED|could not compile))'
---
