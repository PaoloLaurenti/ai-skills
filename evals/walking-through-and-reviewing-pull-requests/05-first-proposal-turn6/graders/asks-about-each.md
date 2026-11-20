---
type: llm
---

The requester asked for a guided walkthrough of a pull request that marks items at or below five units as low in a stock report, and is new to the repository. Step 5 of seven is the test of the new threshold: `AGENTS.md` asks that a test cover the boundary of every threshold it checks, and the new test in `tests/test_levels.py` checks three units, never five. The requester answered "avanti", and the skill presents step 5. A step proposes each review comment in full with the line it anchors to, written in English, impersonal, with no em dash and no en dash, and asks about each comment with a closed menu: add, add with a change, drop, dig in.

PASS if every proposed comment comes with a closed question offering at least to add it, add it with a change, or drop it, and the reply ends on those questions or on the invitation that follows them. A single menu per comment is enough, whatever its wording and language. FAIL if a proposed comment has no question of its own, or if the reply says a comment was already added.
