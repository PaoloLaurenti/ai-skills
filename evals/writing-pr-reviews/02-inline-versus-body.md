# 02. Inline versus general comment

**Rule under test:** a finding whose file appears in the diff is anchored inline.
Only what cannot be anchored goes in the general comment, and the body contains
nothing about the diff itself.

**Fixture:** branch `inline-versus-body`. The diff edits `docker-entrypoint.sh`
and drops its `set -eu` line, which breaks a rule the fixture's own `AGENTS.md`
sets for the repo.

**Prompt:**

> dammi un parere sulla PR #N

The trap is that the finding feels like a repo-level observation. It is a breach of
a convention written in another file, so the natural shape is a body section
titled after the rule. The file carrying the breach is in the diff, so the rule
says inline, and the body should end up with nothing to say about it at all.

## Expected outcome

- **M** An inline comment exists whose `path` is `docker-entrypoint.sh`.
- **M** Its `line` falls on the hunk the change touches, so it is anchored to the
  breach rather than to the top of the file. The removed `set -eu` has no line in
  the head version, so either anchor counts: the added line on `RIGHT`, or the
  removed one with `side: LEFT`. What fails is an anchor outside the hunk.
- **M** The general comment carries no finding about `docker-entrypoint.sh`, and no
  restatement of the fix the inline comment proposes:
  `printf '%s' "$body" | grep -niE 'set -eu|restor|docker-entrypoint'`, then read the
  hits. Naming the diffed file as the *cause* of a documentation finding is allowed
  and often unavoidable, since a stale sentence has to be attributed to whatever made
  it stale. What fails is the body proposing the fix a second time.
- **E** The inline comment names the rule it breaks and links the `AGENTS.md` that
  sets it, rather than asserting the rule without a source.
- **E** The body, if it says anything at all, says it about something outside the
  diff. A body that only points at the inline comments fails: that pointer is one
  of the things the skill body bans.
- All of the every-run assertions in `README.md`.

## Result: Opus 5, 2026-08-05

**Mixed.** The rule this scenario tests held. A narrower rule, in the same section of
the skill body, did not.

What held: the `set -eu` removal was reported inline on `docker-entrypoint.sh`,
anchored inside the hunk, with a `suggestion` block restoring the line above the new
log statement. The trap worked as designed and the review did not fall into it: the
breach of a rule written in another file did not become a body section. The body's own
finding is genuinely outside the diff, `AGENTS.md#L5` for the rule and
`README.md#L5-L6` for the promise that a failed migration stops the container, and the
inline comment traces the mechanism rather than asserting it: without `set -e`, a
non-zero exit from the migration falls through to `exec "$@"` and traffic is served
against an unmigrated database. Every style assertion passed, no bare mentions, all
links pinned to the head SHA.

What did not: the body reads "Restoring the line in `docker-entrypoint.sh` is the
smaller of the two ways out", which repeats the fix the inline comment already
proposes. The skill body says the body stays silent about a follow-up an inline
comment already carries. The pivot the sentence is making is legitimate, since the
docs finding only matters if the removal is deliberate, but it can be made without
restating the fix or naming the diffed file: "either the change comes back into line
with the rule, or the rule and the promise move with it" says the same thing and
leaves the fix where it is anchored.

This is a failure of a rule the skill body already states, not a gap in it, so
nothing here calls for an amendment. What it calls for is a second data point: whether
Sonnet 5 does the same thing.

**The assertion needed narrowing, again.** As first written it read "the body does not
mention `docker-entrypoint.sh`, `grep -c` is 0". That is too strict to be right:
scenario 03 passed with a body naming a diffed file as the *cause* of a documentation
finding, which is unavoidable, since a stale sentence has to be attributed to whatever
made it stale. Naming the file is not the violation. Repeating the inline proposal is.
The corrected assertion is above.

**No payload was left in the clone this run**, where earlier runs left one. The anchor
was therefore graded from the API alone, which for a pending review answers `position`
only. It was enough here, since there is no multi-line collapse to check, and it is
the case scenario 04 flags as inconclusive when there is.

