# Evaluations for `writing-cards`

What the skill is supposed to do, written as cases that `claude plugin eval` runs and
grades. Each case directory holds a `case.yaml` with the prompt, a `scaffold.sh` building
the workspace, and a `graders/` directory, one file per check.

The sandbox reaches no tracker, so every case stops before a write: what the cases test
is everything the skill does on its own side of that line. Whether nothing reaches the
tracker before an explicit yes needs a stand-in for a tracker, which this suite does not
have yet.

## The cases

| Case | Workspace | Rule under test |
| --- | --- | --- |
| `01-asks-the-settings-turn1` | no team settings noted | an Italian request fires the skill, which asks for the team's settings it lacks, the Definitions of Done and Ready, the cards' language and the not-ready status, and drafts no card |
| `02-summarises-the-settings-turn1` | settings noted | the noted settings are read and summarised to the requester, all four |
| `03-slices-checked-turn1` | settings noted, scope and storage given | the slices are behaviours a person can see, they go to a subagent checking for thinner cuts, and no card is drafted yet |
| `04-card-follows-the-definition-of-ready-turn1` | settings noted, one slice in scope | the card takes the Definition of Ready's sections in its order and none of the defaults, its criteria take the default form the Definition of Ready leaves open, no link validity is made up, and the card stays under 41 lines |

Every case also checks that nothing was written, since every one ends before an answer.

## The fixture

`fixture/setup.sh` writes the feature to split, `FEATURE.md`, a password reset whose link
validity is left undecided, and the team's Definition of Ready, `dor.md`, which sets a
card's sections and gives no instructions for its acceptance criteria. Given `cached`, it
puts `fixture/settings.md` where an earlier session would have noted the team's
settings, `~/.local/state/writing-cards/linear-mobile.md`, in the run's home.
`tools/eval.sh` grants Read, Edit and Write there, which a run refuses otherwise.

## Running

```bash
tools/eval.sh writing-cards
```

## Results

Opus 5.5 with Sonnet as judge, three runs per case, skill only, CLI 2.1.330,
2026-11-26: $3.07, 8 minutes.

| Case | With the skill |
| --- | --- |
| `01-asks-the-settings-turn1` | 1.00 |
| `02-summarises-the-settings-turn1` | 1.00 |
| `03-slices-checked-turn1` | 1.00 |
| `04-card-follows-the-definition-of-ready-turn1` | 0.95 |

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
