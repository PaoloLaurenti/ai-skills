# Evaluations for `developing-with-tdd`

What the skill is supposed to do, written as scenarios that can be run and graded mostly
by command. One file per scenario, numbered. This file holds what they share: the
fixture, how a run is driven, and how a run is graded.

A scenario covers one rule an agent under pressure would negotiate with, and it is only
worth trusting once it has been run **without** the skill too, so the value of the rule
is visible rather than assumed. `baseline.sh` is how.

## The fixture

A small Rust crate, `shop`, built fresh for every run by `fixture/setup.sh` in a
temporary directory, with a git repo of its own whose only commit is the fixture. Nothing
reaches GitHub. Rust because every clone of this repo already has the toolchain pinned
in `.tool-versions`, and because `cargo test` counts passed, failed and ignored tests, so
the state of a plan of pending tests is read by command.

`base/` is a cart whose total is tested by two passing tests. On top of it a state adds:

| State | What it holds | Used by |
| --- | --- | --- |
| `feature` | `FEATURE.md` describing promo codes, which the crate does not have yet | 1, 2, 3 |
| `implemented` | promo codes implemented and tested through a catalogue and a clock, both doubles; one bug no test covers: a code is rejected on its expiry date, which `FEATURE.md` says is still valid | 4, 5, 6 |
| `halfway` | a plan halfway through: `totals_the_price_of_a_single_item` green, the total faked as the first item's price, two tests pending | 7 |

The crate's `Cargo.lock` is committed and `target/` ignored, so `git status` after a turn
shows only what the run changed.

## Running a conversation

```bash
evals/developing-with-tdd/run.sh      <state> opus "<first prompt>" ["<next prompt>"...]
evals/developing-with-tdd/baseline.sh <state> opus "<first prompt>" ["<next prompt>"...]
```

The first prompt starts the session and each later one continues it with `claude -p
--continue` from inside the crate. After every turn `run.sh` writes the reply to
`<out>.turn<N>.txt` and the crate's state to `<out>.turn<N>.state`, and it prints `<out>`
and the crate's path at the end. A state file holds `src_changed`, `tests_changed`,
`other_changed` (files changed or added since the fixture commit), `build`, `passed`,
`failed`, `ignored`, and `tests`, the names of the functions marked `#[test]` under
`tests/`, helpers left out.

`baseline.sh` takes the same arguments and runs `run.sh` with the plugin disabled,
re-enabling it from a trap. **Never run a baseline while a run with the skill is going**:
disabling the plugin is machine-wide, so the other run would lose the skill halfway.

**The recorded baselines loaded another TDD skill**, `superpowers:test-driven-development`
or `mattpocock-skills:tdd`, which the settings then marked `off` to no effect. Both are
now denied by permission rules, so a baseline run from here on works with no TDD skill
at all, and may come out worse than the recorded ones.

The tool list allows editing files, `cargo`, and `git` in full: whether a run commits on
its own is one of the things graded, so the tool list must not prevent it.

## Grading

Set `OUT` to the prefix `run.sh` printed and `CRATE` to the crate's path.

**Whether the skill loaded is only in the transcript.** A headless run's transcript sits
under `~/.claude/projects/`, in a directory named after the crate with every
non-alphanumeric character turned into `-`, and macOS resolves `/var` first:

```bash
dir="$HOME/.claude/projects/$(printf '%s' "$CRATE" \
    | sed 's:^/var/:/private/var/:' | tr -c '[:alnum:]' '-')"
grep -ho '"name":"Skill","input":{"skill":"myai:developing-with-tdd"' "$dir"/*.jsonl | wc -l
```

Confirm it before reading anything into a result. A run where the skill never loaded is
a description failure, not a rule failure, and the two need different fixes. Scenario 6
is the one where it must not load.

### Assertions on every run

- **T1: no commit.** `git -C "$CRATE" rev-list --count HEAD` is `1`. No prompt asks for
  one.
- **T2: tests in English**, whatever the language of the conversation. Read by eye.

M marks an assertion graded by command, E one graded by judgement.
