# Evaluations for `walking-through-and-reviewing-pull-requests`

What the skill is supposed to do, written as cases that `claude plugin eval` runs and
grades. Each case directory holds a `case.yaml` with the prompt, a `scaffold.sh` building
the workspace, and a `graders/` directory, one file per check, most of them read from
what Claude did rather than judged.

The skill drives a pull request through `gh`, which the sandbox cannot reach. Every run
talks to a stand-in instead, `tools/fake-gh`, which answers from the recorded pull request
in the workspace's `.git/fake-gh/pull-request.json`, keeps what a run changes, and logs
every call in `.git/fake-gh/calls.jsonl` for the graders to read. A call it does not know
is logged as `"unhandled"`, and every case requires none.

## The cases

The turns of one guided review, the first prompted with "guidami nella PR di questo
branch, devo farne la review" and each later one resuming the recorded conversation. The
trunk runs to the first step that proposes a comment, where two branches part: one says
"avanti", the other says yes and goes on to the delivery.

| Case | Prompt | Rule under test |
| --- | --- | --- |
| `01-level-first-turn1` | the opening request | the reviewer dispatched in the background, and the level asked as a closed menu with nothing of the diff explained |
| `02-step-one-turn2` | "sono nuovo del repo" | step `1/7` and only that: every part of step 1, no proposal, no code explained yet, no second reviewer |
| `03-step-two-turn3` | "avanti" | step `2/7` and only that, explained rather than quizzed |
| `04-existing-comment-cited-turn4` | "avanti" | step `3/7` reaches `is_low`, cites the published comment on `<` against "at or below", and proposes no duplicate |
| `05-first-proposal-turn6` | "avanti" | step `5/7` proposes the boundary test on `tests/test_levels.py` in full, with a closed menu, nothing collected |
| `06-avanti-approves-nothing-turn7` | "avanti" | a word that moves on approves nothing: no step 6, nothing written, the comment asked about again |
| `07-yes-collects-turn7` | "aggiungi i commenti che proponi, poi avanti" | the comment written into the collected review, read back, said to be collected, and the next step in the same message, on a conversation of its own, below |
| `08-delivery-turn9` | "a" | one `POST` creating the pending review with the comment and the general comment, nothing submitted, the local file deleted |

Every case also checks that no call was unhandled, and each turn before the last that
nothing was published, committed or pushed. The collected review sits in the run's home,
outside the workspace a file grader can read, so `07` reads it from the trace; `08` reads
the review from the stand-in's own state, a comment tied to a review still `PENDING`
whose body carries the finding held on `docs/USAGE.md`.

**`01` is the only case with a baseline arm**, since a resumed case runs with the skill
only. Its Δ is the evidence the opening's shape comes from the skill.

## The fixture

A private repo of the owner's, `ai-skills-eval-guided-review`, holding a small stock
report in Python. The pull request, `mark items at or below five units as low in the
report`, has two concepts and something to find in each:

| Where | What is there | Why it is there |
| --- | --- | --- |
| `stock/levels.py` line 16 | `is_low` compares with `<`, the comment and the description say "at or below" | The finding a reviewer is most sure to reach. A published comment already says it, so `04` sees whether a run cites it or proposes it again. |
| `tests/test_levels.py` | the new test checks three units, never five | `AGENTS.md` in the fixture asks for the boundary of every threshold. An inline finding the planted comment does not cover. |
| `docs/USAGE.md` | still says nothing else appears on a report line | Outside the diff, so only the general comment can carry it. |

`fixture/files.sh` writes the two commits, `fixture/setup.sh` pushes them to GitHub and
opens the pull requests once, and `fixture/reset.sh` puts one back.
`fixture/photograph.sh` resets the one on `guided-1` and photographs it into
`fixture/pull-request.json`, with `tools/fake-gh/photograph.sh`, which also keeps what
`gh pr view` answers with every field and as text, for the stand-in to give back. Run it
again only with the histories below in mind.

`fixture/workspace.sh` builds every case's workspace from those files, with no network:
the repo on `guided-1` with fixed commit dates, so a commit id stays the same from run to
run; `origin.git`, a bare repo inside the workspace standing for GitHub; and the
photographed pull request with this repo's own head.

## Running

```bash
tools/eval.sh walking-through-and-reviewing-pull-requests
```

The defaults are those of `tools/eval.sh`: Opus, three runs per arm, Sonnet as judge, the
threshold of 1.0 and a ceiling of 16 dollars. While a case is being written, run it alone
and cheap:

