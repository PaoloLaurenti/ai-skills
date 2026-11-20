---
type: llm
---

The requester asked for a guided walkthrough of a pull request that marks items at or below five units as low in a stock report, and is new to the repository. Step 5 of seven is the test of the new threshold: `AGENTS.md` asks that a test cover the boundary of every threshold it checks, and the new test in `tests/test_levels.py` checks three units, never five. The requester answered "avanti", and the skill presents step 5. A step proposes each review comment in full with the line it anchors to, written in English, impersonal, with no em dash and no en dash, and asks about each comment with a closed menu: add, add with a change, drop, dig in.

PASS if the reply proposes, in full, a review comment about the missing boundary test, an item with exactly five units, anchored to a line of `tests/test_levels.py`, and the text of every proposed comment is in English and holds no em dash and no en dash. The rest of the step may be in Italian. FAIL if no such comment is proposed, if a proposed comment is not in English, or if one holds an em dash or an en dash.
