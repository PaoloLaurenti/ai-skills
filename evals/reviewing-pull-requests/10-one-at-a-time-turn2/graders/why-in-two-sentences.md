---
type: llm
---

The requester chose to go through a review's points one at a time. The reply presents one point: the point's number with its file and line, its comment in a fenced block, then why it matters and an opinion in one or two short sentences, then a question.

PASS if the text between the fenced block and the closing question, saying why the point matters and giving an opinion, runs to at most three sentences, in any language.

FAIL if that text runs to four sentences or more, such as several paragraphs or a list of reasons.