```bash
tools/eval.sh walking-through-and-reviewing-pull-requests --model haiku --runs 1 --case 03-step-two-turn3
```

**An edit to `SKILL.md` reruns the suite before it is offered**, and records the histories
again when the edit changes what an earlier turn says.

## Later-turn cases

`02` to `08` resume a conversation recorded with Opus, as `evals/README.md` describes:
each turn recorded from a run of the case before it with `--keep-temp --ablation none`,
its trace turned into `history.jsonl` by `tools/history-from-trace.sh`, the earlier
history passed as its third argument. Turns 5 and 8 fall on no case, and were recorded
through a case written for each and deleted once run.

Three things tie a history to the state it was recorded in:

- **The reviewer's report** reaches the conversation as a notification, which the rebuilt
  history keeps, so no resumed case dispatches the reviewer again, and each checks it.
- **A file a turn wrote in the run's home** goes in the case's directory, for its
  scaffold to copy: `08-delivery-turn9/collected/` is the collected review as turn 8 left
  it, which the scaffold puts in `~/.local/state/reviews/`. A history names the run's
  home as `~`, since that is the resumed run's own home.
- **The skill as it was.** A history carries the `SKILL.md` its first turn loaded, so an
  edit reaches a resumed case only once its turns are recorded again. After the read-back
  after every addition was strengthened, on 2026-11-20, turns 1 to 6 were recorded again
  for `07` alone, and that conversation went its own way: six steps rather than seven, the
  first proposal at step 4, on `docs/USAGE.md`, left undecided by an "avanti" and asked
  again. So `07` collects that finding under `held` and moves on to step `5/6`, while
  `02` to `06` and `08` keep the conversation recorded before the edit, which the
  strengthened sentence does not touch.
- **The comment ids** in the histories are those of `fixture/pull-request.json`.
  `photograph.sh` reposts the planted comment under a new id, so running it again means
  recording every history again.

## Results

Opus 5.5 with Sonnet as judge, three runs per arm, CLI 2.1.328, and 2.1.329 for `07`,
2026-11-19 and 2026-11-20, each case from its own run once its graders were settled.

| Case | With the skill | Without | Cost |
| --- | --- | --- | --- |
| `01-level-first-turn1` | 0.96 | 0.56 | $1.90 |
| `02-step-one-turn2` | 1.00 | | $0.35 |
| `03-step-two-turn3` | 1.00 | | $1.04 |
| `04-existing-comment-cited-turn4` | 0.93 | | $1.02 |
| `05-first-proposal-turn6` | 1.00 | | $1.09 |
| `06-avanti-approves-nothing-turn7` | 1.00 | | $0.97 |
| `07-yes-collects-turn7` | 1.00 | | $1.32 |
| `08-delivery-turn9` | 1.00 | | $1.89 |

`01` and `04` fell short on graders since corrected, and the same replies pass them: one
menu of `01` matched no form of "nuovo", and the regex now asks for "espert" and "repo";
two step 3s of `04` failed a judge on a sentence saying the report and the test rely on
`is_low`, which its criteria now allow. `07` first scored 0.96, one run in three
skipping the read-back the skill asks for after every addition. After that sentence of
`SKILL.md` was strengthened and the conversation recorded again, the read-back held in 9
runs of 9, over three runs of three on 2026-11-20; the last scored 1.00, and one run of
the one before failed `says-it-is-collected` on a reply whose trace was not kept.

Without the skill, every run of `01` explained the pull request in its first turn and
dispatched no reviewer.

In `08`, every run had its `rm` of the local file refused, no grant covering it, and
handed the command over in its reply. `local-file-deleted` counts the attempt, by `rm`
or by `find -delete`, which `tools/eval.sh` now grants as `find` and the sandbox refuses
just the same.

## Before `claude plugin eval`

The scenarios used to be three hand-run conversations against the live pull requests,
with `run.sh`, `arm.sh` and `baseline.sh` and the state of the fixture read after every
turn. Their files and results, with and without the skill, from 2026-09-25 to
2026-10-30, are in git history: `git log --diff-filter=D -- evals/walking-through-and-reviewing-pull-requests/run.sh`
names the commit that deleted them, and its parent holds them. Two baselines did not
survive the move, since a resumed case runs with the skill only: without the skill,
"avanti" published the review in scenarios 02 and 03.
