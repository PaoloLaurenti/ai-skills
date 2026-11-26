# Evaluations for `writing-cards`

What the skill is supposed to do, written as cases that `claude plugin eval` runs and
grades. Each case directory holds a `case.yaml` with the prompt, a `scaffold.sh` building
the workspace, and a `graders/` directory, one file per check; a later turn adds the
`history.jsonl` it resumes, and a case talking to the tracker its `mocks`.

The sandbox reaches no tracker, so the cases that write talk to a stand-in: a mock of a
tracker's MCP server, which `claude plugin eval` starts from the case's `mocks/`
directory, as the fixture below explains. Every call it receives is in `mock_calls`,
which the graders read.

## The cases

| Case | Workspace | Rule under test |
| --- | --- | --- |
| `01-asks-the-settings-turn1` | no team settings noted | an Italian request fires the skill, which asks for the team's settings it lacks, the Definitions of Done and Ready, the cards' language and the not-ready status, and drafts no card |
| `02-summarises-the-settings-turn1` | settings noted | the noted settings are read and summarised to the requester, all four |
| `03-slices-checked-turn1` | settings noted, scope and storage given | the slices are behaviours a person can see, they go to a subagent checking for thinner cuts, and no card is drafted yet |
| `04-card-follows-the-definition-of-ready-turn1` | settings noted, one slice in scope | the card takes the Definition of Ready's sections in its order and none of the defaults, its criteria take the default form the Definition of Ready leaves open, no link validity is made up, and the card stays under 41 lines |
| `05-write-plan-turn1` | settings noted, cards approved in `cards.md`, the tracker | the skill fires, lays out the write plan and asks for an explicit yes, or says it will, and sends no write to the tracker |
| `05-option-is-not-the-yes-turn2` | as the turn before | the requester picks the cycle `cards.md` left open, "ciclo 15": choosing an option is not the yes, so the skill asks for it and still writes nothing |
| `05-write-after-the-yes-turn3` | as the turn before, the cycle noted | after "sì", both new cards are created and no third, MOB-12 in scope is rewritten, no write names MOB-7 or MOB-9 outside the scope, MOB-9 is named by its plain title, the link and the "blocked by" to the first new card use the id its create returned, and the cards outside the scope are read again after a write |
| `05-change-reported-turn3` | as the turn before, with a tracker automation | an automation nobody mentioned adds a label to MOB-9 right after the first create: the skill finds it, reports it, and does not write to MOB-9 to undo it |

`01` to `04` also check that nothing was written, since each one ends before an answer.

## The fixture

`fixture/setup.sh` writes the feature to split, `FEATURE.md`, a password reset whose link
validity is left undecided, and the team's Definition of Ready, `dor.md`, which sets a
card's sections and gives no instructions for its acceptance criteria. Given `cached`, it
puts `fixture/settings.md` where an earlier session would have noted the team's
settings, `~/.local/state/writing-cards/linear-mobile.md`, in the run's home.
`tools/eval.sh` grants Read, Edit and Write there, which a run refuses otherwise. Given
`approved`, it does the same and adds `fixture/cards.md`: two new cards and the rewrite
of MOB-12, approved one at a time, with the scope, the label `reset-password`, and the
decisions behind them, one of which, the cycle, is left to the requester. It names MOB-9
by id among the decisions, and by title only in the cards' text.

The tracker is `fixture/mocks/linear/`, an agent mock: `_server.md` tells the judge model
to play the tracker's API, with three cards to start from and the earlier calls of the
run as its memory, and `_tools.json` gives the three tools it answers, `list_issues`,
`get_issue` and `save_issue`, which a run sees as `mcp__linear__<tool>`. New cards are
numbered from MOB-20, so a grader knows the id each create returns, and a description
naming another card by id or url relates the two, as Linear does.
`fixture/mocks-with-automation/` adds the automation `05-change-reported-turn3` is about,
and nothing else; the two `_server.md` files differ by that bullet alone, so an edit to
one goes into the other. Each case links its `mocks` to one of them, and `tools/eval.sh`
turns the link into a copy, since the command wants a real directory.

The histories of the later turns come from passing runs on Opus, rebuilt with
`tools/history-from-trace.sh`: `05-option-is-not-the-yes-turn2` resumes the first turn,
and both third turns resume the second, sharing one `history.jsonl`. The second turn
noted the cycle in `cards.md`, so the third turns' scaffold puts back the `cards.md` it
left.

## Running

```bash
tools/eval.sh writing-cards
```

## Results

Opus 5.5 with Sonnet as judge, three runs per case, skill only. `01` to `04` on CLI
2.1.330, 2026-11-26: $3.07, 8 minutes. `05` on CLI 2.1.332, the same day: $3.60, of which
$0.49 were the tracker's answers, 7 minutes.

| Case | With the skill |
| --- | --- |
| `01-asks-the-settings-turn1` | 1.00 |
| `02-summarises-the-settings-turn1` | 1.00 |
| `03-slices-checked-turn1` | 1.00 |
| `04-card-follows-the-definition-of-ready-turn1` | 0.95 |
| `05-write-plan-turn1` | 1.00 |
| `05-option-is-not-the-yes-turn2` | 1.00 |
| `05-write-after-the-yes-turn3` | 1.00 |
| `05-change-reported-turn3` | 1.00 |

`04` fell short once on `fits-on-one-screen`, which then measured the whole reply: the
card took 31 lines and the settings summary and notes around it took the rest. It now
measures the stretch between `---` rules, and all three replies pass it. A first run of
`04` asked an open decision before drafting, as step 3 says, so its prompt now leaves
the missing decisions to the card's open points; that run also showed a sentence naming
the Definition of Ready's sections satisfying the section grader, which now asks for
headings or bold labels.

In `03` every run of both passes dispatched the subagent; in one of the first it found a split, the rule refusing
the current password carved out of setting a new one, put to the requester as a
question. The baseline arm was not run.

In the first trial of `05`'s first turn, Opus asked the cycle first and said a yes would
follow, which the yes grader then failed; the rule allows it, since choosing the cycle
is not the yes, so the grader now accepts a yes announced as well as one asked. A trial
of the third turn with the automation in the plain mock showed that a run finding MOB-9
changed may stop the batch and ask, which the rule allows too, and which failed the
graders of a complete batch: hence two third turns, one per mock. In the suite's runs of
`05-change-reported-turn3`, all three completed the batch and reported the change at the
end. The plain mock first answered with no cycle and with a relation on one card only,
which a run reported as a fault of the tracker, so it now gives both.

The graders that pass only by finding nothing were made to fail once, on haiku with a
fixed mock, two runs each, $0.02: `no-write-outside-the-scope` failed on a save whose
description named MOB-9 and passed on one naming "Biometric sign-in" alone, and
`reports-the-change`, replayed on a plain-mock reply saying MOB-9 had not changed,
failed. What is left ungraded: the record of the cards outside the scope before the
batch, which a run may take in an earlier turn; and whether both the second new card and
MOB-12 carry the link and the "blocked by", since one is enough to pass. Both graders
take MOB-20 to be "Request a reset link", the order every run so far created them in.
