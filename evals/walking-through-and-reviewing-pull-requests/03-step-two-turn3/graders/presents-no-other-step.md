---
type: llm
---

The requester asked for a guided walkthrough of a pull request that marks items at or below five units as low in a stock report, and said they are new to the repository. Step 1 gave a route of seven steps: 1 what the pull request is about, 2 the basics, how the repository is made, 3 the threshold and the question "is this item low?", 4 the report marking low items, 5 the test of the new threshold, 6 the documentation of the report format, 7 summary and creation of the draft review. The requester then answered "avanti", and the skill presents step 2 and stops.

PASS if the reply stays on step 2. FAIL only if it explains what the pull request changes in the concept of a later step: how `is_low` decides whether an item is low and what it compares with, how the report now marks a line, what the new test checks, or what the documentation should now say. Describing the repository as it was before the pull request, its files, the `Stock` class, the report function, how tests are run, the rules `AGENTS.md` sets, and saying which later step covers what, is step 2.
