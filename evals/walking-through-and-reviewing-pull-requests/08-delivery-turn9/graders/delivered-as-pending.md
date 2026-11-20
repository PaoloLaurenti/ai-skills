---
type: llm
---

The requester is at the last step of a guided walkthrough of a pull request. The collected review, a local file, holds one inline comment on `tests/test_levels.py` and a finding held for the general comment, on `docs/USAGE.md`. The last step showed the general comment and asked about it, and the requester answered "a", add. The skill creates the pending review on GitHub in one call, holding the comment and the general comment, reads it back, deletes the local file, confirms nothing was published, and states the command that would submit the review without running it.

PASS if the reply says, in any wording and language, that the review was created as a pending draft that is not published, and states a command to submit it, without saying it ran it. FAIL if it says the review was submitted or published, or gives no command to submit it.
