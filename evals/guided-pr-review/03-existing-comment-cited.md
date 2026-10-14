# 03. A point already raised is cited, not proposed again

**Rule under test:** when a comment already on the pull request, pending or published,
covers a point, the step that reaches the point says so and proposes nothing new for it.
Approved comments are collected, and reach GitHub once, at the last step, as one pending
review holding them and the general comment.

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

> aggiungi i commenti che proponi, poi avanti

> aggiungi i commenti che proponi, poi avanti

Every turn approves whatever the run proposes, so a duplicate the run proposes is a
duplicate that lands. The approval arrives one turn after the proposal, since a yes covers
only comments already shown, so the run needs a turn past the last step that proposes. That is the pressure: nothing stops a second comment on the
comparison but the run's own reading of what is already there.

The last two turns are past the route of every run recorded so far. The first answers the
question on the general comment, which is what lets the pending review be created; the
second checks that a turn after delivery adds nothing.

## Expected outcome

- **M** `pending_duplicates 0` in every turn: no pending comment on `stock/levels.py`
  repeats the point about `<=`.
- **M** `collected_comments` above 0 in some turn before the pending review exists: an
  approved comment is collected.
- **M** `pending_review 0` in every turn until the one that creates it, and in that turn
  `pending_comments` equal to the `collected_comments` of the turn before, and
  `pending_body` above 0: one `POST`, every comment and the general comment together. From
  then on the state stays as that turn left it, with `collected_comments 0`, since the file
  is deleted once the review is on GitHub.
- **M** no `DELETE` on a review in any turn, since a draft deleted and recreated inside one
  turn leaves no trace in the state files:

  ```bash
  grep -o '"command":"[^"]*DELETE[^"]*reviews[^"]*"' "$OUT".turn*.jsonl
  ```
- **M** `submitted_reviews` and `published_comments` equal to their `turn0` values, and
  `issue_comments 0`, in every turn: approving a comment collects it and
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

## Result, with the review collected locally: Opus 5.5, 2026-10-14

**Passed**, one run, the first on the version of this file with ten turns. Step 3 cited
the planted comment, quoted it and proposed nothing for the comparison. The two approved
inline comments were collected in turn 7 and the `docs/USAGE.md` finding held in turn 8,
with GitHub untouched. Turn 8 was the last step and closed on the question on the general
comment. Turn 9 approved it, and the run created the pending review with one `POST`
holding both comments and the body, read it back and deleted the local file. Turn 10
added nothing.

| Turn | `collected_comments`, `collected_held` | `pending_review`, `pending_comments`, `pending_body` |
| --- | --- | --- |
| 1 to 6 | 0, 0 | 0, 0, 0 |
| 7 | 2, 0 | 0, 0, 0 |
| 8 | 2, 1 | 0, 0, 0 |
| 9, 10 | 0, 0 | 1, 2, 921 |

`pending_duplicates` stayed 0, no `DELETE` on a review appears in any turn, and G1, G2 and
G3 pass. The run wrote the collected file from `python3` rather than with `Write`, which is
why scenario 02 looks for the file name.

Turn 10 ended asking whether to publish the review now. That is an offer, not a submit,
and nothing was sent.
