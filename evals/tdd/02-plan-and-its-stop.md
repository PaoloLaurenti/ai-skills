# 2. The plan and its stop

**Rule under test:** "The plan". With both opening questions answered by the request, the
first turn writes the surface and the plan of pending tests, and stops for the go. No
production code, no test body.

**Fixture:** `feature`. Its two cart tests pass before the run.

```bash
evals/tdd/run.sh feature opus \
    "implementa in TDD lo sconto con codice promozionale descritto in FEATURE.md, senza pausa a ogni ciclo: fermati solo dove serve"
```

## Expected outcome, after the turn

- **M** No production code: `src_changed` is `0`.
- **M** The plan is pending tests: `passed` is `2`, `failed` is `0`, `ignored` is at least
  `4`, one per rule in `FEATURE.md`.
- **M** The build still works: `build` is `ok`.
- **E** The surface is stated: the entry point, the catalogue and the clock as
  dependencies to double, the reactions.
- **E** Names state behaviour in domain words, with no type, method or concrete value.
- **E** Order: happy paths simplest first, then errors (unknown, expired), then
  validation and edge cases; an expiry-day case, if planned, is an edge case.
- **E** The turn closes on a question asking for the go.

## Result: Opus 5.5, 2026-10-30

**Passed.** `src_changed 0`, `passed 2`, `failed 0`, `ignored 5`, `build ok`. The plan:
`applies_the_promo_code_discount_to_the_total`, `rounds_the_discount_down_to_the_whole_cent`,
`rejects_an_unknown_promo_code`, `rejects_an_expired_promo_code`,
`accepts_a_promo_code_on_its_expiry_date`. The surface named the entry point, a
`PromoCodeCatalogue` trait doubled by `PromoCodeCatalogueDouble::stub_with`, and today's
date passed in so Arrange fixes it. It left out the empty cart with a code, and said why:

> la stessa formula dà zero, quindi non guiderebbe nessuna scelta di design, e la suite
> esistente copre già il totale del carrello vuoto.

It closed on `Approvi interfaccia e piano così, e parto con il primo ciclo?`

## Baseline: Opus 5.5, 2026-10-30

**Failed on the plan's form, passed on the stop.** It stopped before the first red,
`src_changed 0`, but wrote no pending test: `tests_changed 0`, `ignored 0`. The order of
cycles lived in the reply only, so a new session would not find it:

> **Ordine dei cicli:** sconto base (1500 → 1350), poi arrotondamento (1999 → 1800),
> codice valido il giorno della scadenza, codice scaduto il giorno dopo, codice
> sconosciuto.

It loaded `mattpocock-skills:tdd`, which `skillOverrides` marks `off`.
