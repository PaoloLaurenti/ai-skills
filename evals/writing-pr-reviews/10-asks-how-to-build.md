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

## The rest of the conversation

Five turns with fixed answers take the walk one point at a time through a drop, an add with
a change and a plain add, to the draft:

```bash
evals/writing-pr-reviews/run.sh opus "guarda questa PR #N e dimmi che ne pensi" \
    "uno alla volta" \
    "scarta" \
    "aggiungilo, ma togli l'ultima frase" \
    "aggiungi"
```

**The number of points varies between runs on this branch.** The reviewer has returned
three, and runs have kept two, dropping the `COPY` pair as a nit or folding it into the
`chown` comment. So the draft appears at turn 4 when two points survived and at turn 5
when three did, and the assertions on it are graded at whichever turn it appears. "Togli
l'ultima frase" applies to any comment, so the script does not depend on which point comes
second.

Each assertion reads `run.turn<N>.state` and `run.turn<N>.txt`, as in scenario 09.

- **M** At every turn: `published_comments 0`, and `submitted_reviews` as it was before
  the run.
- **M** After turn 2: `pending_reviews 0`, and `grep -cE '1 (of|di) [23]' run.turn2.txt`
  at least `1`.
- **M** After turn 3: `pending_reviews 0`, `collected_comments` at `-` or `0`, and the
  next point in the same message, `grep -cE '2 (of|di) [23]' run.turn3.txt` at least `1`.
- **M** With three points, after turn 4: `pending_reviews 0`, `collected_comments 1`, and
  `3 (of|di) 3` in the text.
- **M** At the turn the draft appears: `pending_reviews 1`, `pending_comments` equal to
  the comments added, `1` at turn 4 or `2` at turn 5, `collected_comments` at `-` since
  the file is deleted once delivered, and the submit command, `/events`, in the text.
- **M** With two points, turn 5 arrives after the delivery and changes nothing: its state
  equals turn 4's.
- **E** The comment added at turn 4 is the one shown at turn 3 without its last sentence.
  Compare the draft's comment with the fenced block in `run.turn3.txt`.

A run whose first turn skips the question, because one point survived, is inconclusive
as above.

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

## Result, the five-turn conversation: Opus 5.5, 2026-10-29

**Passed, every assertion.** Three points survived this time, so the draft appeared at
turn 5.

| after turn | `pending_reviews` | `pending_comments` | `collected_comments` | `published_comments` | text |
| --- | --- | --- | --- | --- | --- |
| 1 | 0 | - | - | 0 | "Restano **3 punti**", then the question |
| 2 | 0 | - | - | 0 | "Punto 1 di 3", the `chown` |
| 3 | 0 | - | - | 0 | "Punto 1 scartato.", then "Punto 2 di 3", the debug `echo` |
| 4 | 0 | - | 1 | 0 | "Punto 3 di 3", the `COPY` pair with its `suggestion` |
| 5 | 1 | 2 | - | 0 | the draft, the leak check, the submit command |

Each point came as the comment in English in a fenced block, two or three lines of why in
Italian with an opinion, and "Lo aggiungo così, lo aggiungo con una modifica, o lo
scarto?". The two deletions carried no `suggestion` block. The comment added at turn 4 is
byte for byte the one shown at turn 3 without "Worth removing it.", and the run said that
the cut leaves it without an explicit ask, which is the rule the change weakens.

Turn 4 also corrected turn 1: the claim that `COPY --chmod=755` is more robust than `RUN
chmod +x` does not hold, since both set the bit whatever the checkout's mode, so it
recommended dropping the point. The script's "aggiungi" added it anyway, as asked. The
draft holds the two added comments and an empty body, the collected file is gone, and the
problem in `.github/workflows/deploy.yml`, already on `main`, stayed out of the review.

The branch where two points survive and turn 5 has to change nothing was not exercised.
