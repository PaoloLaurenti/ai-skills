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
- **M** That comment's anchor spans the replaced lines, so it carries `start_line`
  as well as `line`, and `line - start_line` covers every line the block replaces:
  `gh api "repos/$R/pulls/$N/reviews/$id/comments" -q '.[] | "\(.path) \(.start_line) \(.line) \(.position)"'`.
- **M** The anchor is expressed as `line`, not as `position`. A comment posted with
  `position` comes back with `line`, `start_line` and `side` all `null`, and
  `position` addresses a place in the diff rather than a range of lines, so it cannot
  span the three being replaced. Scenario 01 showed a run taking that route for a
  single-line comment, where it is harmless. Here it is the failure itself, and the
  `null` is what to look for.
- **M** The comment on the debug line carries no `suggestion` block. A block whose
  content is empty, or one reproducing the neighbouring lines to express a
  deletion, both fail.
- **M** No `+` or `-` markers inside any block:
  `printf '%s' "$inline" | grep -A5 '```suggestion' | grep -cE '^[+-]'` is `0`.
- **E** Indentation inside the block matches the file exactly, since the block is a
  literal replacement.
- **E** The block contains only the final code, with the reasoning in the prose
  around it.
- All of the every-run assertions in `README.md`.

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
