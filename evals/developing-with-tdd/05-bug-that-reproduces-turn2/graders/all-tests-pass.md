---
type: regex
target: trace
# tests/promo.rs holds the four tests of the fixture and the regression test: 5 passed,
# none ignored, and nothing failing or failing to build afterwards.
pattern: 'test result: ok\. 5 passed; 0 failed; 0 ignored(?![\s\S]*(?:test result: FAILED|could not compile))'
---
