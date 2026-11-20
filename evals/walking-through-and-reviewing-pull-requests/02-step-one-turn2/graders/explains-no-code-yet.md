---
type: llm
---

The requester asked for a guided walkthrough of a pull request, was asked for a level, and answered "sono nuovo del repo", new to the repository. The pull request marks items at or below five units as low in a stock report, touching `stock/levels.py`, `stock/report.py` and `tests/test_levels.py`, and carries one published comment on `stock/levels.py` saying `is_low` compares with `<` where "at or below" is meant. The skill now presents step 1 and stops. Step 1 holds, in its own words and in any order: a header numbered `1/TOT` with a title; why the pull request exists; the files it touches with the size of the change; what is already known, which includes the review so far, the published comment on `stock/levels.py`; the route, every step numbered with its title; and an invitation to go on naming the next step.

PASS if the reply does not walk through how the changed code works, line by line, as a later step would. Saying in a line what each touched file adds belongs to step 1. FAIL if it does.
