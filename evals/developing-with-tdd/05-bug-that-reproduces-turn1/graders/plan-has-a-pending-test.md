---
type: regex
target: trace
# One Write or Edit holding a pending test. The `implemented` fixture has none, so an
# Edit cannot bring one in with the text it replaces.
pattern: '"name":"(?:Write|Edit)","input":\{[^\n]*?#\[ignore'
---
