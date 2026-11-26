---
type: llm
focus: trace
---

The trace is a run of an agent asked for cards on a tracker. An earlier session noted the team's settings in a file: no Definition of Done, the Definition of Ready in `dor.md`, cards in English, and "To refine" as the status of a card not ready. The skill opens by summarising those settings to the requester, so they can correct what changed.

PASS if a message the agent wrote to the requester, not only a file it read, restates those settings, all four, in any wording and language.

FAIL if no message to the requester restates them, or one of the four is missing from it.
