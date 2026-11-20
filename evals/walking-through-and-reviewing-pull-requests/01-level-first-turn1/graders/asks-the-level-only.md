---
type: llm
---

The request asks for a guided walkthrough of a pull request before reviewing it. The skill asks, in the first turn, how much to explain, as a closed menu of levels (new to the repository, knows the repository but not the technology, expert), and nothing else: no step, no explanation of the change, before the answer. A reviewer dispatched in the background may finish inside the turn, and the session then writes one more short message, such as a line saying it is still waiting for the level, with the menu in the message before it.

The text judged is the turn's final message. PASS if it is the menu of levels, or a short line waiting for the level to be chosen, in any language and any wording. Mentioning the reviewer, the pull request's title or number, or that the review has not started is not a reason to fail.

FAIL if it presents a step of the walkthrough, explains what the pull request changes or why, lists findings or proposed comments, or gives a verdict on the pull request.
