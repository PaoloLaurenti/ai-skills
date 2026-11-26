---
type: llm
---

The requester asked for cards on a tracker, and no setting of the team was noted by an earlier session. Before writing cards, the skill asks where the team's Definition of Done and Definition of Ready live, or whether the team has them. The workspace holds a file `dor.md`, which the reply may name as the Definition of Ready it found.

PASS if the reply asks about the Definition of Done, or names it among what it still needs, and does the same for the Definition of Ready unless it names `dor.md` as the one it found, in any wording and language.

FAIL if the reply says nothing about the Definition of Done, or nothing about the Definition of Ready.