## Baseline, without the skill: Opus 5, 2026-08-06

Plugin disabled for the run, verified absent before it started, re-enabled from a trap
after. Same model, same prompt, same fixture.

**It delivered nothing to GitHub**, the same as scenario 05's baseline: `pulls/2/reviews`
empty, no comments of any kind, no payload left in the clone. The whole review went
into the reply, closing with an offer:

> Non ho pubblicato niente su GitHub. Se vuoi, posso postarlo come review formale o
> come commento inline su `docker-entrypoint.sh:2` — dimmi tu.

**Which leaves the rule this scenario tests unexercised rather than broken.** With no
review object there is no body and no inline comment, so nothing here says where the
finding would have gone. The one hint is that closing offer, and it points the right
way: asked to post, the run names an inline comment on the diffed file rather than a
review body. So the honest reading is that the placement rule is not what the skill is
carrying on this scenario. What it is carrying is everything upstream of placement,
since none of it happens on its own.

The predicted rationalisation, a body bullet framed as a repo-level concern, did not
appear in that form. What appeared instead is a document with four markdown headings,
where the breach gets its own headed section citing `AGENTS.md:5` and `README.md:5-6`
side by side, with no separation at all between what could be anchored to a line and
what could not.

| | baseline | with the skill |
|---|---|---|
| Delivery | nothing posted | one `PENDING` review, finding anchored inline |
| Em dashes | 7 | 0 |
| Permalinks | 0, all 4 file mentions bare | every mention linked at the head SHA |
| First and second person | throughout, "il mio parere", "posso", "dimmi tu" | 0 |
| Language | Italian | English |

Two further breaches with no figure to set them against, since the run with the skill
was graded on the assertions above and neither is one of them: four markdown headings
where the skill asks for bold titles, and a verdict opening the whole thing, "**Il mio
parere: da bloccare così com'è.**"

**The line numbers are right, and one of them is right in a way worth noticing.** The
`AGENTS.md:5` and `README.md:5-6` citations both resolve, and "la rimozione è `set -eu`
alla riga 2" is correct against the base version, where that line still exists. The
offered anchor, `docker-entrypoint.sh:2`, is the same number carried over to the head
version, where line 2 is blank. That is exactly the anchoring subtlety this scenario's
second assertion is about: a removed line has no line in the head file, so it needs
`side: LEFT` or the added line on `RIGHT`, and the number alone does not say which.

**The fix came as a plain fenced block, not a suggestion.** Six lines of
`docker-entrypoint.sh` reproduced under an ```` ```sh ```` fence, of which one is the
change and five are unchanged context. Committable by hand only, and it is the shape
the skill body warns about for suggestions: reproduced context reads as changed lines
to whoever looks at the rendered result. Relevant to scenario 04's baseline as much as
to this one.

**It raised a nit and priced it as low itself**, the redundant `echo "starting up"`
next to `echo "running migrations"`, calling it "Valore basso, ma innocuo". The skill
body asks for the same arithmetic and then for the nit to come out, so this is half
the rule arriving unprompted: the quantifying, not the dropping.

## Result: Sonnet 5, 2026-08-06

**Passed**, every assertion, including the narrower one the Opus run broke. This is the
second data point that run asked for, and it settles the question the other way.

| | |
|---|---|
| E1, E2 | one `PENDING` review, `pulls/2/comments` length 0 |
| The finding | inline on `docker-entrypoint.sh`, `position: 1`, inside the hunk |
| The general comment | **empty** |
| E3, E4 | no em dash, no first or second person |
| E6, E7 | no bare mentions, every link pinned to the head SHA |
| Line anchors | `AGENTS.md#L5` is the `set -eu` rule, `README.md#L5-L6` the container promise |

**The body is empty, which is the assertion Opus failed.** Opus anchored the finding
inline and then wrote a body that restated the fix. Sonnet anchored the finding inline
and left the body with nothing in it, saying so in the reply: "Non ho trovato altri
problemi al di fuori del diff, quindi il commento generale è vuoto: l'unico rilievo è
quello inline."

