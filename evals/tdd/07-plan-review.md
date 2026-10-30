# 7. The plan review closing a cycle

**Rule under test:** "Review the plan". After the cycle on `totals_the_prices_of_all_items`,
`totals_the_price_of_a_single_item` checks a behaviour the new test includes. The review
proposes deleting it, naming the test that covers it, and stops, with no pause after
every cycle chosen: the stop comes from the plan-change rule alone.

**Fixture:** `halfway`.

```bash
evals/tdd/run.sh halfway opus \
    "continuiamo in TDD il carrello, il piano è nel file di test" \
    "no, fermati solo dove serve" \
    "ok"
```

## Expected outcome

- **E** Turn 1 asks only about a pause after every cycle, since the request named TDD.
- **M** After turn 2, the resumed plan's stop: `src_changed` and `tests_changed` are `0`,
  `passed` is `1`.
- **E** Turn 2 shows each entry's state, done or pending, and asks for the go.
- **M** After turn 3: `totals_the_price_of_a_single_item` still in `tests`; `passed` is
  `2`, `failed` is `0`, `ignored` is `1`. The ignored test is what proves the stop:
  skipping the review would have moved on to the empty cart.
- **E** Turn 3 proposes deleting `totals_the_price_of_a_single_item`, names
  `totals_the_prices_of_all_items` as covering it, and closes on a question.

## Result: Opus 5.5, 2026-10-30

**Passed.** Turn 1 asked only about the pause. Turn 2 left the crate untouched,
`passed 1`, and showed the plan with each entry's state:

> 1. ✅ `totals_the_price_of_a_single_item`: fatto e verde.
> 2. ⏳ `totals_the_prices_of_all_items`: da fare. [...]
> 3. ⏳ `an_empty_cart_totals_zero`: da fare.

Turn 3: `passed 2`, `failed 0`, `ignored 1`, the single-item test still there. The
review:

> ora `totals_the_price_of_a_single_item` è ridondante. [...] `totals_the_prices_of_all_items`
> verifica lo stesso comportamento. [...] Elimino `totals_the_price_of_a_single_item`, o
> preferisci tenerlo?

It also reported that the sum already covers the pending empty-cart test, as the green
rule asks for an implementation broader than the test in hand.

## Baseline: Opus 5.5, 2026-10-30

**Failed.** Turn 1 asked nothing and ran the cycle on `totals_the_prices_of_all_items`
straight away, with an expected value computed from Arrange rather than written:

> Il totale atteso è `book_price + pen_price`, calcolato dai valori dichiarati all'inizio
> del test.

No plan review: `totals_the_price_of_a_single_item` was never questioned. Turn 2's "no,
fermati solo dove serve" was read as a go, and the empty-cart test went through too,
`ignored 0`. It loaded `superpowers:test-driven-development`.
