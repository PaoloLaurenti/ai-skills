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

## Baseline, without the skill: Opus 5, 2026-08-05

Plugin disabled for the run, verified absent before it started, re-enabled from a trap
after. Same model, same prompt, same fixture.

**It delivered nothing to GitHub.** No review, pending or otherwise, and no comments:
`pulls/5/reviews` came back empty. It wrote the whole thing into the reply and closed
with "Il parere è qui in chat, non ho pubblicato nulla su GitHub. Se vuoi che lo posti
come review sulla PR, dimmelo." Every delivery rule in the skill body is therefore
carrying its full weight: the pending review, the inline anchoring, the submit command
reported back. None of it happens on its own.

**Every style rule broke, and the numbers are the point:**

| | baseline | with the skill |
|---|---|---|
| Em dashes | 6 | 0 |
| `###` headings | 4 | 0, bold titles instead |
| Permalinks | 0, all 15 file mentions bare | every mention linked at the head SHA |
| Verdict paragraph | "Verdetto: chiederei una modifica" | none |
| Preamble describing the change | "Cosa fa: sposta la porta..." | none |
| First and second person | 5 | 0 |
| Language | Italian | English |

**And the rule the fixture was built to test held without the skill.** It found the
stale `README.md:5` claim on its own, quoted it verbatim, cited `AGENTS.md:7` for why
the edit belongs in the same change, and proposed the one-line fix. It even verified
the line was still there on the branch rather than assuming it.

That is the outcome this scenario file predicted might happen, and it says something
the skill body currently does not: the documentation cross-check is not, on this
model and this fixture, the thing that would be skipped. What is worth weighing before
concluding anything from one run: the fixture makes the finding prominent, since the
`README.md` is nine lines long and the change touches three sites that all repeat the
same number. A cross-check is easy here in a way it is not in a large repo. So the
honest reading is that this run does not license dropping the rule, and it does move
the rule down the list of what earns the skill its place.

**One thing the baseline found that the run with the skill also found**, independently:
that nothing reads `config/default.toml`, so the README's claim about configuration was
already imprecise before this change. Both arrived there, which makes it a property of
the fixture rather than of either configuration.
