# Evaluations for `reviewing-pull-requests`

What the skill is supposed to do, written as cases that `claude plugin eval` runs and
grades. Each case directory holds a `case.yaml` with the prompt, a `scaffold.sh` building
the workspace, and a `graders/` directory, one file per check, most of them read from
what Claude did rather than judged.

The skill drives a pull request through `gh`, which the sandbox cannot reach. Every run
talks to a stand-in instead, `tools/fake-gh`, which answers from the photographed pull
request in the workspace's `.git/fake-gh/pull-request.json`, keeps what a run changes, and
logs every call in `.git/fake-gh/calls.jsonl`, with the payload a review is created from
as its `input`. A call it does not know is logged as `"unhandled"`, and every case
requires none.

## The cases

One case per rule, each at its first turn and with a baseline arm, so its Δ shows what
the rule adds. `09` and `10` go on through the turns that follow, resuming a recorded
conversation.

| Case | Pull request | Rule under test |
| --- | --- | --- |
| `01-pending-state-turn1` | `#14`, `pending-state` | non-negotiable 1: asked to "leave a review", the review is created pending, never submitted, and the reply gives the submit command |
| `02-inline-versus-body-turn1` | `#2`, `inline-versus-body` | a finding whose file is in the diff sits inline, linking the `AGENTS.md` rule it breaks, and the general comment does not repeat it |
| `03-permalinks-turn1` | `#3`, `permalinks` | every file named is a link pinned to the head, the README outside the diff included, and each line anchor falls on the line the finding is about |
| `04-suggestion-blocks-turn1` | `#4`, `suggestions` | the `COPY` and `chmod` collapse into a `suggestion` anchored on every replaced line, the debug line's deletion gets no block, no block carries `+` or `-` |
| `05-docs-cross-check-turn1` | `#5`, `docs-cross-check` | the stale port in `README.md`, outside the diff, quoted and linked in the general comment, with a proposal carrying no port |
| `06-architectural-claims-turn1` | `#6`, `architectural-claims` | the deployed unit is `platform-api`, as the workflow defines it, never the `example-service` the description claims |
| `07-append-to-open-draft-turn1` | `#7`, `open-draft` | non-negotiable 2: an open draft is appended to, its comments and general comment kept word for word, and the one breaking the conventions named in the reply |
| `08-values-in-docs-turn1` | `#8`, `values-in-docs` | the doc section copying two values is answered inline with prose that carries no number and names `app/limits.py` |
| `09-follow-up-first-thread-turn1` | `#13`, `follow-up` | a follow-up counts two threads, presents the first alone, proposes resolving it and asks, with nothing written to GitHub |
| `10-asks-how-to-build-turn1` | `#4`, `suggestions` | a request naming no approach gets one question, after the reviewer, between the draft now and one point at a time |

The later turns, each resuming the conversation recorded up to it:

| Case | Prompt | Rule under test |
| --- | --- | --- |
| `09-follow-up-resolve-turn2` | "sì" | the yes resolves the first thread at once and nothing else, and thread 2 of 2 follows in the same message |
| `09-follow-up-final-turn-turn3` | "sì" | the second thread resolved, then the final turn: the untouched two-decimals thread, `app/currency.py` changed by no thread, and the question whether to review the new commits |
| `09-follow-up-delivery-turn4` | "no, niente review dei nuovi commit…", with a reply to add | one pending draft holding the reply on the two-decimals thread, by GraphQL, nothing published or submitted, the submit command given |
| `10-one-at-a-time-turn2` | "uno alla volta" | point 1 of 3 alone, its comment in a fenced block, one question, nothing written anywhere |
| `10-drop-collects-nothing-turn3` | "scarta" | nothing collected, point 2 of 3 in the same message |
| `10-add-with-a-change-turn4` | "aggiungilo, ma togli l'ultima frase" | the comment collected in `~/.local/state/reviews/` without its last sentence, never sent to GitHub, point 3 of 3 next |
| `10-delivery-turn5` | "aggiungi" | one `POST` creating the draft with the two comments added, nothing submitted, the submit command given, the local file deleted |

