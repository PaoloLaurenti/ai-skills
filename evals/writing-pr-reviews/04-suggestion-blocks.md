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

## Baseline, without the skill

Not yet recorded. Predictions: the fix described in prose rather than handed over as
a block, or a block with a single-line anchor. For the deletion, a block containing
the surrounding lines minus the deleted one is the shape to watch for.
