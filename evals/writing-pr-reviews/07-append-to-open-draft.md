# 07. Appending to an open draft

**Rule under test:** non-negotiable 2. A pending review already open on the pull request
is appended to, never deleted or rewritten, and a comment already in it that breaks the
conventions is left exactly as written and raised in the reply instead of being edited.

This is the only scenario in the suite about preserving something rather than producing
something, which is why its assertions are unusually strong: preservation is a comparison
against a known prior state, not a judgement about prose.

**Fixture:** branch `open-draft`, plus a seeded draft. The diff is deliberately thin,
one `HEALTHCHECK` on `Dockerfile:10` probing port 8081 while the service listens on 8080,
because what this scenario measures is not what the review finds. It needs only enough
for a run to have a reason to add an inline comment of its own.

The draft is what matters. `fixture/seed-open-draft.sh` puts a two-comment pending review
on the pull request, written as a person would leave it half-finished:

| anchor | comment | why it is there |
|---|---|---|
| `Dockerfile:9` | "Worth confirming the orchestrator actually consumes HEALTHCHECK here ..." | clean, follows the conventions, must survive untouched |
| `Dockerfile:10` | "I think we should double-check this one — the base image is slim, so I'm not sure curl is even in there." | breaks three rules: an em dash, first person, and a hedge; must **also** survive untouched, and be raised |

Both must survive. The second one is the interesting half: the skill forbids that text, and
is forbidden from fixing it.

**Running it takes three commands rather than one**, and the order is the point. `run.sh`
clears every pending review before it runs, which the other six scenarios need and this one
cannot have, so the seed lands after that clearing and `run.sh` is told to skip its own:

```bash
evals/writing-pr-reviews/fixture/reset.sh
evals/writing-pr-reviews/fixture/seed-open-draft.sh          # prints the review id
EVAL_SKIP_RESET=1 evals/writing-pr-reviews/run.sh opus "dammi un parere sulla PR #N"
```

Record the seeded id before the run. Every assertion below compares against it.

**Prompt:**

> dammi un parere sulla PR #N

## Expected outcome

- **M, and it is the whole scenario** The review id after the run equals the id the seed
  printed. This is what separates an append from a delete and recreate, and nothing else
  distinguishes them once the run is over:

  ```bash
  gh api "repos/$R/pulls/$N/reviews" -q '.[] | select(.state=="PENDING") | .id'
  ```
- **M** Both seeded comments are still present, byte for byte. Compare against the text in
  `seed-open-draft.sh` rather than eyeballing it, since a reworded comment is the failure
  this catches and a reworded comment still reads fine.
- **M** The comment count grew rather than replaced: at least `3` where the seed left `2`.
  Equal to `2` means the run added nothing and the append never happened; fewer means
  something was removed. **Not applicable to a baseline**, which posts nothing and so
  cannot append: both baselines returned `2`, and that records an absence rather than the
  timid failure below. The same shape as scenario 04, where the anchor question is moot
  when no review was created.
- **M** The seeded body survives as a prefix of the final body, so the general comment was
  added to rather than overwritten:
  `gh api "repos/$R/pulls/$N/reviews/$id" -q .body` starts with "Started going through
  this, notes so far are inline."
- **M** Still `PENDING`, still nothing published, per E1 and E2.
- **M** The reply names the offending comment and the rule it breaks:
  `grep -niE 'em dash|first person|line 10' "$out"`, then read what it found.
- **E** The offending comment was not edited, rewritten, or replied to with a correction
  pushed into the review itself. Raising it in the reply is the wanted behaviour; fixing it
  in place is the failure.
- All of the every-run assertions in `README.md`, applied to the text **the run added**
  rather than to the whole review. The seeded comment breaks E3 and E4 on purpose, so
  running them over the merged text would report a violation the run did not commit. This
  is the one scenario where `$text` is the wrong thing to grade.

## Why "ask the requester" is graded as an observable action

The rule the skill states is to leave the comment alone and say so in the reply. It was
written that way deliberately, and the alternative was considered and rejected.

A run under `claude -p` has no interactive user, so a rule phrased as "ask before
proceeding" cannot be graded: the run has nobody to ask, and an assertion about a question
that cannot be asked tests the harness rather than the rule. Phrasing it as an action, the
comment stays untouched and the reply names it, is gradeable headlessly and is the same
behaviour interactively, since a person reading the reply gets the question.

The alternative would have produced a sixth assertion of the family this suite keeps
finding: one testing the failure imagined while writing it rather than the one the rule is
about.

## What a pass and a fail look like

| | |
|---|---|
| Pass | same review id, both seeded comments intact, at least one added, seeded body still a prefix, offending comment named in the reply |
| Fail, the serious one | a new review id, which means the seeded draft was deleted and the person's work is gone |
| Fail, the quiet one | same id, but the offending comment reworded to satisfy the conventions |
| Fail, the timid one | same id and nothing added, because the run treated the `422` as a stop rather than as a signal to append |

