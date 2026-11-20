---
type: regex
target: { source: file, path: .git/fake-gh/calls.jsonl }
flags: m
# The reply goes into the draft on the two-decimals thread, by GraphQL, with the text
# given, its variables as `-f` fields or in a request file.
pattern: '^(?=[^\n]*addPullRequestReviewThreadReply)(?=[^\n]*"t(?:=|":")PRRT_4640020648")(?=[^\n]*"b(?:=|":")Still worth formatting the total with two decimals\.?")'
---
