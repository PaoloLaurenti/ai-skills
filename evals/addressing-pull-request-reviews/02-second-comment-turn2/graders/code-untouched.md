---
type: regex
target: trace
# The first comment's answer changes no code, so nothing in the repo is written. Saving
# the accepted reply goes to a file outside the working tree, which this leaves alone.
pattern: '"name":"(?:Write|Edit)","input":\{[^\n]*?"file_path":"[^"]*/(?:temperature/|README\.md|AGENTS\.md)'
match: not_contains
---
