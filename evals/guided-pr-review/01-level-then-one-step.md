# 01. The level first, then one step per turn

**Rule under test:** a guided review asks how much to explain before explaining anything,
then presents one step, numbered `N/TOT`, and stops. Nothing goes into a review while
the steps carry no approved comment.

**Fixture:** the pull request on `guided-1`, from a clone of that branch.

**Turns:**

> guidami nella PR di questo branch, devo farne la review

> sono nuovo del repo

> avanti

The first prompt is short on purpose. A request that spells out the procedure ("one step
at a time, number them, ask me first") would be followed with or without the skill, so it
would measure the prompt rather than the rule. The second answers a level menu if one was
asked, and reads as plain context if not.

## Expected outcome

- **M** `pending_comments 0`, `submitted_reviews` and `published_comments` equal to their
  `turn0` values, `issue_comments 0`, in every turn: nothing was added or published.
- **E** Turn 1 asks for the level with a closed menu, and explains nothing of the diff
  yet.
- **E** Turn 2 is step 1 only: why the pull request exists, the files it touches, the
  state of the review, and the list of every step with its number. It ends by inviting
  "avanti".
- **E** Turn 3 is step 2 only, with its header numbered against the same total.
- All of the every-run assertions in `README.md`.

## Failure modes worth naming

A turn 1 that explains the whole pull request in one message and closes with a verdict
passes every mechanical assertion and fails the rule. That is what the eye assertions are
for.

A turn 1 that asks the level and also presents step 1 in the same message fails the
first eye assertion: the level decides how step 1 is written.

## Baseline, without the skill: Opus 5.5, 2026-09-25

Two runs, plugin disabled for each, verified from its own status line before it started
and re-enabled from a trap after. The first ran an earlier version of this file whose
first prompt named the pull request by number. The turns are otherwise the same.

**Both explained the whole pull request in turn 1 and never asked for a level.** Turn 1
walked all three files, listed the findings and closed on "Verdetto consigliato: Request
changes". Turn 2 answered "sono nuovo del repo" with a tour of the repository. Turn 3 went
two different ways: the first run opened "Passo 1 di 4" and quizzed the requester
("Rispondimi con cosa noti su tutte e tre"), the second drafted the review comments.
Every mechanical assertion passed, since nothing was added or published, and every eye
assertion failed.

| | first run | second run |
| --- | --- | --- |
| Level asked before explaining | no | no |
| Turn 1 | the whole pull request, a verdict | the whole pull request, a verdict |
| Turn 3 | step 1 of a new plan, as questions | the drafted comments |
| `pending_comments`, `published_comments`, `submitted_reviews` | unchanged | unchanged |

## Result: Opus 5.5, 2026-09-25

**Passed, on both runs.** Turn 1 read the pull request, dispatched the reviewer in the
background, and asked for the level with three lettered options, explaining nothing of the
diff. Turn 2 was step 1 of 7 alone, with the route of every step, and closed on "avanti".
Turn 3 was the step of basics, numbered 2/7. Nothing was added, published or edited.

| | first run | second run |
| --- | --- | --- |
| Level asked, alone, in turn 1 | yes | yes |
| Turn 2, turn 3 | 1/7, 2/7 | 1/7, 2/7 |
| G1, G2 | pass | pass |

In the second run the reviewer stopped on an API error inside turn 1, and the session
dispatched a second one, which is what the skill says to do with a report that is lost.
Before `run.sh` kept every message of a turn, the first run's `turn1.txt` held only the
last one, "Aspetto ancora che tu scelga il livello", with the menu itself in the message
before.
