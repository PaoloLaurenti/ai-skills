# 05. Docs cross-check

**Rule under test:** every review checks the diff against the documentation the repo
keeps about itself, names the file rather than saying "the docs", and quotes the
claim the change has made stale. The skill's own text calls this step mandatory and
easy to skip, which is what makes it the scenario a baseline exposes most clearly.

**Fixture:** branch `docs-cross-check`. The diff moves the listening port from 8080
to 3000 in `Dockerfile` and `config/default.toml`. The fixture's `README.md` states
that the service listens on 8080 and is **not** in the diff, so nothing in the
change points at it.

**Prompt:**

> dammi un parere sulla PR #N

## Expected outcome

- **M** The general comment names `README.md`:
  `printf '%s' "$body" | grep -c 'README\.md'` is at least `1`.
- **M** It quotes the stale claim rather than paraphrasing it, so the port appears:
  `printf '%s' "$body" | grep -c '8080'` is at least `1`.
- **M** The mention is a permalink pinned to the head SHA, per E7.
- **M** The finding is in the body rather than inline, because `README.md` is not in
  the diff: `gh api "repos/$R/pulls/$N/reviews/$id/comments" -q '[.[] | select(.path == "README.md")] | length'`
  is `0`.
- **M** The word "docs" is not used generically in place of a filename:
  `printf '%s' "$body" | grep -niE 'the docs\b'` finds nothing.
- **E** The finding says what the sentence should say now, not only that it is
  stale.
- **E** The fixture's `AGENTS.md` rule that documentation moves in the same change
  as the behaviour is either cited or satisfied by the proposal.
- All of the every-run assertions in `README.md`.

## What a baseline is expected to show here

This is the scenario where the absence of the rule should be most visible, because
the documentation sits outside the diff and nothing in the diff points at it. A run
without the skill has no reason to go looking, so the likely result is a competent
review of the port change that never mentions `README.md` at all. If the baseline
finds it anyway, that is worth knowing: it would mean the rule is carrying less
weight than the skill body assumes, and the body should say so.

## Baseline, without the skill

Not yet recorded.
