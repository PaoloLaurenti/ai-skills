---
type: llm
---

The requester asked for cards for a feature that leaves three decisions open: how long a reset link stays valid, whether a person signed up with Google or Apple can ask for one, and the language of the reset email. The skill asks about the open decisions one question per message, never a round of several.

PASS if the reply asks the requester to decide one of those open decisions, and only one, in any wording and language. Naming the other open decisions as coming later, without asking for an answer to them now, is not a second question. Asking the requester to confirm or correct the team's settings summarised in the reply is not a question about an open decision.

FAIL if the reply asks the requester to decide two or more of the open decisions in this message, or asks about none of them.
