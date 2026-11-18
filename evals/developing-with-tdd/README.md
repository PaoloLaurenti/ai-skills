# Evaluations for `developing-with-tdd`

What the skill is supposed to do, written as cases that `claude plugin eval` runs and
grades. Each case directory holds a `case.yaml` with the prompt, a `scaffold.sh` building
the workspace, and a `graders/` directory, one file per check, most of them read from
what Claude did rather than judged.

A case covers one rule an agent under pressure would negotiate with. A first-turn case
also runs without the skill, the baseline arm, so the value of the rule is visible
rather than assumed: the report shows both scores and their difference, Δ. Only the arm
with the skill decides whether a case passes.

## The cases

| Case | Rule under test | Fixture state |
| --- | --- | --- |
| `01-opening-questions-turn1` | before any code, ask only whether to use TDD | `feature` |
| `01-opening-questions-turn2` | after "sì", ask only whether to pause at every cycle | `feature` |
| `02-plan-and-its-stop-turn1` | state the surface, write the plan as pending tests in order, and stop for the go | `feature` |
| `03-one-cycle-at-a-time-turn1` | with a pause at every cycle, write the plan and stop for the go | `feature` |
| `03-one-cycle-at-a-time-turn2` | after "ok, procedi", one cycle, red on an assertion, then pause | `feature` and its plan |
| `04-bug-that-does-not-reproduce-turn1` | a reported bug that no test reproduces is not fixed | `implemented` |
| `05-bug-that-reproduces-turn1` | a reported bug gets a regression test planned, then a stop | `implemented` |
| `05-bug-that-reproduces-turn2` | after "ok", the red shows the bug and the fix turns it green | `implemented` and its plan |
| `06-skill-does-not-fire-turn1` | a rename that keeps behaviour does not load the skill | `implemented` |
| `07-plan-review-turn1` | resuming a plan, ask only whether to pause at every cycle | `halfway` |
| `07-plan-review-turn2` | after "no", show the plan and stop | `halfway` |
| `07-plan-review-turn3` | after "ok", the plan review closing a cycle proposes deleting the redundant test | `halfway` |

Every case also checks that nothing was committed, since no prompt asks for it.

**When `skill-loaded` fails, the other graders say nothing about the rule.** A run where
the skill never loaded is a description failure, and the fix belongs in the description,
not in the body. The description carries English phrasings only, and `01`'s first turn
is the evidence that it fires on an Italian request; if that grader fails, put the
Italian phrasings back and rerun before reading anything else.

**`04` and `06` pass without the skill too**, so their Δ near 0 is expected. They stay
as guards: `04` of the rule on bugs, which a change to the skill could break, and `06` of
the description, which must not fire on a refactoring.

## The fixture

A small Rust crate, `shop`, built fresh for every run by `fixture/setup.sh` in the run's
empty workspace, with a git repo of its own whose only commit is the fixture. Nothing
reaches GitHub. Rust because every clone of this repo already has the toolchain pinned
in `.tool-versions`, and because `cargo test` counts passed, failed and ignored tests, so
the state of a plan of pending tests is read from its output.

`base/` is a cart whose total is tested by two passing tests. On top of it a state adds:

| State | What it holds |
| --- | --- |
| `feature` | `FEATURE.md` describing promo codes, which the crate does not have yet |
| `implemented` | promo codes implemented and tested through a catalogue and a clock, both doubles; one bug no test covers: a code is rejected on its expiry date, which `FEATURE.md` says is still valid |
| `halfway` | a plan halfway through: `totals_the_price_of_a_single_item` green, the total faked as the first item's price, two tests pending |

A case resuming after a turn that wrote files keeps them in its own `crate/`, which
`setup.sh` copies over the state before the fixture commit.

## Running

```bash
tools/eval.sh developing-with-tdd
```

The defaults are Opus, three runs per arm, Sonnet as judge, the threshold of 1.0 and a
ceiling of 16 dollars. A full run took 32 minutes and cost $14.18 in estimated list
prices, counted against the plan's usage rather than billed, unless the organisation has
extra usage on. The report lands in `evals/results/<timestamp>/report.html`, which git
ignores.

While a case is being written, run it alone and cheap:

```bash
tools/eval.sh developing-with-tdd --model haiku --runs 1 --case 05-bug-that-reproduces-turn1
```

`--case` takes a glob without character classes, so `0[345]-*` matches nothing: run one
case per invocation instead.

**An edit to `SKILL.md` reruns the suite before it is offered**, and re-records the
histories below when the edit changes what a first turn says.

## Later-turn cases

A case named `-turn2` or `-turn3` resumes a recorded conversation, `history.jsonl`, and
answers it with a short prompt such as "ok". It runs with the skill only, and its graders
see only the new turn.

The history is a recording of the skill as it was. After a change to `SKILL.md` it no
longer shows what the skill would say, so record it again:

1. Run the earlier turn's case with `--keep-temp --ablation none`, which keeps the
   run's trace.
2. Rebuild the history from that trace:

   ```bash
   tools/history-from-trace.sh <trace.jsonl> "<the earlier turn's prompt>" > history.jsonl
   ```

   For a third turn, pass the second turn's `history.jsonl` as a third argument and the
   new turn is appended to it.
3. Copy into the case's `crate/` the files the recorded turn wrote, if any.
4. Run `tools/check.sh`, which proves no machine-local path or name survived.

Why histories are rebuilt this way, and what else a case author needs to know about the
command, such as the line on shell loops every `case.yaml` repeats and how to replay a
judge, is in [`evals/README.md`](../README.md).

## Before `claude plugin eval`

The scenarios used to be hand-run files graded by command, with `run.sh` and
`baseline.sh`. Their results of 2026-10-30, with and without the skill, are in git
history at commit `80769fb`, the last one holding them.