Every case that delivers a review also checks, over the payload in the log, what the
hand-run suite checked on every review: one `POST` and no `event`, nothing published, no
em dash, no first or second person, every repo file linked outside a `suggestion` block,
every permalink at the workspace's head, no inline comment's prose past 80 words before
its block, English, and the reviewer dispatched before anything was written.

## The fixture

A private repo of the owner's, `ai-skills-eval-fixture`, holding a small service with a
`Dockerfile`, an entrypoint, a config file, a `README.md`, an `AGENTS.md` and a deploy
workflow, and one pull request per branch, each planting what its case looks for.

`fixture/files.sh <branch>` writes `main` and the branch's commit, and both
`fixture/setup.sh`, which creates the repo on GitHub once, and `fixture/workspace.sh`,
which builds a case's workspace, call it, so the two hold the same files.
`fixture/photograph.sh` puts every pull request back as the cases start from it with
`fixture/reset.sh`, seeds the follow-up's submitted review, author's fix and reply with
`fixture/seed-submitted-review.sh`, and photographs each pull request into
`fixture/pull-requests/<branch>.json` with `tools/fake-gh/photograph.sh`. Run it again
only with the histories below in mind.

`fixture/workspace.sh <branch>` builds a workspace with no network: the repo on the branch
with fixed commit dates, the follow-up's fix included; `origin.git`, a bare repo inside
the workspace standing for GitHub; and the photograph, its commits replaced by the
workspace's own, since the follow-up fetches the reviewed commit and logs what changed
since. On `open-draft` it creates the draft someone left open, `fixture/open-draft.json`,
through the stand-in, since GitHub lists a pending review's comments nowhere a photograph
reads; given thread ids, it resolves them, as a recorded turn of the follow-up did.

## Running

```bash
tools/eval.sh reviewing-pull-requests
```

The defaults are those of `tools/eval.sh`: Opus, three runs per arm, Sonnet as judge and
the threshold of 1.0. Its ceiling of 16 dollars is too low for this suite: the first
turns cost about $22 and the later ones about $8, so a full run passes
`--max-cost-usd 35`. While a case is being written, run it alone and cheap:

```bash
tools/eval.sh reviewing-pull-requests --model haiku --runs 1 --case 05-docs-cross-check-turn1
```

**An edit to `SKILL.md` reruns the suite before it is offered**, and records the histories
again when the edit changes what an earlier turn says.

## Later-turn cases

The later turns of `09` and `10` resume a conversation recorded with Opus, as
`evals/README.md` describes: each turn rebuilt by `tools/history-from-trace.sh` from a
passing run of the case before it, kept with `--keep-temp`, the earlier history passed as
its third argument. `10`'s first turn was recorded from a run whose reviewer left three
points, the chown with no `USER`, the `COPY` and `chmod` pair, and the debug line.

Three things tie a history to the state it was recorded in:

- **What a turn changed on the pull request.** The workspace starts from the photograph,
  so `fixture/workspace.sh` replays a recorded turn's writes before the run: given thread
  ids, it resolves them through the stand-in, as `09`'s turns 2 and 3 did.
- **A file a turn wrote in the run's home** goes in the case's directory, for its
  scaffold to copy: `10-delivery-turn5/collected/` is the collected review as turn 4 left
  it, which the scaffold puts in `~/.local/state/reviews/`.
- **The comment ids** in `09`'s histories are those of
  `fixture/pull-requests/follow-up.json`. `photograph.sh` seeds the review again under
  new ids, so running it again means recording those histories again.

## Results

Opus 5.5 with Sonnet as judge, three runs per arm, CLI 2.1.329, 2026-11-20.

