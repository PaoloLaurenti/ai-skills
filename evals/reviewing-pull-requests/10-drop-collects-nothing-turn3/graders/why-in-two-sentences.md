---
type: llm
---

The requester is going through a review's points one at a time and has just answered on one. The reply acknowledges that answer, then presents the next point: its number with its file and line, its comment in a fenced block, then why it matters and an opinion in one or two short sentences, then a question.

PASS if the text between the next point's fenced block and the closing question, saying why the point matters and giving an opinion, runs to at most three sentences, in any language. What comes before the fenced block, such as the acknowledgement of the previous answer, is not counted.

FAIL if that text runs to four sentences or more, such as several paragraphs or a list of reasons.
