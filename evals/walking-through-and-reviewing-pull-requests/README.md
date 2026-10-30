# Evaluations for `walking-through-and-reviewing-pull-requests`

What the skill is supposed to do, written as scenarios that can be run and graded
mostly by command. One file per scenario, numbered. This file holds what they share:
the fixture, how a run is driven, and the assertions every run gets.

A scenario is only worth trusting once it has been run **without** the skill, so the
value of the rule is visible rather than assumed.

## The fixture

One private repo, `<the authenticated login>/ai-skills-eval-guided-review`, with three
open pull requests carrying the same diff, one per scenario, on `guided-1`, `guided-2` and
`guided-3`. Three rather than one so the scenarios of an arm run side by side, since a
pending review belongs to one pull request.

The diff, `mark items at or below five units as low in the report`, has two concepts and
something to find in each:

| Where | What is there | Why it is there |
| --- | --- | --- |
| `stock/levels.py` line 16 | `is_low` compares with `<`, the comment and the description say "at or below" | The finding a reviewer is most sure to reach. A published comment already says it, so scenario 03 can see whether a run cites it or proposes it again. |
| `tests/test_levels.py` | the new test checks three units, never five | `AGENTS.md` in the fixture asks for the boundary of every threshold. An inline finding the planted comment does not cover. |
| `docs/USAGE.md` | still says nothing else appears on a report line | Outside the diff, so only the general comment can carry it. |

```bash
evals/walking-through-and-reviewing-pull-requests/fixture/setup.sh              # creates the repo and the pull requests, once
evals/walking-through-and-reviewing-pull-requests/fixture/reset.sh guided-1     # puts one pull request back, before every run
```

Both take `EVAL_FIXTURE_REPO` to point somewhere else. The owner is read from
`gh api user -q .login` at run time rather than written down.

**`reset.sh` resets one pull request, never all three**, because the other two may carry
runs still going. It forces the branch back to the `guided-base` tag, restores the
description from `fixture/pr-body.md`, deletes the pending review and every published
comment, and posts the planted comment from `fixture/review.json` again. So the planted
comment gets a new id on every run, and a scenario reads it from the API. Each reset also
leaves one more submitted review with no body behind, since GitHub never deletes a
submitted review, which is why the assertions compare `submitted_reviews` with its `turn0`
value rather than with a number.

**A run that publishes a review with a body costs the pull request its number.** GitHub
neither deletes that review nor lets its body be emptied, and the next run on the same pull
request read it as a review somebody had already written: it cited the leftover body instead
of proposing the points it covered. So `reset.sh` closes a pull request carrying one and
opens a fresh one on the same branch. That is why the prompts say "la PR di questo branch"
rather than a number, and why `run.sh` clones the scenario's branch alone: the clone of one
scenario must not see what another run pushed to its own branch meanwhile.

## Running a conversation

A guided review is a conversation, and what a scenario grades is where each turn stops
and what the pending review holds by then. So a run is a list of prompts: the first
starts the session and each later one continues it with `claude -p --continue` from the
same clone.

```bash
evals/walking-through-and-reviewing-pull-requests/arm.sh with    opus 01 02 03     # the three scenarios side by side, as installed
evals/walking-through-and-reviewing-pull-requests/arm.sh without opus 01 02 03     # the same, with the plugin disabled
```

`arm.sh` reads each scenario's turns from the blockquoted lines of its file, which quote
nothing else, and runs it on its own branch. `without` disables the plugin once around all
the runs and switches it back on from a trap. Other plugins stay enabled, so a baseline is
"this machine without this plugin" rather than a bare model. Disabling the plugin also
disables `reviewing-pull-requests`, which lives in it, so a baseline has neither skill.

For one conversation by hand, `run.sh` and `baseline.sh` take the model and the prompts
as arguments, and `EVAL_BRANCH` picks the pull request:

```bash
EVAL_BRANCH=guided-2 evals/walking-through-and-reviewing-pull-requests/run.sh opus "guidami nella PR di questo branch, devo farne la review" "sono nuovo del repo"
```

`run.sh` writes the state right after the reset to `<out>.turn0.state`, and after every
turn the stream of the turn to `<out>.turn<N>.jsonl`, every message it wrote to
`<out>.turn<N>.txt`, and the state to `<out>.turn<N>.state`. Every message rather than the
last one: the reviewer dispatched in the background finishes inside a turn and wakes the
session again, so the last message of a turn can be a line about the reviewer, with the
step itself in the message before. A state
file holds `pull_request`, `pending_review`, `pending_comments`, `pending_body`,
`pending_duplicates`, `collected_comments`, `collected_held`, `submitted_reviews`,
`published_comments`, `issue_comments`, `remote_head` and `local_dirty`.
`pending_duplicates` counts pending comments on `stock/levels.py` that mention `<=` or "at
or below", which is the point the planted comment already makes. The two `collected_`
counts read the file the skill builds the review in before creating it,
`~/.claude/reviews/OWNER-REPO-N.json`, which `reset.sh` deletes. They read 0 when the file
is not there, and a run that named the file differently looks the same, so a 0 that
matters is confirmed against the `Write` calls in the turn's `.jsonl`.

Confirm the skill loaded before reading anything into a result. A run where it never
loaded is a description failure, not a rule failure, and the two need different fixes.
A headless run's transcript sits under `~/.claude/projects/`, in a directory named after
the clone with every non-alphanumeric character turned into `-`, and macOS resolves
`/var` first:

```bash
dir="$HOME/.claude/projects/$(printf '%s' "$clone" \
    | sed 's:^/var/:/private/var/:' | tr -c '[:alnum:]' '-')"
grep -ho '"skill":"myai:[a-z-]*"' "$dir"/*.jsonl | sort | uniq -c
grep -ho '"name":"Agent","input"' "$dir"/*.jsonl | wc -l
```

The second count is the reviewer dispatch `reviewing-pull-requests` requires. Both patterns
match a tool call rather than prose.

## Assertions on every run

- **G1: nothing published.** `submitted_reviews` and `published_comments` equal their
  `turn0` values in every turn, and `issue_comments` is 0.
- **G2: no code touched.** `local_dirty 0` and `remote_head` equal to the `guided-base`
  tag in every turn. A guided review reads the change; it does not edit it.
- **G3: every proposed comment in English, with no em dash and no en dash**, whatever
  the language of the conversation. The steps are in the conversation's language.

  ```bash
  grep -c '—\|–' "$OUT".turn*.txt
  ```

  counts dashes in the steps as well, so read the matches: only those inside a proposed
  comment count against the run.
