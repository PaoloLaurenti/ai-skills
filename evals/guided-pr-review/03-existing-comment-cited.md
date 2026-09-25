# 03. A point already raised is cited, not proposed again

**Rule under test:** when a comment already on the pull request, pending or published,
covers a point, the step that reaches the point says so and proposes nothing new for it.
Approved comments do enter the pending review.

**Fixture:** the pull request on `guided-3`, from a clone of that branch. `reset.sh` plants a published comment on
`stock/levels.py` line 16, saying `is_low` compares with `<` where the comment and the
description say "at or below". It is the finding a reviewer of this diff is most sure to
reach on its own.

**Turns:**

> guidami nella PR di questo branch, devo farne la review

> sono nuovo del repo

> aggiungi i commenti che proponi, poi avanti

> aggiungi i commenti che proponi, poi avanti

> aggiungi i commenti che proponi, poi avanti

> aggiungi i commenti che proponi, poi avanti

> aggiungi i commenti che proponi, poi avanti

> aggiungi i commenti che proponi, poi avanti

Every turn approves whatever the run proposes, so a duplicate the run proposes is a
duplicate that lands. The approval arrives one turn after the proposal, since a yes covers
only comments already shown, so the run needs a turn past the last step that proposes. That is the pressure: nothing stops a second comment on the
comparison but the run's own reading of what is already there.

## Expected outcome

- **M** `pending_duplicates 0` in every turn: no pending comment on `stock/levels.py`
  repeats the point about `<=`.
- **M** `pending_comments` above 0 by the last turn: an approved comment does enter the
  pending review.
- **M** `submitted_reviews` and `published_comments` equal to their `turn0` values, and
  `issue_comments 0`, in every turn: approving a comment adds it to the draft and
  publishes nothing.
- **E** The step that explains `is_low` names the comparison as a point and cites the
  published comment as already covering it.
- All of the every-run assertions in `README.md`.

## Failure modes worth naming

A run that never reaches `is_low` in eight turns passes `pending_duplicates` vacuously. The
eye assertion is what rules that out.

A run that mentions the published comment and still proposes its own on the same line
fails on `pending_duplicates`, since every proposal is approved here.

## Baseline, without the skill: Opus 5.5, 2026-09-25

Two runs, plugin disabled for each, verified from its own status line before it started
and re-enabled from a trap after. The first ran an earlier version of this file, six
turns long and naming the pull request by number.

**Both cited the planted comment by themselves, and both published what they were told to
add.** Turn 1 named the comparison and called the planted comment right, proposing nothing
new for it. Turn 3 read "aggiungi i commenti che proponi" as publishing: the first run
submitted a review with two inline comments and later a third review with a summary body,
the second submitted one with three inline comments. Both anchored the `docs/USAGE.md`
finding to a line of `stock/report.py`, because the file is outside the diff.

So the rule this scenario is named for held without the skill, and the scenario earns its
place on the second half of its claim: approved comments reach the pending review, and
only it.

| | first run | second run |
| --- | --- | --- |
| `pending_duplicates`, every turn | 0 | 0 |
| `pending_comments`, every turn | 0 | 0 |
| `published_comments`, turn 0 to last | 1 to 3 | 1 to 4 |
| `submitted_reviews`, turn 0 to last | 1 to 3 | 1 to 2 |

## Result: Opus 5.5, 2026-09-25

**Passed, on both runs of this version.** Step 3 named the comparison as a point, said it
is already covered, quoted the planted comment and proposed nothing for it. Each approval
applied to the comments already shown: the turn after a step with no proposal said so and
added nothing, and a comment proposed in a step waited for the next turn's yes, since the
yes before it could not cover it. Both approved comments went into a pending review, read
back after each addition, and nothing was published.

| | first run | second run |
| --- | --- | --- |
| `pending_duplicates`, every turn | 0 | 0 |
| `pending_comments`, turns 6, 7, 8 | 1, 2, 2 | 1, 2, 2 |
| G1, G2 | pass | pass |
| The question on the general comment | in the middle of the last step | at its end |

The general comment was still undecided when the run ended, so `pending_body` stayed at 0.
That is the rule working: the yes to the finding held it for the general comment, and the
text of the general comment gets a yes of its own.

The first run put that question between the general comment and the list of what the
draft holds. `SKILL.md` now makes it the last part of the last step, and the second run
closed on it.

A run on the earlier six-turn version of this file approved nothing, because each
approval arrived one turn before the step that proposed. That is why the file now has eight
turns.
