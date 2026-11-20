---
type: llm
---

The requester asked for a guided walkthrough of a pull request that marks items at or below five units as low in a stock report, and said they are new to the repository. Step 1 gave a route of seven steps: 1 what the pull request is about, 2 the basics, how the repository is made, 3 the threshold and the question "is this item low?", 4 the report marking low items, 5 the test of the new threshold, 6 the documentation of the report format, 7 summary and creation of the draft review. The requester then answered "avanti", and the skill presents step 2 and stops.

PASS if the reply is step 2 of that route, the basics of how the repository is made, explained for someone new to the repository, and ends by inviting the requester to go on and naming step 3, in any wording and language. FAIL otherwise.
