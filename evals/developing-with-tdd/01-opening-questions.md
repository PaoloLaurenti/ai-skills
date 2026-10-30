# 1. The two opening questions

**Rule under test:** "Before the first test". A request to change behaviour that names no
TDD gets the question whether to proceed with TDD at all, and nothing is written before
it is answered. After a yes, the question on a pause after every cycle comes in a turn of
its own. It also tells whether the description fires on an Italian request with no
Italian phrasing in it, which decides whether those phrasings come back.

**Fixture:** `feature`.

```bash
evals/developing-with-tdd/run.sh feature opus \
    "implementa lo sconto con codice promozionale sul carrello descritto in FEATURE.md" \
    "sì"
```

## Expected outcome

- **M** The skill loaded: the transcript count in `README.md` is at least `1`.
- **M** Nothing written after turn 1: `src_changed`, `tests_changed` and `other_changed`
  are `0` in `run.turn1.state`.
- **M** Nothing written after turn 2: the same in `run.turn2.state`.
- **E** Turn 1 closes on the one question whether to proceed with TDD, and asks nothing
  about pauses.
- **E** Turn 2 closes on the one question whether to pause after every cycle.

A run where the skill did not load fails the description, not the rule: rerun with the
Italian phrasings put back before reading the rule's result.

## Result: Opus 5.5, 2026-10-30

**Passed.** The skill loaded on the Italian prompt with English phrasings only in the
description, which settled whether the Italian ones were needed. Both turns left the
crate untouched: `src_changed`, `tests_changed` and `other_changed` at `0`.

Turn 1 closed on the TDD question alone:

> La proposta è di procedere in TDD con `myai:tdd`, un test alla volta attraverso la
> superficie pubblica del carrello. [...] Procedo in TDD?

Turn 2 asked about the pause in a turn of its own:

> Ora la seconda domanda, che va in un turno a parte. [...] Vuoi che mi fermi alla fine
> di ogni ciclo?

## Baseline: Opus 5.5, 2026-10-30

**Failed.** Without the skill the run asked nothing and built the whole feature in turn 1:
`src_changed 4`, `tests_changed 1`, `passed 7`. It loaded
`superpowers:test-driven-development`, which `skillOverrides` marks `off`, and said of its
own red:

> ho seguito la TDD, ma solo il primo test e quello sulla scadenza li ho visti fallire
> prima di implementare. Gli altri tre [...] sono passati appena scritti

## Result, after the rename to `developing-with-tdd`: Opus 5.5, 2026-10-30

**Passed.** The skill loaded under its new name on the same Italian prompt. Turn 1 closed
on `Vuoi che lo sviluppi in TDD?`, turn 2 on `Vuoi che mi fermi dopo ogni ciclo?`, and
the crate stayed untouched after both.
