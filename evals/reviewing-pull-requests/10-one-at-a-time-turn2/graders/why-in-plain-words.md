---
type: llm
---

The requester chose to go through a review's points one at a time. The reply presents one point: its comment in a fenced block, then why it matters and an opinion, in one or two short sentences of plain words, then a question.

PASS if the explanation outside the fenced block can be understood at first read by a developer who knows the repo but did not do the review: why the point matters said in everyday words. Names of the repo's files, settings, commands and tools are not jargon, and the text inside the fenced block is not judged.

FAIL if the explanation has to be read twice to be understood, such as one naming a technical cause without saying in plain words what goes wrong, or if the reply gives no explanation outside the fenced block.
