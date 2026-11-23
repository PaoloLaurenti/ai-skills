---
type: llm
---

The requester chose to go through the review's points one at a time before any draft is created. The skill gives one point per turn, in this order, and nothing about any other point: the point's number and total with the file and line linked; the comment exactly as the draft would carry it, in a fenced block; why it matters and an opinion, in one or two short sentences; one question: add, add with a change, or drop.

PASS if the reply presents a single point, shows its comment text in a fenced block, opened by three or more backticks, and closes on one question offering to add it, change it or drop it, in any wording and language.

FAIL if it presents more than one point, shows no comment text, creates or mentions creating a draft now, or asks no question.
