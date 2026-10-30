# 6. The skill does not fire

**Rule under test:** the description's "Not for refactoring that keeps behaviour". A
rename changes no behaviour, so no TDD question and no plan.

**Fixture:** `implemented`.

```bash
evals/developing-with-tdd/run.sh implemented opus "rinomina total in total_price nel carrello"
```

## Expected outcome, after the turn

- **M** The skill did not load: the transcript count in `README.md` is `0`.
- **M** The reply never mentions TDD: `grep -ci tdd "$OUT".turn1.txt` is `0`.
- **M** The rename is done: `grep -c 'fn total_price' "$CRATE"/src/cart.rs` is `1`.
- **M** Every test still passes: `failed` is `0`, `passed` is `6`.

## Result: Opus 5.5, 2026-10-30

**Passed.** No `myai:tdd` load in the transcript, no mention of TDD in the reply,
`fn total_price` in `src/cart.rs`, `passed 6`, `failed 0`. The callers in `src/promo.rs`
and `tests/cart.rs` were updated with it.

## Baseline: Opus 5.5, 2026-10-30

**Passed, as with the skill.** Same rename, same callers updated, `passed 6`. The
scenario guards the description rather than a rule.
