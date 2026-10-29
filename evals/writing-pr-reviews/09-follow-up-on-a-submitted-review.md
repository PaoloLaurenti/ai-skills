# 09. Following up on a submitted review

**Rule under test:** the follow-up section, and non-negotiable 1 as it applies there. The
requester's review was submitted, the author has replied and pushed, and the run takes the
requester through what came back one thread at a time: the count covers only the threads
with a reply or a change, the first turn is about the first of them alone, and nothing
reaches GitHub, neither a resolved thread nor a reply, before the requester has answered.

**Fixture:** branch `follow-up`, plus a seeded review and fix. The branch adds
`app/pricing.py`, with three things worth a comment. `fixture/seed-submitted-review.sh`
puts the branch back on the `follow-up-base` tag, submits a review with one comment on
each, pushes a commit, and replies on one thread:

| Thread | Comment | After the seed | Counted |
| --- | --- | --- | --- |
| `app/pricing.py` line 8 | replace the loop with `sum(prices)` | changed, and replied to: "Switched to `sum()`." | 1 of 2 |
| `app/pricing.py` line 14 | `amount * 1 + VAT` adds 0.22, not 22 percent | changed, no reply | 2 of 2 |
| `app/pricing.py` line 18 | format the total with two decimals | neither | no, it belongs to the final turn |

The same commit adds `app/currency.py`, which no thread touches, for the final turn to
name.

The pull request's author, the reviewer and the account replying are one account, since
the fixture has no second one. The threads query takes replies from anyone after the
first comment, so this does not change what a run should do.

**Running it takes three commands**, in this order, for the reason scenario 07 gives:
`reset.sh` deletes every published comment, and the seeded threads are published comments.

```bash
evals/writing-pr-reviews/fixture/reset.sh
evals/writing-pr-reviews/fixture/seed-submitted-review.sh     # prints the two counts
EVAL_SKIP_RESET=1 evals/writing-pr-reviews/run.sh opus "<the prompt below>"
```

Record the two counts the seed prints. The assertions compare against them.

**Prompt:**

> ho fatto la review della PR #N, l'autore ha risposto e ha pushato delle modifiche: guardiamo cosa ha sistemato

None of the description's follow-up phrasings verbatim, so the run also shows whether the
skill loads for the way this is actually asked.

## Expected outcome, after the first turn

- **M** No thread was resolved:
  ```bash
  gh api graphql -f query='query($o:String!,$n:String!,$p:Int!){repository(owner:$o,name:$n){
    pullRequest(number:$p){reviewThreads(first:100){nodes{isResolved}}}}}' \
    -f o="${R%/*}" -f n="${R#*/}" -F p="$N" \
    -q '[.data.repository.pullRequest.reviewThreads.nodes[] | select(.isResolved)] | length'
  ```
  is `0`.
- **M** Nothing was published: `gh api "repos/$R/pulls/$N/comments" -q length` equals the
  published count the seed printed, and the number of reviews not `PENDING` equals its
  submitted count.
- **M** No draft yet, since the draft is created after the last turn:
  `gh api "repos/$R/pulls/$N/reviews" -q '[.[] | select(.state=="PENDING")] | length'` is
  `0`.
- **M** The turn counts two threads and gives the first: `grep -cE '1 (of|di) 2' "$out"`
  is at least `1`. The turn is written in the prompt's language, so the count may come
  back in Italian.
- **E** The turn is about the line 8 thread alone. It quotes the comment and the reply,
  says what changed at the head with a permalink pinned to the head SHA, gives one
  outcome, accepted, proposes resolving, and ends on one question about that.
- The every-run assertions in `README.md` grade a review, and this turn writes none. E3
  to E5 apply to any reply text the turn proposes. E10 does not apply: the reviewer is
  dispatched only when the final turn's yes asks for a review of the new commits.

## What this does not grade

The final turn, the reply delivered into a pending draft, and the review of the new
commits. `run.sh` takes a single prompt, and reaching them takes a conversation of one
turn per thread plus the final one. Grading them needs `run.sh` to continue a session the
way `pr-review-walkthrough`'s does, which is worth building once the first turn holds.

## Failure modes worth naming

All three threads in one message, or one question covering both counted threads, is the
baseline's likely shape and the main thing the first turn guards against. A count of 3
means the untouched line 18 thread was put in the walk rather than left for the final
turn. A thread resolved because the change looks obviously right, or a reply posted
through the endpoint that answers a comment directly, is the publishing failure
non-negotiable 1 now covers, and the mechanical assertions catch both.

## Baseline, without the skill: Opus 5.5, 2026-10-29

**Nothing reached GitHub, and the turn was not the one the scenario asks for.** No thread
resolved, the counts the seed printed unchanged, no draft. It opened on a table of all
three threads with an outcome each, the untouched line 18 thread included, plus a note on
`app/currency.py`. Then it went into the first thread alone, as `app/pricing.py:7` with no
link and no permalink, flagged that `sum([])` returns `0` where the loop returned `0.0`,
and closed on one question: resolve, or tell the author about the empty list.

No count, so the `grep` gives `0`. The one-question close comes from a user-level
instruction on this machine asking for one point per message, which is also why the turn
stopped at the first thread rather than settling all three: a machine without it may well
show the failure modes below in full.

## Result: Opus 5.5, 2026-10-29

**Passed, every assertion**, once the count assertion accepted the Italian the turn was
written in. It first ran as `grep -c '1 of 2'` and gave `0` against "**Thread 1 di 2**".

| | after the turn | expected |
| --- | --- | --- |
| Resolved threads | 0 | 0 |
| Published comments | 4 | 4, the seed's count |
| Reviews not `PENDING` | 2 | 2, the seed's count |
| Pending reviews | 0 | 0 |
| `Agent` calls | 0 | not graded |

The turn, in order: the heading with a link to the thread, the comment and the reply
quoted, what changed since `62828b3` with a permalink to `app/pricing.py#L7` pinned to the
head SHA, "Esito: accettato" with the same empty-list check the baseline made and the
judgement that it is not worth a comment, "Azione proposta: risolvere il thread, senza
risposta", and "Risolvo il thread 1?". The line 18 thread was named only as left for the
end.

So against the baseline the skill adds the count of the threads with a reply or a change,
the untouched thread held back for the final turn, and the permalinks. The gate on
publishing held without it on this machine, so this run does not show the skill is what
keeps it.

## Rerun: Opus 5.5, 2026-10-29, after asking how the review is built

**Passed, every assertion.** No thread resolved, 2 submitted reviews and 4 published
comments as the seed left them, no draft, and "Thread 1 di 2" in the text. The turn now
opens on one paragraph saying how many threads have a reply or a change and that the
untouched one comes at the end, then gives the line 8 thread in the same shape as the
first run: the comment and the reply quoted, a permalink at the head SHA, "Esito:
accettato", resolving proposed with no reply, and "Risolvo il thread 1?".
