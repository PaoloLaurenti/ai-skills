---
type: llm
---

The requester asked for cards on a tracker, and no setting of the team was noted by an earlier session. Before writing cards, the skill asks once for the team's settings it lacks, among them the status a card goes in while it is not ready.

PASS if the reply asks which status of the tracker holds cards that are not ready yet, or a card still to refine, or names it among what it still needs from the requester, in any wording and language. The skill may ask one question per message and list the rest.

FAIL if the reply does not ask about that status.
