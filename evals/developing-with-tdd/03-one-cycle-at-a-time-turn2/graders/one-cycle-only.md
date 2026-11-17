---
type: regex
target: trace
# The plan holds the cart's two tests and six pending ones, so one cycle leaves 3 passed and
# 5 ignored, and no later run passes more: a second test turned green would be anticipated.
pattern: 'test result: ok\. 3 passed; 0 failed; 5 ignored(?![\s\S]*(?:test result: FAILED|could not compile|test result: ok\. [4-9] passed))'
---
