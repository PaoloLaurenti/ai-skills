# 04. Suggestion blocks

**Rule under test:** a fix that is a concrete edit to diffed lines is handed over as
a committable `suggestion` block whose anchor covers every replaced line, and a
pure deletion gets no block at all.

**Fixture:** branch `suggestions`. The diff carries both shapes on purpose:

- a `COPY` of the entrypoint followed by a `RUN chmod +x` in a separate layer,
  three lines that collapse into one `COPY --chmod=755`, so the anchor has to span
  all three while the replacement is a single line;
- a leftover `echo "DEBUG: entrypoint starting"` in `docker-entrypoint.sh`, which
  should simply go, so there is no replacement text to hand over.

**Prompt:**

> guarda questa PR #N e dimmi che ne pensi

## Expected outcome

- **M** The comment on the `COPY` lines contains a fenced `suggestion` block:
  `printf '%s' "$inline" | grep -c '```suggestion'` is at least `1`.
- **M** The anchor spans the replaced lines, so it carries `start_line` as well as
  `line` and the two cover every line the block replaces. **Grade this from the
  payload, not from the API.** A pending review's comments read back with `position`
  and `original_position` only, and `line`, `start_line` and `side` all `null`, so the
  endpoint cannot answer this question at all:

  ```bash
  cat "$clone"/review*.json | grep -A4 '"path"'   # the anchor as sent
  ```

  The skill posts from a JSON file, so the file is in the clone the run happened in.
  Where a run leaves none, the anchor is ungradeable and the scenario is inconclusive
  rather than passed.
- **M** The comment on the debug line carries no `suggestion` block. A block whose
  content is empty, or one reproducing the neighbouring lines to express a
  deletion, both fail.
- **M** No `+` or `-` markers inside any block. Read the block contents rather than
  a window after the fence, since a fixed `grep -A` window spills into the prose
  below and reports lines that are not in any block:
  `gh api "repos/$R/pulls/$N/reviews/$id/comments" -q '.[].body' | awk '/^```suggestion/{f=1;next} /^```$/{f=0} f' | grep -cE '^[+-]'`
  is `0`.
- **E** Indentation inside the block matches the file exactly, since the block is a
  literal replacement.
- **E** The block contains only the final code, with the reasoning in the prose
  around it.
- All of the every-run assertions in `README.md`.

## Result: Opus 5, 2026-08-05

**Passed.** Ran twice, identical both times, and the second run existed only because
the first was graded wrong. The record of that mistake is the most useful thing on
this page.

What the payload sent:

| anchor | block |
|---|---|
| `Dockerfile` `start_line: 5` `line: 7`, both `RIGHT` | the collapsed `COPY --chmod=755 --chown=1000:1000` |
| `Dockerfile` `line: 9` | that line plus `USER 1000:1000` |
| `docker-entrypoint.sh` `line: 4` | none |

Lines 5 to 7 are the three being replaced, so the anchor covers every one of them.
The insertion anchored the single neighbouring line and reproduced only that line.
The pure deletion got no block at all, prose only. Every style assertion passed,
`PENDING` held, nothing leaked, and the permalinks carried the head SHA.

**How the first grading got it wrong, since the same trap is waiting for the next
scenario.** Reading the comments back from `reviews/$id/comments` shows
`position: 7` with `line`, `start_line` and `side` all `null`. That reads exactly
like a single-line anchor, and it is not: for a **pending** review the API populates
`position` and `original_position` only, whatever the payload contained. The
conclusion drawn from it, that the suggestion would replace one line of three, was
wrong, and it went as far as an amendment to the skill body answering a failure that
never happened. What settled it was reading the `review.json` the run left in the
clone, which is the skill body's own rule about verifying a claim at its source
rather than inferring it from something that looks conclusive.

**One real finding, outside the assertions.** The submit command reported back was
`repos/OWNER/REPO/pulls/4/events`, which is not an endpoint: submitting needs
`pulls/4/reviews/REVIEW_ID/events`. Scenario 01 reported the correct form, so this is
inconsistent rather than always wrong, and the skill body stated the create and the
delete commands while leaving the submit one to be reconstructed. That part of the
amendment stands, because that failure did happen.

