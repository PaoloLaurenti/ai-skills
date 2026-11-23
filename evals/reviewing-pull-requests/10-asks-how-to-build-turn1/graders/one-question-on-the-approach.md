---
type: llm
---

The requester asked for an opinion on a pull request without saying how the review should be built. Once the reviewer's findings are filtered, the skill asks one question: create the pending draft now with every point, or go through the points one at a time first. Before it, one line per point and one counting what was set aside. Nothing is posted before the answer, and no comment text is shown yet.

PASS if the reply closes by asking which of the two approaches to take, in any wording and language. Listing the points briefly before the question is not a reason to fail.

FAIL if the reply shows the full text of a review comment, says the draft was created or published, starts walking the first point, or does not ask how to build the review.
