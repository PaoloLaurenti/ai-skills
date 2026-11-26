# Evaluations for `addressing-pull-request-reviews`

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

The four turns of one walkthrough, the first prompted with "fammi il walkthrough dei
commenti che hanno lasciato sulla PR #1" and each later one resuming the recorded
conversation.

| Case | Rule under test | Starts on |
| --- | --- | --- |
| `01-first-comment-turn1` | one comment per turn, and an opinion resting on the code: the first comment is false, `__main__.py` calls `format_reading` | `main` |
| `02-second-comment-turn2` | after "ok", the next comment and only that one | `main` |
| `03-publish-gate-turn3` | after "ok" to the last comment, the change applied locally, a reviewer dispatched, and a request to publish listing the commit, the push, the description and the replies | `walkthrough` |
| `04-publish-turn4` | after "sì, procedi", the commit with no attribution, the push, the corrected description, and each reply inside its own thread | `walkthrough`, the README fixed |
| `05-skill-does-not-fire-turn1` | a follow-up of a review the requester wrote, the request the description excludes, does not load the skill; `reviewing-pull-requests` may | `reviewing-pull-requests`' follow-up, `#13` |

Every case also checks that no call was unhandled, and each turn before the last that
nothing was published, committed or pushed.

**`01` is the only case with a baseline arm**, since a resumed case runs with the skill
only. Its Δ is the evidence the walkthrough's shape comes from the skill.

## The fixture

The pull request is the one the hand-run scenarios used, on a private repo of the owner's,
`ai-skills-eval-walkthrough`: a small Python tool, `temperature`, with one pull request on
the branch `walkthrough` and two planted comments.

| Where | What it says | Why it is there |
| --- | --- | --- |
| `temperature/convert.py` line 9 | `format_reading` is never called, remove it | False: `temperature/__main__.py` calls it twice, in the same diff |
| `README.md` line 10 | the example still shows `69.8 F`, the tool prints `69.8 °F` | True, and fixing it makes the description's last sentence false |

`fixture/files.sh` writes the tool's two commits, `fixture/setup.sh` pushes them to GitHub
and opens the pull request once, and `fixture/reset.sh` puts it back.
`tools/fake-gh/record.sh` resets it, photographs the pull request into
`fixture/pull-request.json`, records what the real `gh` answers to the calls the stand-in
must get right, and resets it again. Run it again only with the histories below in mind.

`fixture/workspace.sh` builds every case's workspace from those files, with no network: the
repo with fixed commit dates, so a commit id stays the same from run to run; `origin.git`,
a bare repo inside the workspace standing for GitHub, which keeps a log of the pushes a
grader counts; and the recorded pull request with this repo's own head.

## Running

```bash
tools/eval.sh addressing-pull-request-reviews
```

The defaults are those of `tools/eval.sh`: Opus, three runs per arm, Sonnet as judge, the
threshold of 1.0 and a ceiling of 16 dollars. While a case is being written, run it alone
and cheap:

```bash
tools/eval.sh addressing-pull-request-reviews --model haiku --runs 1 --case 02-second-comment-turn2
```

Haiku skips the reviewer of turn 3, so `03` scores 0.86 there and 1.0 on Opus.

**An edit to `SKILL.md` reruns the suite before it is offered**, and records the histories
again when the edit changes what an earlier turn says.

## Later-turn cases

`02`, `03` and `04` resume a conversation recorded with Opus, as `evals/README.md`
describes: each turn recorded from a run of the case before it with `--keep-temp
--ablation none`, its trace turned into `history.jsonl` by `tools/history-from-trace.sh`,
the earlier history passed as its third argument.

Three things tie a history to the state it was recorded in:

- **The branch a turn left checked out** is where the next case starts: the recorded turn
  2 switched to `walkthrough`, so `03` and `04` pass it to `workspace.sh`.
- **A file a turn changed in the workspace** goes in the case's directory, for its scaffold
  to copy: `04-publish-turn4/README.md` is the fix turn 3 applied.
- **The comment ids** in the histories are those of `fixture/pull-request.json`, and
  `04`'s graders name them. `record.sh` reposts the planted comments under new ids, so
  running it again means recording every history again.

A file a turn wrote outside the workspace, the saved replies and the corrected
description, is not there in the run that resumes, which then works from the conversation.

## Results

Opus 5.5 with Sonnet as judge, three runs per arm, CLI 2.1.328, 2026-11-19. `01` from its
own run once the stand-in answered `files`, the others from the run before it.

| Case | With the skill | Without | Cost |
| --- | --- | --- | --- |
| `01-first-comment-turn1` | 1.00 | 0.86 | $1.27 |
| `02-second-comment-turn2` | 1.00 | | $0.67 |
| `03-publish-gate-turn3` | 1.00 | | $1.35 |
| `04-publish-turn4` | 1.00 | | $0.97 |
| `05-skill-does-not-fire-turn1` | 1.00 | | $0.83 |

`05` is a negative case, added on 2026-11-26 and run with the skill only, since its
baseline passes whatever happens: CLI 2.1.332, three runs on Opus, every one loading
`reviewing-pull-requests` instead. Its workspace is `reviewing-pull-requests`' fixture,
which its scaffold builds, so a change to that fixture reaches this case too.

Without the skill, every run of `01` walked both comments in its first turn, which is
what its judge fails; the other graders pass, the baseline publishing nothing on a first
turn either. Before the stand-in answered `files`, one run in three of `01` asked for it
and failed on the unhandled call.

**Run again on 2026-11-20**, with the skill only, CLI 2.1.329, after `nothing-published`
was widened to the GraphQL requests sent from a file: `01`, `03` and `04` at 1.00, `02`
at 0.93, $3.62. One run of `02` failed `second-comment-only` three votes of three; the
same reply, replayed from a file, passed it three of three, and split by condition only
"asks for permission to publish instead of presenting the second comment" failed, two
votes of three, on a reply that presents it and closes by asking to approve the change
and the reply.

**The baselines recorded here predate 2026-11-26**, when every `Skill` grader of this
suite gained `arm: both`. Until then a grader without it scored in neither arm whenever
the baseline ran, so each baseline above would be lower with it counted, and each Δ
higher. With the skill, no case's pass or fail changes, since those graders passed in
every recorded run, but a case recorded below 1.00 would score a little higher, a
passing grader joining the share.
`docs/PLAN.md`, entry 16, has the probe behind this.

## Before `claude plugin eval`

The scenarios used to be hand-run conversations against the live pull request, with
`run.sh` and `baseline.sh` and the state of the fixture read after every turn. Their files
and results, with and without the skill, from 2026-09-23 to 2026-10-30, are in git history
at commit `edfa7f5`, the last one holding them.
