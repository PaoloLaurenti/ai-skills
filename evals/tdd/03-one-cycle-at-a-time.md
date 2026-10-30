# 3. One cycle at a time, nothing anticipated

**Rule under test:** "The cycle", red and green. After the plan's go, one cycle turns one
pending test green with the least code, and every other pending test stays pending.
With a pause after every cycle chosen, the turn stops after that one cycle.

**Fixture:** `feature`.

```bash
evals/tdd/run.sh feature opus \
    "implementa in TDD lo sconto con codice promozionale descritto in FEATURE.md, con una pausa a ogni ciclo" \
    "ok, procedi"
```

## Expected outcome

- **M** After turn 1: `src_changed` is `0` and `ignored` is the plan's size, call it `P`.
- **M** After turn 2: `passed` is `3`, `failed` is `0`, `ignored` is `P - 1`.
- **E** The transcript of turn 2 shows a red on an assertion before any green, not a
  panic or a "not implemented".
- **E** If the green took an obvious implementation, the reply says so and names the
  pending tests it was checked against.
- **E** Turn 2 closes on the pause after the cycle.

## Result: Opus 5.5, 2026-10-30

**Passed.** Turn 1: `src_changed 0`, `ignored 6`, so `P` is 6. Turn 2: `passed 3`,
`failed 0`, `ignored 5`. The red went through compilation first, then the assertion, and
the green was a Fake It chosen by the transformation list:

> Il primo rosso è stato un errore di compilazione. [...] Il test è poi fallito
> sull'asserzione: `left: Ok(0)`, `right: Ok(900)`.
>
> **Green.** Il metodo restituisce la costante `Ok(900)`, senza logica vera. È la
> trasformazione più alta che bastava, da nil a costante.

The plan review proposed changing the data of a pending test so the next green has to
read the percentage, and asked before doing it. The turn closed on the pause:
`Vado avanti col secondo ciclo, rounds_the_discount_down_to_the_whole_cent, usando il 15%?`

## Baseline: Opus 5.5, 2026-10-30

**Failed.** Turn 1 ran a whole cycle before any plan was approved, `src_changed 3`, with
no pending test in the file, and opened on an error case rather than the simplest happy
path:

> Il ciclo 1 è chiuso: test rosso, poi verde, e nessun refactor necessario. [...] **Il
> comportamento del ciclo:** un codice che il negozio non conosce viene rifiutato come
> sconosciuto.

The red itself was sound, on the assertion: `left: Ok(1200)`, `right: Err(Unknown)`.
It loaded `superpowers:test-driven-development`.
