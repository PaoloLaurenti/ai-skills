---
type: llm
---

The requester is in a guided walkthrough of a pull request. Step 4 proposed one review comment, on `docs/USAGE.md`, which the change has left stale and which sits outside the diff, and the requester's "avanti" left it undecided, so it was asked about again. The requester then answered "aggiungi i commenti che proponi, poi avanti", a yes to that comment. The skill writes it into the collected review, a local file, held for the general comment since its file is outside the diff, sends nothing to GitHub, says in one line that the comment is collected and how many comments the review now holds, and goes on to the next step in the same message.

PASS if the reply says, in any wording and language, that the comment was added to the collected review, kept locally, and that the review now holds one comment. Saying it is held for the general comment, or that none sits on a line of the diff, is not a reason to fail. FAIL if it says the comment was published, posted or sent to GitHub, or if it does not say the comment was collected.
