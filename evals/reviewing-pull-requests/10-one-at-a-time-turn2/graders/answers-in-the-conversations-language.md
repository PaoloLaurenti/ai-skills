---
type: llm
---

The requester writes in Italian. The skill writes the review itself in English, and every message to the requester in the conversation's language, turn after turn.

PASS if the reply's own sentences, those addressed to the requester, are in Italian. English inside quoted review comments, replies or descriptions, in a comment or reply shown for GitHub, in code, commands, file names or thread titles, is not a reason to fail.

FAIL if the reply's own sentences addressed to the requester are in English or another language than Italian.
