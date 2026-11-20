---
type: regex
flags: m
# Step 6's header, bold at the start of a line as every recorded step writes it: the next
# step comes only once each comment of this one has an answer. The invitation naming
# step 6 is not a header.
pattern: '^\*\*6\s*/\s*7'
match: not_contains
---
