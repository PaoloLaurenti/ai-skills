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

## Result: Sonnet 5, 2026-08-06

**Passed** every mechanical assertion, with one bare mention that should have been a
link and one paraphrase where the assertion asks for a quotation.

| | |
|---|---|
| E1, E2 | one `PENDING` review, `pulls/5/comments` length 0 |
| The finding's home | the body, and no inline comment on `README.md` |
| The stale claim | present, with the port: "still says the service listens on port 8080" |
| "the docs" | never used generically |
| E3, E4 | no em dash, no first or second person |
| E7 | every link pinned to the head SHA |
| Line anchors | `README.md#L5`, `README.md#L8-L9`, `AGENTS.md#L7`, `app/server.py#L3`, all correct |

**It found the second, unplanted problem too**, the one the Opus run surfaced: the
`README.md#L8-L9` claim that every setting has a key in `config/default.toml` does not
hold, since nothing parses that file. Three runs on this scenario have now reached it
independently, so it is the fixture rather than either model or the skill.

**Where it fell short of the Opus run, in two small ways.**

The stale claim is paraphrased rather than quoted. The assertion passes, because it
checks that the port appears, and the intent behind it is a verbatim quotation so the
author can find the sentence by searching for it. Worth reading as a near miss rather
than a pass.

And the same bare repeated mention that scenario 03 caught on this model: the inline
comment links `app/server.py` twice and then writes "Worth wiring `server.py` to
actually load this file" in plain text. One occurrence here against four there, same
rule, same direction.

**The `config/default.toml` inside the paraphrased README sentence is not a violation**,
the same exception both earlier runs' bare mentions fell under.

## Baseline, without the skill: Sonnet 5, 2026-08-06

Plugin disabled for the run, verified absent before it started, re-enabled from a trap
after. Nothing posted, the review written into the reply in Italian.

**It found the stale claim, quoted it verbatim, and cited the rule.** `README.md:5`
with "The service listens on port 8080" in quotation marks, `AGENTS.md:7` for
documentation moving with behaviour, both line numbers correct, and a concrete
proposal: add `README.md` to the diff as a fourth file. It also found the pre-existing
imprecision about `config/default.toml` being the single source of configuration, and
was careful to label it as pre-existing rather than caused by this change.

**So the cross-check rule holds without the skill on both models**, which is what the
Opus baseline above predicted might turn out to be true and what this run confirms.
Four runs on this scenario, two with the skill and two without, and all four reached
the planted finding. On this fixture the documentation cross-check is not the thing
that gets skipped, and one run is no longer the basis for saying so.

The caveat the Opus entry raised still applies and is worth repeating rather than
assuming remembered: the fixture's `README.md` is nine lines long in a seven-file repo,
and the change touches three sites carrying the same number. A cross-check is easy here
in a way it is not in a real repository, so this says the rule is not carrying weight
*on this fixture*, not that it carries none.

| | Opus baseline | Sonnet baseline | with the skill |
|---|---|---|---|
| Delivery | nothing posted | nothing posted | one `PENDING` review, both models |
| The planted finding | found | found | found |
| Em dashes | 6 | 5 | 0 |
| Markdown headings | 4 | 0 | 0 |
| Permalinks | 0, 15 bare mentions | 0, 9 bare mentions | every mention linked |
| Verdict | "Verdetto: chiederei una modifica" | "**Verdetto:** richiedi modifiche" | none |
| Language | Italian | Italian | English |

Both baselines closed on a verdict line opening with the same word, which is now the
third pair of runs to produce nearly identical banned phrasing across two models.
Whatever the ban on a verdict paragraph is fighting, it is not a quirk of one model.

## Rerun: Opus 5, 2026-08-07

Rerun against the delegation change. Every assertion this scenario exists for held, and
the new non-negotiable did not.

| | |
|---|---|
| E1, E2 | one review `PENDING`, nothing published |
| E3, E4 | no em dash, no first or second person |
| E6, E7 | nothing unlinked, every permalink on the head SHA |
| E9 | one inline comment, 57 words |
| M | `README.md` named in the body, the stale `8080` quoted, no inline comment on it, no generic "the docs" |

**It loaded `superpowers:requesting-code-review` and then reviewed the diff itself.** The
skill was invoked, `code-reviewer.md` was read, and the next thing in the transcript is
"Analysis is done. Writing the review now." followed by the payload. No subagent was
dispatched at any point.

One of two runs in this rerun to skip the dispatch, out of seven. Worth noting which two:
03 and 05 are the scenarios whose findings sit largely outside the diff, in documentation,
which is the material the reviewer subagent is least equipped to bring back. Whether that
is the cause or a coincidence of two runs is not settled here.

## Rerun: Opus 5, 2026-08-07, after the dispatch amendment

**E10 is `1`, on the second of the two scenarios that failed it hours earlier.** With 03,
that is both runs the amendment was written for, corrected.

Everything else held: `README.md` named twice in the body and the stale `8080` quoted
twice, no inline comment on it, no generic "the docs", one inline comment at 49 words, no
em dash, no first or second person, every permalink on the head SHA.

E6 reported `config/default.toml`. It sits inside a sentence quoted verbatim from
`README.md`, which is the "literal syntax quoted from a file" exception the skill body
lists, and linking inside quoted material would corrupt the thing being quoted.
