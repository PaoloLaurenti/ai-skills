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

| `06-card-after-humanizer-turn1` | as `04`, with `humanizer` installed and a link to the design frame in `FEATURE.md` | the card goes through `humanizer`, and still passes every grader of `04`, which it shares through links, and keeps the design frame's link |
| `07-one-question-with-grilling-turn1` | settings noted, scope and storage given, three open decisions, `grilling` installed | step 3 follows `grilling`, yet asks one open decision only, with its recommended answer, never a numbered round, and drafts no card |
| `08-skill-does-not-fire-turn1` | no team settings noted, the tracker | "implementa la card MOB-12" asks to build what a card describes, which the description excludes, so the skill does not load; another skill of the plugin may |

`01` to `04`, `06` and `07` also check that nothing was written, since each one ends
before an answer.

`06` and `07` load a second plugin beside this one through their `plugins` key, the one
installed on the machine that runs them, which `tools/eval.sh` copies in; on a machine
without it they fail to load, as `evals/README.md` explains. `04` is the path without
`humanizer`; no case asks the open decisions without `grilling`.

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

`06` and `07` change `FEATURE.md` in their own scaffold after it, leaving the shared
fixture as `05`'s histories saw it: `06` turns the design line into a link to a frame,
which the fixture otherwise lacks, and `07` adds two open decisions that do not depend
on the link's validity, since a round of one question passes "one per message" whatever
the rule says.

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

| Case | With the skill | Baseline, 2026-11-26 | Δ |
| --- | --- | --- | --- |
| `01-asks-the-settings-turn1` | 1.00 | 0.06 | +0.94 |
| `02-summarises-the-settings-turn1` | 1.00 | 0.00 | +1.00 |
| `03-slices-checked-turn1` | 1.00 | 0.47 | +0.53 |
| `04-card-follows-the-definition-of-ready-turn1` | 1.00 | 0.71 | +0.24 |
| `05-write-plan-turn1` | 1.00 | 0.50 | +0.50 |
| `05-option-is-not-the-yes-turn2` | 1.00 | | |
| `05-write-after-the-yes-turn3` | 1.00 | | |
| `05-change-reported-turn3` | 1.00 | | |
| `06-card-after-humanizer-turn1` | 1.00 | 0.67 | +0.33 |
| `07-one-question-with-grilling-turn1` | 1.00 | 0.43 | +0.57 |
| `08-skill-does-not-fire-turn1` | 1.00 | 1.00 | 0.00 |

The baseline arm ran on CLI 2.1.332 on 2026-11-26, the first turns only, since a later
turn runs with the skill alone: three runs per arm, $10.68 for both arms, 12 minutes with
`-j 2`, a mean Δ of +0.52. The Δ is a share of graders, so it counts the `skill-loaded`
grader too, which no baseline run can pass. By its graders, without the skill Opus
drafted cards in `01` and asked none of the team's settings, read no noted settings in
`02`, dispatched no subagent in `03`, wrote criteria in a form of its own in `04` and
`06`, and asked no explicit yes in `05`; in `07` two runs asked the open decisions as a
round, and one asked a single decision. `08` passes in both arms, as a negative case
must. Those are the graders' verdicts, which entry 16 of `docs/PLAN.md` measures against
the requester's.

In the same run, with the skill, one run of `04` scored 0.86, and the fault was the
grader's: `no-default-sections` matched "**Why it's in To refine:**", a note after the
card, as the default section "Why". It now counts only a heading or a bold label naming
the section alone, and on a replay passed that reply and failed a copy with "**Why**:"
in it.

`06` and `07` ran on CLI 2.1.332 the same day, with `humanizer` 4.0.0 and
`mattpocock-skills` 1.3.1. Every run of `06` passed the card to `humanizer`, telling it
to keep the sections, links and criteria's form, and showed the rewritten card, $1.03;
a trial on haiku did not call it, saying the card was plain enough already. `07` first
scored 0.86: no run loaded `grilling`, listed in all of them, while step 3 said to
"follow it" where installed, and a throwaway copy asking for it by name did load it and
still asked one decision. Step 3 now says to load it before the first question, and
`07` scored 1.00, $0.63, with `01` to `04` rerun on the new text, all at 1.00, $3.21.
An earlier version of `07`, with the fixture's design line, had one run ask for the
design file's link first, a missing source rather than an open decision, so the case's
`FEATURE.md` now says there is no design. `one-question` was made to fail first, on a
replayed reply asking two decisions: four runs, FAIL every vote.

`04` fell short once on `fits-on-one-screen`, which then measured the whole reply: the
card took 31 lines and the settings summary and notes around it took the rest. It now
measures the stretch between horizontal rules. Its first version of that failed every
run unnoticed, since a comment naming the rule held three dashes, which end a grader's
frontmatter; rewritten, `04` scored 1.00 again on Opus, three runs, $0.59. A first run of
`04` asked an open decision before drafting, as step 3 says, so its prompt now leaves
the missing decisions to the card's open points; that run also showed a sentence naming
the Definition of Ready's sections satisfying the section grader, which now asks for
headings or bold labels.

In `03` every run of both passes dispatched the subagent; in one of the first it found a split, the rule refusing
the current password carved out of setting a new one, put to the requester as a
question. The baseline arm's runs of `03`, on 2026-11-26, are in the table above.

In the first trial of `05`'s first turn, Opus asked the cycle first and said a yes would
follow, which the yes grader then failed; the rule allows it, since choosing the cycle
is not the yes, so the grader now accepts a yes announced as well as one asked. A trial
of the third turn with the automation in the plain mock showed that a run finding MOB-9
changed may stop the batch and ask, which the rule allows too, and which failed the
graders of a complete batch: hence two third turns, one per mock. In the suite's runs of
`05-change-reported-turn3`, all three completed the batch and reported the change at the
end. The plain mock first answered with no cycle and with a relation on one card only,
which a run reported as a fault of the tracker, so it now gives both.

`08` is the suite's one negative case: it guards the description against firing where it
should not, and its baseline arm passes whatever happens, so it runs with the skill only.
Its workspace leaves out `cards.md`, where MOB-12 is a card to rewrite and the request
would be ambiguous. A trial on haiku and three runs on Opus, CLI 2.1.332, 2026-11-26,
$0.50: 1.00, and every run read MOB-12 from the tracker, found no code to build it in,
and asked where the code is. The haiku run offered to rewrite the card with this skill,
without loading it. Its grader was made to fail first, in a throwaway plugin, as
`docs/PLAN.md`, entry 16, records.

The graders that pass only by finding nothing were made to fail once, on haiku with a
fixed mock, two runs each, $0.02: `no-write-outside-the-scope` failed on a save whose
description named MOB-9 and passed on one naming "Biometric sign-in" alone, and
`reports-the-change`, replayed on a plain-mock reply saying MOB-9 had not changed,
failed. What is left ungraded: the record of the cards outside the scope before the
batch, which a run may take in an earlier turn; and whether both the second new card and
MOB-12 carry the link and the "blocked by", since one is enough to pass. Both graders
take MOB-20 to be "Request a reset link", the order every run so far created them in.
