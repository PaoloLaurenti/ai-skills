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

## Result: Opus 5, 2026-08-05

**Passed**, every assertion, and it found a second documentation problem the fixture
had not planted.

| | |
|---|---|
| The finding's home | the body, and no inline comment on `README.md` |
| The stale claim | quoted, not paraphrased: "The service listens on port 8080" |
| The replacement | given: "The service listens on port 3000." |
| The file | named and linked, and "the docs" appears nowhere |
| Section title | **Docs to refresh**, in bold rather than as a heading |

Five line anchors, all correct at the head SHA: `README.md#L5` is the port sentence,
`README.md#L8-L9` the configuration paragraph, `AGENTS.md#L7` the rule about
documentation moving with behaviour, `config/default.toml#L2` the `port = 3000` key,
and `Dockerfile#L9` the `EXPOSE 3000` line.

**The unplanned finding.** Beyond the contradiction the fixture builds in, it
reported that `README.md#L8-L9`, "There are no environment variables: every setting
has a key in that file", has become *misleading* rather than false: it reads as
though the TOML were the port's only home, while the port is also a literal in
`app/server.py`. That is the rule about reporting wording that has merely become
misleading, applied without being prompted by a contradiction.

**The bare mention is correct.** Stripping the links leaves one `Dockerfile`, in the
phrase "a Dockerfile instruction having no way to read the file". That names the
format rather than the repo's file, so a permalink would be wrong. Same shape as
scenario 03's bare mention: what survives the strip has to be read, not counted.

**Praise landed on the line that earns it**, not in the body: the inline comment on
`app/server.py` opens by noting that all three copies of the number moved together in
one change. The skill body asks for exactly that placement, and the body carries no
verdict.

## What a baseline is expected to show here

This is the scenario where the absence of the rule should be most visible, because
the documentation sits outside the diff and nothing in the diff points at it. A run
without the skill has no reason to go looking, so the likely result is a competent
review of the port change that never mentions `README.md` at all. If the baseline
finds it anyway, that is worth knowing: it would mean the rule is carrying less
weight than the skill body assumes, and the body should say so.

## Baseline, without the skill

Not yet recorded.
