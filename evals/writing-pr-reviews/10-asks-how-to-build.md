# 10. Asking how the review is built

**Rule under test:** the section "Choosing how the review is built". A request naming
no approach, on a change with several findings, gets one question once the reviewer
has returned: create the pending draft now with every point, or go through them one at
a time first. The question names how many points survived and the weightiest of them,
and nothing reaches GitHub before it is answered.

**Fixture:** branch `suggestions`, the one scenario 04 uses, with the normal reset. Its
diff carries three things worth a comment: a `chown 1000:1000` with no `USER` to give it
effect, a `COPY` and a `RUN chmod +x` that collapse into one `COPY --chmod=755`, and a
leftover debug `echo`. The 2026-10-29 run of scenario 04 found all three.

**Prompt:**

> guarda questa PR #N e dimmi che ne pensi

Scenario 04's prompt before it gained ", con tutti i punti in una volta", so it names no
approach.

## Expected outcome, after the first turn

- **M** No draft yet:
  `gh api "repos/$R/pulls/$N/reviews" -q '[.[] | select(.state=="PENDING")] | length'` is
  `0`.
- **M** Nothing published: `gh api "repos/$R/pulls/$N/comments" -q length` is `0`.
- **M** The reviewer ran before the question: E10 in `README.md`, at least `1`.
- **M** The turn offers walking the points one at a time:
  `grep -ciE '(uno|una) alla volta|one at a time' "$out"` is at least `1`.
- **E** One message, giving the number of points and naming the weightiest, closing on
  the one question between the two approaches. No comment text yet, since neither
  approach has been chosen.
- The every-run assertions in `README.md` grade a review, and this turn writes none. E10
  is the only one that applies.

**A run whose reviewer leaves a single point is inconclusive, not failed.** The skill
skips the question then and creates the draft, as it should, so the run says nothing
about the question. Reset and run it again.

## Failure modes worth naming

A draft created at once is the shape seven of the eight scenario runs on 2026-10-29
took when nothing asked for an approach. Walking the first point straight away, with no
choice offered, is the shape the run of scenario 04 took that day, following a
user-level instruction to raise one point per message. A question asked before the
reviewer returned cannot name the count, and E10 at `0` catches it.

## Result: Opus 5.5, 2026-10-29

**Passed, every assertion.**

| | after the turn | expected |
| --- | --- | --- |
| Pending reviews | 0 | 0 |
| Published comments | 0 | 0 |
| `Agent` calls | 1 | at least 1 |
| `grep` for one at a time | 1 | at least 1 |

The reviewer returned three points and the run kept two: the `chown 1000:1000` with no
`USER`, named as the weightiest, and the debug `echo`. It dropped the `COPY` and `RUN chmod
+x` pair as a nit, quantified at a couple of layers for a script of a few hundred bytes,
and said so. It checked `README.md` and `AGENTS.md`, then closed on "Vuoi che crei subito
la review in stato pending con entrambi i commenti, o preferisci vederli uno alla volta
prima che la crei?"

The two points came as Italian summaries rather than as comment text, which fits: the
question needs to say what is at stake, and the comments are written once an approach is
chosen.

Dropping the `COPY` pair matters beyond this scenario, since scenario 04 expects a
`suggestion` block on exactly those lines.