## The assertion that catches the subtle failure

A single-line anchor on a three-line collapse is the failure this scenario is for.
GitHub accepts it, the comment renders, and the suggestion silently proposes
replacing one line out of three, leaving the other two behind. Nothing about the
rendered result looks wrong, so the `start_line` and `line` check is what separates
a working suggestion from one that quietly breaks the build when committed.

## Baseline, without the skill: Opus 5, 2026-08-06

Plugin disabled for the run, verified absent before it started, re-enabled from a trap
after. Same model, same prompt, same fixture. Nothing was posted: `pulls/4/reviews`
empty, no comments, no payload in the clone, the review written into the reply and
offered at the end, "Non ho postato nulla su GitHub: questa è solo la mia lettura.
Dimmi se vuoi che la lasci come review sulla PR."

**No `suggestion` block anywhere, and the anchor question is moot rather than failed.**
With no review object there is nothing anchored to anything, so the assertion this
scenario exists for, whether `start_line` and `line` span all three replaced lines,
cannot be graded at all. That is worth stating as an absence rather than reading
anything into it, for the reason recorded above this section: an inference about a
suggestion's anchor is what cost this repo a withdrawn amendment.

**The replacement text is there, correct, and not committable.** Three fenced blocks
carry code, none of them a suggestion:

| the finding it sits under | what the block holds |
|---|---|
| the debug line | `+echo "DEBUG: entrypoint starting"`, the diff quoted back |
| the inert `chown` | `+RUN chown 1000:1000 docker-entrypoint.sh`, the diff quoted back |
| the three-line collapse | `COPY --chown=1000:1000 --chmod=755 docker-entrypoint.sh ./` |

So the prediction holds in a sharper form than it was written in. The fix is not
merely described: the exact replacement line is handed over, verbatim and right. What
is missing is the mechanism, the anchor spanning lines 5 to 7 of `Dockerfile` and the
`suggestion` fence that makes the line one click to commit. Two of the three blocks
are the diff quoted back with its `+` markers, which is what a plain fence is for and
is exactly the content that must never end up inside a suggestion.

**The deletion got prose only**, which is what the rule asks for, arrived at for the
wrong reason. `docker-entrypoint.sh:4` is cited, correctly, and the line is called
residual debug output that should not reach production. No block, no reproduced
neighbours, so the shape the prediction watched for did not appear. With nothing
committable anywhere in the review, though, this says nothing about whether a deletion
would have been treated differently from a replacement.

| | baseline | with the skill |
|---|---|---|
| Delivery | nothing posted | one `PENDING` review |
| `suggestion` blocks | 0 | 2, plus none on the deletion |
| Anchor spanning the collapse | no anchor exists | `start_line: 5`, `line: 7` |
| Em dashes | 7 | 0 |
| Permalinks | 0, all 7 file mentions bare | every mention linked at the head SHA |
| First and second person | throughout, "la mia lettura", "se scegliete", "Dimmi se vuoi" | 0 |
| Language | Italian | English |

Also present and banned, with no recorded figure to set them against: four markdown
headings, and a verdict in the opening line, "Verdetto: **chiederei modifiche**".

**The substance was strong, and none of it is what this scenario grades.** Beyond the
two findings the fixture plants, it worked out that `main` already carried
`COPY --chmod=755` on that very line, so the pull request dismantles a working one-line
idiom to reach the same mode in three instructions and three layers, and that the
`chown` is inert while the mode is `755` and no `USER` directive exists. It then priced
the docs question correctly: `AGENTS.md` is not breached yet, because nothing
observable changes, and would be if the non-root direction were taken. Whether the run
with the skill said any of this is not answerable from the record above, which is a
summary rather than a transcript. What the baselines do settle, across all five, is
that finding real problems is not what the skill is contributing.
