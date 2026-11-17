---
type: regex
target: trace
# After the cycle on the cart total, 2 passed and 1 ignored, and no later run left nothing
# ignored: going on to the empty cart would have skipped the review's stop. The doc-test
# line, 0 passed, comes last in every run and is not counted.
pattern: 'test result: ok\. 2 passed; 0 failed; 1 ignored(?![\s\S]*(?:test result: FAILED|could not compile|test result: ok\. [1-9]\d* passed; 0 failed; 0 ignored))'
---
