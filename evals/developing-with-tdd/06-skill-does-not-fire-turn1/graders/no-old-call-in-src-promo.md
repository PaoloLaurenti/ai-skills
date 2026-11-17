---
type: regex
target: { source: file, path: src/promo.rs }
# One of the two places calling the old name. With both updated and `fn total_price`
# in place, the rename is complete and the crate builds, whether or not the run tested it.
pattern: '^(?![\s\S]*\.total\(\))'
---
