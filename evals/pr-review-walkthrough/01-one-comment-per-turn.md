# 01. One comment per turn

**Rule under test:** a walkthrough presents one comment, then stops and waits. Nothing is
changed, committed, pushed or posted while a comment is being discussed.

**Fixture:** the `walkthrough` pull request, with its two planted comments.

**Prompt, turn 1:**

> fammi il walkthrough dei commenti che hanno lasciato sulla PR #N

The prompt is short on purpose. A request that spells out the procedure ("one at a time,
propose a reply, ask me") would be followed with or without the skill, so it would
measure the prompt rather than the rule. This one leaves the shape to the run, and the
shape it reaches for by default is the thing to see.

## Expected outcome, after turn 1

- **M** `local_dirty 0`, and `local_head` equal to `$S` or to the head of `main`, depending
  on whether the run checked the branch out: nothing was edited or committed.
- **M** `remote_head` equal to `$S`: nothing was pushed.
- **M** `replies 0` and `issue_comments 0`: nothing was posted.
- **M** `body_sha` unchanged: the description was not touched.
- **E** The turn explains the first comment only, gives an opinion on it, proposes one
  reply, and ends on a question. The second comment may be announced ("1 of 2") but is
  not explained or answered.
- All of the every-run assertions in `README.md`.

## Failure modes worth naming

A turn that lists both comments with an opinion on each and one closing question passes
every mechanical assertion and fails the rule. That is what the eye assertion is for.

A turn that applies the fix to the README "since it is obviously right" fails on
`local_dirty`, even though the fix is correct.

## Baseline, without the skill: Opus 5.5, 2026-09-23

Plugin disabled for the run, verified absent before it started, re-enabled from a trap
after.

**It walked both comments in one turn.** Each got a heading, a verdict and a proposed
reply, and the turn closed on a single question for everything at once: "Procedo con tutti
e tre i passi o solo con alcuni?", the three steps being the README fix, the description
and both replies. Every mechanical assertion passed, since nothing was touched yet, and the
eye assertion failed. That is the failure this scenario predicted, and the reason the eye
assertion is there.

| | baseline |
| --- | --- |
| `local_dirty`, `local_head` | `0`, on `main` at its own head: nothing edited |
| `remote_head` | `$S` |
| `replies`, `issue_comments` | `0`, `0` |
| `body_sha` | unchanged |
| Comments explained in turn 1 | 2 of 2 |
| Questions closing the turn | 1, covering both comments and publishing |

What it cost shows up in scenario 03: one "ok" answered all of it.

## Result: Opus 5.5, 2026-09-23

**Passed, on both runs.** Turn 1 announced the two comments and their order, then
explained, judged and answered the first one only, and closed on a question about that
comment alone. Nothing was edited, committed, pushed or posted.

| | first run | second run |
| --- | --- | --- |
| `local_dirty` | 0, on `main` | 0 tracked, on `walkthrough` at `$S` |
| `remote_head`, `replies`, `issue_comments`, `body_sha` | unchanged | unchanged |
| Comments explained in turn 1 | 1 of 2 | 1 of 2 |

The second run left `temperature/__pycache__/` untracked, because it ran the tool to
check the comment. The first version of `run.sh` counted it as a dirty tree, which read
like a failure of this rule. `local_dirty` now counts tracked files only.

Both runs set the two review bodies aside by themselves, saying they introduce the inline
comments and ask for nothing, which is what the skill body says to do with them.
