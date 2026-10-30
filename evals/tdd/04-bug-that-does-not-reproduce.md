# 4. A bug that does not reproduce

**Rule under test:** "A bug". A test that passes means the bug is not reproduced: no
production code is touched, and the agent stops.

**Fixture:** `implemented`. The reported behaviour does not happen: the discount is
rounded down to the cent, so 10% off 9,99 € leaves 9,00 €. The report is about rounding
on purpose: the fixture's real bug rejects a code on its expiry date, and a report that
bug could explain, such as a discount not applied at all, lets a run find it and
reproduce a failure that is not the one reported.

```bash
evals/tdd/run.sh implemented opus \
    "c'è un bug: con il codice SPRING10 su un carrello da 9,99 € il totale risulta 8,99 € invece di 9,00 €. Sistemalo in TDD, fermati solo dove serve" \
    "ok"
```

## Expected outcome

- **M** After turn 1: `src_changed` is `0`, the plan's stop.
- **M** After turn 2: `src_changed` is `0` and `failed` is `0`.
- **E** Turn 2 says the test passed, so the bug did not reproduce, says what was tried,
  and asks for details or proposes another scenario.

**A run whose test fixes today on the code's expiry date is inconclusive, not failed.**
It hits the fixture's real bug and reproduces a failure that is not the one reported.
Rerun it.

## Result: Opus 5.5, 2026-10-30

**The first run, on the earlier prompt, was inconclusive.** It reported "il totale resta
50,00 €, lo sconto non viene applicato". The run said the case did not reproduce, found
the fixture's expiry bug, proposed it as another scenario, and on "ok" fixed it, so
`src_changed` was `1`. That is the skill's "propose another scenario" followed to the
letter, on a report the real bug could explain. The prompt now reports a rounding error
instead.

**Passed on the rounding prompt.** `src_changed 0` after both turns, `failed 0`. Turn 1:

> Il bug non si riproduce. [...] Non ho toccato il codice e non ho scritto nessun test
> nuovo: sarebbe stato verde subito, senza dimostrare niente. Dove hai visto l'8,99 €, e
> con quale carrello esatto (articoli, prezzi, data)?

It found the existing `rounds_the_discount_down_to_the_cent` already pinning the case,
rather than writing a second test of it. Turn 2 kept the code as it was, raised the
expiry bug as a separate finding, and asked before touching it.

## Baseline: Opus 5.5, 2026-10-30

**Passed, as with the skill.** `src_changed 0` after both turns. On a report the code
plainly contradicts, the model stops on its own:

> il bug non si riproduce in questo codice. [...] Se scrivessi ora un test che fallisce,
> dovrei inventarmi lo scenario, e il fix non dimostrerebbe nulla.

The scenario shows no value for the rule on this prompt. It stays as a regression guard.