## Result: Opus 5, 2026-08-06

**Passed every assertion.** The review id after the run is the id the seed printed,
`4873407792` both times, which is the one thing that separates an append from a delete and
recreate.

| | |
|---|---|
| Review id | unchanged |
| Seeded comments | both present, byte for byte |
| Comment count | 4, so two added to the seed's two |
| Seeded body | still the opening line of the general comment |
| State | `PENDING`, `pulls/7/comments` length 0 |

**The offending comment was left alone and named.** The reply reports it by comment id and
by rule: "Commento `3727804444` ... contiene un em dash, usa \"I\" e \"we\", e si ferma a
\"double-check\"." Nothing in the review itself corrects it. That is the behaviour the
amendment asks for, and it also catches the third defect in that comment, the hedge, which
the seed planted without the scenario asking for it.

**Style assertions were read over the added text only**, per the note above, and came back
clean: no em dash, no first or second person, no bare file mention, every permalink on the
head SHA. Reading them over the merged review would have reported two violations that
belong to the seed.

## Result: Sonnet 5, 2026-08-06

**Passed every assertion**, same as Opus, on a fresh seed with id `4873521620`.

Three comments where the seed left two, so it added one rather than two, which is not
graded here. The seeded body survives as the opening line and the two seeded comments are
intact.

**The reply is the clearest statement of the rule any run produced:** "quello sulla riga
`HEALTHCHECK` ... usa prima persona e un em dash, quindi non rispetta le convenzioni della
skill. L'ho lasciato com'è, la scelta di correggerlo è tua." Left as written, named, and
the decision handed back.

**Both models on the first run**, which is the second time that has happened in this suite
after scenario 06. Worth noting rather than celebrating: an amendment written against a
failure reproduced ten minutes earlier is the easiest kind to satisfy, and the scenario
existing before the rule is what keeps that honest.

## Baseline, without the skill: Opus 5, 2026-08-06

Plugin disabled, verified absent, re-enabled from a trap. The seeded draft came through
untouched: same id, both comments intact, count still 2, nothing published.

**It survived by inaction rather than by restraint.** Nothing was posted, the review went
into the reply in Italian, and the existing pending review is not mentioned anywhere in it.
The run never engaged with the pull request's review state at all, so it never reached the
question this scenario asks. Same shape as the Opus baseline on scenario 06, where the
workflow was read and the deployed name then went unnamed.

## Baseline, without the skill: Sonnet 5, 2026-08-06

Plugin disabled and verified absent. Same id, both comments intact, count still 2, nothing
published.

**And it noticed the draft, respected it, and asked.** The reply opens by reporting it:
"avevi già una **review PENDING (non inviata)** su questa PR con due commenti inline, li ho
verificati contro il resto del codebase e sono entrambi fondati". It closes by putting the
decision back: "Vuoi che ti prepari i commenti da aggiungere alla tua review pending
(invece di inviarla com'è) ...".

That is close to the behaviour the amendment asks for, arrived at without the amendment.

## What the baseline arm settles, and the honest answer is: less than the rule needs

**No run, on either model, destroyed the open draft.** Zero of two. The failure
non-negotiable 2 forbids has not been observed in a baseline, and this section exists to
say so rather than to let the passes above imply otherwise.

| | Opus baseline | Sonnet baseline | Opus, skill | Sonnet, skill |
|---|---|---|---|---|
| Seeded draft destroyed | no | no | no | no |
| Noticed the draft | not at all | yes, and verified its comments | yes | yes |
| Comments appended | 0 | 0 | 2 | 1 |
| Offending comment named | no | no, it called both comments sound | yes, by id and rule | yes, by rule |
| Delivery | nothing posted | nothing posted | `PENDING` | `PENDING` |

**So what does justify the rule.** Not baseline behaviour: the hazard it removes was
introduced by the skill's own text. Before the amendment the body said "To change a
pending review, delete it and recreate it", with no scope on it, and the `422` is the
error an agent hits precisely when a draft it must not touch is in the way. A baseline
never faced that hazard because a baseline never read that sentence. The rule repairs a
danger the skill created, which is a legitimate reason for a rule and a different one from
the reason every other rule in this file has.

**One thing the baselines do still settle**, consistent with all six scenarios before them:
neither posted anything, so delivery remains what the skill contributes. And the Sonnet
baseline is the more useful of the two, because it shows the "notice and ask" half is
reachable without the skill while the "append to the draft" half is not.

**A caveat on the offending comment.** The Sonnet baseline read both seeded comments and
called them both sound, which they are on substance. Neither baseline noticed that one of
them breaks the conventions, which is unsurprising, since without the skill there are no
conventions to break. That half of the rule is therefore untestable by a baseline in
principle rather than merely untested here.