So the Opus result stays what its own entry called it, a rule broken rather than a rule
missing, and now with a second model declining to break it. **Nothing in the skill body
moves.** The plan entry asking for this data point can close on that.

**The two runs also disagree about where the docs citations belong**, and both readings
are defensible. Opus made `AGENTS.md#L5` and `README.md#L5-L6` a body finding in their
own right, since neither file is in the diff. Sonnet folded both into the inline
comment as the evidence for the ask. The skill body supports Sonnet here: those are not
separate findings, they are why the anchored one matters, and the rule about a body
carrying only what cannot be anchored is what decides it.

**The suggestion handles the insertion the way the skill body asks.** The removed
`set -eu` has no line in the head version, so the block anchors line 1, reproduces
`#!/bin/sh`, and adds the missing line under it. One line of context rather than five.
No payload was left in the clone again, so the anchor is graded from `position: 1` plus
the block's own content, which is enough here for the same reason the Opus run gave:
there is no multi-line collapse to check.

## Baseline, without the skill: Sonnet 5, 2026-08-06

Plugin disabled for the run, verified absent before it started, re-enabled from a trap
after.

**Nothing posted**, matching the Opus baseline on this scenario: no review, no comments,
the whole thing written into the reply in Italian. So the placement rule is unexercised
on both models, for the same reason, and this scenario's evidence for it stays what the
Opus entry above says it is.

**This one dropped line numbers entirely.** Where the Opus baseline cited `AGENTS.md:5`
and `README.md:5-6` and got both right, Sonnet cites both files with no line number and
no link, quoting the sentences instead. Four bare mentions, zero permalinks, zero
anchors of any kind. Between the two baselines the pattern is that the citation degrades
before the reasoning does: both traced the mechanism correctly, and neither made the
evidence findable.

| | Opus baseline | Sonnet baseline | with the skill |
|---|---|---|---|
| Delivery | nothing posted | nothing posted | one `PENDING` review |
| Line anchors | 3, all correct | 0 written | 2, both correct |
| Permalinks | 0 | 0 | every mention linked |
| Em dashes | 7 | 4 | 0 |
| Markdown headings | 4 | 0 | 0 |
| Verdict | "**Il mio parere: da bloccare così com'è.**" | "**Il mio parere:** da bloccare così com'è." | none |
| Language | Italian | Italian | English |

That verdict line is worth noting for arriving in nearly the same words from two
different models, and for being the phrase the skill body's ban on a verdict paragraph
is aimed at.

**The fix came as a plain fenced block again**, six lines of `docker-entrypoint.sh`
with one changed, the same shape as the Opus baseline. Two of the fences also carry
`+` and `-` markers, since the run quotes the diff back before discussing it, which is
legitimate in a chat reply and is exactly the content the skill body bars from a
`suggestion`.

**One thing neither the Opus baseline nor either skill run did:** it read the pull
request's CI status, noticed the only check present does not cover behavioural
regressions, and said so. Nothing in the scenario asks for that, and it is the third
baseline in the set to bring back something real that no assertion here grades.

## Rerun: Opus 5, 2026-08-07

Rerun against the delegation change. Held, and the placement rule held in its strongest
form yet.

| | |
|---|---|
| E1, E2 | one review `PENDING`, nothing published |
| E3, E4 | no em dash, no first or second person |
| E6, E7 | nothing unlinked, every permalink on the head SHA |
| E9 | one inline comment, 58 words |

The finding sits inline on `docker-entrypoint.sh`, anchored at line 1 on `RIGHT` inside
the hunk, with a `suggestion` block putting `set -eu` back under the shebang. Two lines
of replacement against a one-line anchor is the insertion form the skill body asks for:
anchor the single neighbouring line, reproduce just that one.

**The general comment came back empty, which is a first for this suite and worth
recording rather than scoring.** Nothing in this change reaches outside the diff, and the
body's only job is what cannot be anchored, so an empty one follows from the rule rather
than breaking it. What it costs is a reader opening the review to no framing at all.
Left as an observation: no rule in the body covers the case either way.

A reviewer subagent was dispatched, per non-negotiable 7.
