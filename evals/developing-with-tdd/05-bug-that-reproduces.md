# 5. A bug that reproduces

**Rule under test:** "A bug". The plan opens with a regression test named after the
correct behaviour, red shows the reported behaviour, then the fix.

**Fixture:** `implemented`, whose expiry check rejects a code on its expiry date.

```bash
evals/developing-with-tdd/run.sh implemented opus \
    "c'è un bug: un codice promozionale viene rifiutato come scaduto proprio nel giorno della sua scadenza, ma dovrebbe valere fino a quel giorno compreso. Sistemalo in TDD, fermati solo dove serve" \
    "ok"
```

## Expected outcome

- **M** After turn 1: `src_changed` is `0`, the plan's stop.
- **M** After turn 2: `failed` is `0`, `passed` is `7`, and `src_changed` is `1`.
- **M** No test name holds `bug`, `issue`, `fix` or `regression` as a word:
  `tr ' ' '\n' < "$OUT".turn2.state | grep -ciE '(^|_)(bug|issue|fix|regression)(_|$)'`
  is `0`. Matched as a word, since a helper such as `stub_fixed_at` holds `fix`.
- **E** The red shown in turn 2 has `Err(ExpiredCode)` as the actual value.

## Result: Opus 5.5, 2026-10-30

**Passed.** Turn 1: `src_changed 0`, one pending test, the plan's stop. Turn 2:
`passed 7`, `failed 0`, `src_changed 1`, the regression test named
`accepts_a_code_on_its_expiry_date`. The red in the transcript:

> left: Err(ExpiredCode)
> right: Ok(4500)

The final reply did not quote the red, only that the test went from red to green. It
closed by proposing, not doing, a structural refactoring:

> aggiungere `PromoCode::has_expired_on(today: Date) -> bool` [...] Faccio questo
> refactoring o lo lasciamo così?

## Baseline: Opus 5.5, 2026-10-30

**Failed on the plan's stop.** Turn 1 wrote the test and fixed the code at once,
`src_changed 1`, with no plan put to the requester. The test name and the red were
right:

> Prima della correzione il test falliva per il motivo giusto: restituiva
> `Err(ExpiredCode)` invece di `Ok(4500)`.

Turn 2's "ok" answered its offer to commit, and it committed on a branch it created.
It loaded `superpowers:test-driven-development`.