| Case | With the skill | Without | Cost |
| --- | --- | --- | --- |
| `01-pending-state-turn1` | 1.00 | 0.45 | $2.32 |
| `02-inline-versus-body-turn1` | 1.00 | 0.56 | $2.05 |
| `03-permalinks-turn1` | 1.00 | 0.67 | $2.23 |
| `04-suggestion-blocks-turn1` | 1.00 | 0.67 | $2.21 |
| `05-docs-cross-check-turn1` | 0.96 | 0.56 | $2.36 |
| `06-architectural-claims-turn1` | 0.94 | 0.56 | $2.35 |
| `07-append-to-open-draft-turn1` | 0.91 | 0.74 | $2.67 |
| `08-values-in-docs-turn1` | 0.98 | 0.60 | $2.28 |
| `09-follow-up-first-thread-turn1` | 1.00 | 0.43 | $1.57 |
| `10-asks-how-to-build-turn1` | 1.00 | 0.40 | $2.02 |
| `09-follow-up-resolve-turn2` | 1.00 | | $0.76 |
| `09-follow-up-final-turn-turn3` | 1.00 | | $0.81 |
| `09-follow-up-delivery-turn4` | 1.00 | | $1.20 |
| `10-one-at-a-time-turn2` | 0.94 | | $0.86 |
| `10-drop-collects-nothing-turn3` | 1.00 | | $0.85 |
| `10-add-with-a-change-turn4` | 0.94 | | $1.59 |
| `10-delivery-turn5` | 0.88 | | $1.44 |

Several fell short on graders or on the stand-in, since corrected, and the same runs pass
them:

- `every-file-linked` read a file named inside a fenced block, or inside a sentence
  proposed as a document's new wording, where a link would corrupt the text handed over:
  two runs of `05`, one of `08`. It now reads a mention only outside both.
- `example-service-not-the-deployed-unit`, a regex in `06`, matched the reviews that
  corrected the description's claim. It is gone; the judge on the deployed unit passed
  every run.
- `comments-under-80-words` measured as one comment the general comment `07` appended to
  the open draft by GraphQL. It now measures the comments a run adds.
- The stand-in did not know a GraphQL request given as a file, `gh api graphql --input`,
  which one run of `07` sent, nor a review created from `-f` fields, which one run of
  `09`'s turn 4 sent. It knows both now, and `09`'s turn 4, run again, scored 1.00.
- The judges of `10`'s turns 2 and 4 asked for a fence of three backticks and for the
  final text shown in the reply, which `SKILL.md` does not ask; replayed, the failing
  replies passed 8 votes of 9 and 6 of 6.
- `local-file-deleted` in `10`'s turn 5 looked for `rm`, and every run used
  `find -delete`, which `tools/eval.sh` grants as `find`. The sandbox refuses both, and
  each run handed the command over in its reply.

Two shortfalls were the skill's: one run of `06` named `app/server.py`, and one of `07`
"the Dockerfile", without a link.

**After the linking rule was strengthened**, the same day, to name "the Dockerfile" or
"the README line" and a second mention, and to reread every comment before sending, the
suite ran again with the skill only, three runs per case, $24.29: every case at 1.00 but
`10-asks-how-to-build-turn1`, at 0.89. On the way, `07` showed three faults outside the
skill, since corrected: `every-file-linked` counted "a Dockerfile `HEALTHCHECK`", the kind
of instruction rather than this repo's file, and now skips a file name after "a"; the
stand-in kept `-F body=@file` as the literal path rather than the file's contents;
`reviewer-before-the-append` missed an append sent as `gh api graphql --input`. The
rule worded as first approved still left "The Dockerfile line" bare in two runs of three,
and the reread before sending closed it.

`10`'s first turn failed `one-question-on-the-approach` in two runs, three votes each,
while the same replies replayed from a file passed it three votes of three. Split into one
condition each, the replay failed "names the weightiest point" on both: the replies list
every point in detail and ask, without saying which point weighs most, which `SKILL.md`
asks for. The baseline arm, which the change does not touch, was not run again.

Without the skill, every run of `01` tried to submit its review, with an `event` or
`gh pr review`, no run of `07` appended to the open draft, and no run of any case
dispatched a reviewer.

## Before `claude plugin eval`

The scenarios used to be run by hand against the live pull requests, with `run.sh` and
`baseline.sh`, the state of the fixture read by command after every turn, and Sonnet as
a second model. Their files and results, with and without the skill, from 2026-08-05 to
2026-10-30, are in git history: `git log --diff-filter=D -- evals/reviewing-pull-requests/run.sh`
names the commit that deleted them, and its parent holds them.
