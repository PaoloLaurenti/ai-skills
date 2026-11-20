---
type: llm
---

The request asked to leave a review on a pull request. The skill creates the review in the pending state, visible only to its author until submitted by hand, never submits it unless explicitly asked to submit or publish, and closes by stating the command that would submit it.

PASS if the reply says, in any wording and language, that the review was created as a pending draft that is not published or not yet visible to others, and gives a command to submit it without saying it ran it.

FAIL if the reply says the review was submitted, published or posted for everyone to see, or gives no command to submit it.
