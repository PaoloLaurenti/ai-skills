# Plan

Work queued for this repo, ordered for execution. It comes out of an audit
against Anthropic's [skill authoring best
practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview),
the `superpowers:writing-skills` guidance, and the [Claude Code plugin
docs](https://code.claude.com/docs/en/plugins).

Task 1 carries the most value by a wide margin and is the only real work left, now
that the mechanical checks that ran ahead of it are done. Its scaffolding is in
place, so what remains there is running the scenarios and recording what the runs
say. Task 2 is a tripwire rather than work.

Already in line, so absent from this plan: the `description` field states
triggering conditions without summarising the workflow, trigger phrasings reach
the loader in both languages, the skill body sits under the 500-line limit, and
the Red flags section gives the skill its self-check list. That second item was
recorded here as fine while it was not, which the entry on the truncated
description below explains.

## 1. Write evaluation scenarios for the skill

The largest gap. Nothing in the repo tests whether the rules hold.

Anthropic asks for at least three evaluations per skill, plus a run against every
model the skill is meant for. `superpowers:writing-skills` goes further: run the
scenario against a subagent *without* the skill first, record the rationalisations
verbatim, and only then write or amend the guidance, so each rule answers a
failure that was actually observed.

Where this stands: twenty runs are recorded, five scenarios and five baselines on each
of Opus 5 and Sonnet 5, which is the whole model set. On Opus, four scenarios passed and
one broke a narrower rule beside the one it tests. On Sonnet, three passed and two
failed, on the permalink rule and the suggestion rule. The baselines turned "the rules
appear to hold" into "each rule answers a failure that was observed" for most of the
rules, with two exceptions the entry below names. Nothing here calls for a change to the
skill body: every failure recorded is a rule the body already states plainly.

What is left in this task is the migration to `claude plugin eval`, which is blocked on
early access, so the substance of it is done.

Two of those runs found problems the fixture had not planted: a configuration key
nothing in the tree reads, and a `README.md` sentence that had become misleading
rather than false. Three exposed defects in the assertions themselves. Two reported
violations on clean reviews, both of them counting where they should have been reading.
The third failed the other way, passing a real violation in silence, and only a
baseline run produced the violation that revealed it.

Two defects surfaced before any run, both of which would have read as a skill
failure rather than a harness one. Dry-running the fixture scripts against stubbed
`gh` and `git push` showed that a machine can scope its git identity to a directory
tree, leaving the fixture's temporary clone without one, so the fixture now derives
an identity from the authenticated account. And a run has to happen from inside a
clone of the fixture rather than from this repo, or `gh` resolves the wrong
repository entirely.

Both open questions about the harness are now answered. A `claude -p` prompt does
reach the skill: nothing in the output announces the load, but the review carries its
fingerprints, created pending rather than submitted, written in English out of an
Italian prompt, every file mention a permalink at the head SHA. And a fresh
`CLAUDE_CONFIG_DIR` does **not** carry credentials, so a baseline cannot be run that
way. It disables the plugin instead and re-enables it from a trap.

Worth knowing before this goes further: `claude plugin eval` exists, takes
`evals/**/case.yaml` or `evals/**/prompt.md` plus `graders/*.md`, and carries an
`--ablation with-without` mode that runs the no-plugin baseline arm itself, along with
repeated runs, a judge model, an HTML report and a cost ceiling. It is in early access
and gated on this machine, so nothing can be run through it yet, and the hand-rolled
harness stays. The direction it sets is clear enough to plan for, and the inputs
already written, the fixture, the prompts and the assertions, are what a case file
would need.

Three decisions make the task executable at all, so they come before any
scenario gets written:

- [x] **A fixture the scenarios run against.** Every scenario needs a real pull
      request: a diff, a head SHA, a repo where `gh api` can create a pending
      review. Set up a sandbox repo with one open PR per scenario, plus a reset
      step between runs, since GitHub allows one pending review per user per PR
      and every run leaves one behind.
- [x] **A run procedure and pass criteria.** Each scenario is a file carrying
      the prompt and a checklist of observable assertions, because almost every
      expected outcome checks mechanically: review state `PENDING` via `gh api`,
      the finding inline rather than in the body, permalinks containing the head
      SHA, the `suggestion` anchor covering every replaced line. A suite graded
      by eye is not a regression suite.
- [x] **A clean baseline environment.** The symlink makes the skill loadable in
      every session on this machine, and a scenario prompt is written to match
      the skill's description. A baseline run needs the plugin genuinely absent:
      disabled, or a session under an alternate configuration directory. Verify
      the skill did not load before trusting the run as a baseline.

The skill runs to 400 lines of rules, and the ones most worth testing are those
an agent under pressure will negotiate with:

- [x] **Pending state.** A scenario where the review reads as finished and the
      request is ambiguous about publishing. Expected: the review stays `PENDING`
      and the submit command gets reported back.
- [x] **Inline versus general comment.** A finding whose file sits in the diff.
      Expected: it lands inline, not as a body bullet. Ran mixed: the finding did land
      inline and the trap did not catch it, while the body restated the fix the inline
      comment already proposed, which the same section of the skill body forbids. A
      rule broken rather than a rule missing, so it wants a second data point rather
      than an amendment.
- [x] **Permalinks and line numbers.** A review naming several repo files.
      Expected: every mention is a link pinned to the head SHA, with line numbers
      checked against that ref rather than recalled.
- [x] **Suggestion blocks.** A fix that is a concrete edit to diffed lines.
      Expected: a `suggestion` block whose anchor covers every replaced line, and
      no block at all for a pure deletion.
- [x] **Docs cross-check.** The skill's own text calls this step mandatory and
      easy to skip, which is exactly what a baseline exposes. A diff that
      contradicts a claim in the fixture repo's `README.md`. Expected: the
      finding names the file, links it, and quotes the stale claim.
- [x] **Style rules as assertions on every scenario, not a scenario of their
      own.** Em dashes, first and second person, English: the rules a model
      violates most naturally, because they fight its default writing style, and
      each one greps. Every review any scenario produces gets these checks.
- [x] Record a baseline run without the skill for each scenario, so the value of each
      rule is visible. All five recorded, each under its own heading in its scenario
      file, all on Opus 5. What they settle:

      **Delivery is what the skill contributes, and scenario 01 proves it rather than
      implying it.** Four baselines posted nothing at all and wrote the review into the
      reply. The fifth submitted: a published `COMMENTED` review plus an inline comment
      visible to the repo, settled on after GitHub refused `REQUEST_CHANGES` on an own
      pull request. So the pending-state rule is not merely unexercised without the
      skill, it is contradicted, and the run's own words show publishing was never a
      decision it weighed. That is the strongest single result in the set.

      **The style rules break every time.** Seven em dashes in each of the four new
      baselines, without exception, and that identical number is not a counting
      artifact: occurrences and matching lines agree at seven in all four. Four
      markdown headings in three of them, a verdict opening in all four, first and
      second person throughout, and Italian wherever the prompt was Italian. Scenario
      03's prompt is the one written in English, so its English review is the prompt's
      doing and not evidence.

      **Permalinks and line numbers are intermittent rather than absent**, which is
      new. Scenario 03's baseline wrote five permalinks unprompted and got all four of
      its line numbers right, then pinned two of the five links to the base commit
      instead of the head SHA: correct today, stale the moment either file moves, and
      invisible to any grading that only checks the link resolves. Scenario 01's
      baseline cited `AGENTS.md:8` for a rule on line 7 of a seven-line file. One run in
      two.

      **Two rules got no baseline evidence at all**, because both need a review object
      to exist before they can be broken: inline versus body placement, and the
      suggestion anchor. Their scenario files say so rather than scoring them as
      vindicated. The nearest thing to evidence is scenario 02's baseline offering,
      unprompted, to post its finding "come commento inline", which points the way the
      rule already goes.

      **Finding real defects is not what the skill contributes.** Every baseline found
      what the fixture planted, and three went beyond it: the stale README claim in 05,
      the PR description that misdescribes its own diff in 03, and in 04 the
      observation that `main` already carried `COPY --chmod=755`, making the change
      net-negative.

      Two harness fixes came out of the round. Scenario 03's head-SHA assertion was
      `grep -c 'blob/main'` is `0`, which a link pinned to the base commit passes in
      silence, and that is exactly what the baseline produced: it is now a prefix match
      of every `blob/<sha>` against `headRefOid`. And the grading recipe in
      `evals/writing-pr-reviews/README.md` assumed a review object exists, which four of
      five baselines did not create, so it now says the reply is the text to grade when
      nothing was posted.
- [x] run the whole suite against Sonnet 5, five scenarios and five baselines. Done,
      `--model sonnet` resolving to `claude-sonnet-5`, and every result recorded in its
      scenario file beside the Opus one. The prediction in this entry was that the style
      rules would be the first to give. It was wrong, and usefully so:

      **The style rules held perfectly.** Zero em dashes, zero first or second person,
      zero markdown headings and English throughout, across all five reviews, on the
      model where they were expected to break. Every baseline broke them, as before.

      **Two rules gave instead, and both cost an extra step at writing time.** Scenario
      03 failed E6: four bare mentions, every one of them a repeat of a file that had
      already been linked once. Scenario 04 failed its own rule outright, no
      `suggestion` block anywhere, with a concrete `USER 1000:1000` insertion written as
      prose where Opus handed over a committable block on the same line. Neither is a
      gap in the skill body, which states both rules plainly and repeats them in the red
      flags, so nothing there moves. What they are is the first evidence the model set
      was built to produce: rules that hold on Opus and give on Sonnet.

      **Delivery held on both models**, which is the load-bearing result. Five `PENDING`
      reviews, nothing published, the submit command reported with its review id every
      time, and the empty general comment appearing correctly on the three scenarios
      whose findings all sit in the diff.

      **The baselines reproduced across models almost exactly.** Scenario 01's Sonnet
      baseline submitted a published review for the same stated reason as the Opus one,
      `REQUEST_CHANGES` being unavailable on an own pull request. The other four posted
      nothing. Every baseline broke every style rule, and two produced nearly identical
      banned verdict lines on two different models.

      One thing to carry forward, recorded in scenario 01: Sonnet shipped a `suggestion`
      pinning `FROM python:3.14-slim`, a version nothing in the run could verify, having
      hedged the uncertainty in the prose beside it. Opus wrote the same block and
      deleted it for exactly that reason. Worth a second data point before deciding
      whether the suggestions section should say outright that an unverified value does
      not belong in a committable block.
- [x] get scenario 02 a second data point. Settled, and it settles the other way: Sonnet
      passed the scenario outright, including the narrower rule Opus broke, leaving the
      general comment empty and saying so. So the Opus result stays a recorded
      inconsistency, and nothing in the section about what the general comment contains
      moves.
- [ ] move the suite onto `claude plugin eval` once it leaves early access. It takes
      `evals/**/case.yaml`, or `evals/**/prompt.md` plus `graders/*.md`, resolves a
      skills-directory plugin by name, and carries `--ablation with-without` for the
      baseline arm, `--runs` for repetition, `--judge-model`, an HTML report and
      `--max-cost-usd`. That covers by hand exactly what has been built by hand here,
      including the two parts that were hardest, the baseline arm and bounding the
      spend. Every invocation currently answers "`plugin eval` is currently in early
      access" and `eval init --bare` writes nothing, so this is a migration to plan
      rather than to start. What already exists is the input it wants: the fixture,
      the prompts, and the assertions.
- [x] amend the skill body where a run actually failed. One did: a submit command
      rebuilt into `pulls/N/events`, which is not an endpoint, so the body now states
      the command instead of leaving it to be reconstructed, and says how the API
      reports a pending anchor. An earlier amendment about `position` was written and
      then withdrawn, because the failure it answered had not happened. Reading a
      pending comment back shows `position` with `line` and `start_line` null
      whatever the payload sent, and the payload had the anchor right all along. Cost:
      one wasted run and one wrong entry in a scenario file. Worth keeping in view,
      since it is the skill body's own rule about verifying a claim at its source,
      broken while grading a skill built to enforce it.
- [x] Decide the model set deliberately: the models that actually deliver these
      reviews, not the Haiku-Sonnet-Opus trio the docs use as an example. A
      model the skill never runs on is not worth a run.
- [x] Wire the suite into `AGENTS.md`: every edit to a `SKILL.md` runs that
      skill's scenarios before the commit. Without that line a future session
      has no way to know the suite exists, and "regression suite for every later
      edit" stays an intention.

Deliberately not doing: a checker of its own for the mechanical assertions. It was
queued here as the obvious next tool, and `claude plugin eval` shipping graders as
`graders/*.md` with a judge model makes it work to be thrown away. The assertions
written into each scenario file are the specification those graders will want, which is
the part worth having either way.

Store the scenarios in `evals/<skill-name>/`, one file per scenario, at the repo
root rather than inside `skills/`. Everything under a skill directory is surface
the plugin serves, and support files there are for tools and heavy reference,
not tests.

## 2. Tripwire: progressive disclosure

No action yet. The skill body is at 400 lines against a 500-line limit, so the
next substantial addition forces the split.

Deliberately not splitting now: moving rules out of `SKILL.md` in a skill whose
job is enforcing discipline risks them not being read at all. When the split
becomes unavoidable, the section to move is the delivery mechanics for a pending
review, which is reference material rather than a rule, and reference files stay
one level deep from `SKILL.md`.

- [ ] revisit when a skill body crosses roughly 450 lines

## Completed

**Rename the plugin namespace to `myai`.** A skill is invoked as
`/myai:writing-pr-reviews`. The objection raised against the name and set aside:
in a listing where every plugin ships skills for an AI agent, `ai` distinguishes
nothing, since what a namespace carries is whose the skills are rather than what
they are. What the rename cost is the answer to the question the packaging entry
below left open, and it is smaller than that entry assumed: the namespace comes
from the `name` field in `.claude-plugin/plugin.json` alone, so renaming the
symlink was tidiness rather than a requirement, and the rest was eleven mentions
in the prose.

**Validate the skills in CI, and before a commit.** `tools/check.sh` is the single
command: formatting, lints as errors, then the test suite. A commit runs it through
`.githooks/pre-commit`, which each clone points at with `git config core.hooksPath
.githooks`, and `.github/workflows/checks.yml` runs it on every push and pull
request after installing the Rust version `.tool-versions` names, then adds
`claude plugin validate`. One script rather than three copies of the same list,
because a hook and a workflow that drift apart teach a working copy to lie.

The greppable rules went into the suite rather than into shell steps in the
workflow, for the same reason: no machine-local paths, no owner name outside its
two allowed places, and a README skills table that matches `skills/` in both
directions, since a row pointing at a deleted skill is the same drift as a skill
with no row.

The list this task carried, closed:

- [x] the checks themselves, for every `skills/*/SKILL.md`: the frontmatter
      parses, `name` matches the directory name, `description` is present and
      under 1024 characters, and the body is under 500 lines, plus the two rules
      that came out of writing the tests, a description cut short by a YAML
      comment and a directory holding no `SKILL.md`
- [x] a GitHub Actions workflow running that suite
- [x] the greppable rules from `AGENTS.md`: no machine-local paths, and no owner
      name outside the clone URL in `README.md` and the copyright line in
      `LICENSE`
- [x] a check that the README skills table carries a row for every directory
      under `skills/`, and, added after the fact, no row for a skill that is gone
- [x] run `claude plugin validate` in the same workflow, tolerating its expected
      warnings
- [x] update the `AGENTS.md` claim that there was "no build, and no test suite"
      and add the checks to its Layout block
- [x] not on the original list: formatting and lints in the same gate, and
      `.githooks/pre-commit` running it before a commit

Three things the writing taught, none of them foreseen here. The name to look for
has to come out of the copyright line, because a check spelling it out would break
the rule it enforces. It has to be the whole name, because a first name flags this
plan's own record of `paolo-prs-review`. And the fixtures have to assemble
machine-local paths at run time, since a literal one in a test file is a violation
the suite reports against itself, which is the self-specification trap this plan
spotted, one level deeper than it looked. The flake the plan expected did not
materialise either: `claude plugin validate` exits 0 with its warnings, so
tolerating them reads the exit status and matches no text. Worth carrying over
from the original entry: the `name` check is load-bearing rather than cosmetic,
because in a plugin skill `name` supplies the last segment of the command, so a
drifting `name` means the command stops matching the directory it lives in.

**Recover the half of the description YAML was dropping.** The frontmatter did
list the trigger phrasings in both languages, which is why this plan recorded
that as already in line. The loader never saw them: an unquoted `#` in
`"dammi un parere sulla PR #N"` opens a YAML comment, so the value arrived 147
characters long instead of 288, and every phrasing after that point, all the
English ones included, was silently dropped. A `>-` block scalar fixes it without
changing a word of the text. `AGENTS.md` now carries the rule for anyone writing
a description, and the checker reports the shape, so it cannot come back
unnoticed. Worth remembering how it surfaced: every other check on that file
passed, and only parsing the frontmatter as YAML rather than reading its lines
exposed it, which is the whole argument for the checker being a real program.

**Add a LICENSE and a .gitignore.** MIT, with the copyright holder named, which
cost a second exception to the no-personal-names rule: a holder line has to
identify someone for the grant to mean anything, exactly as the clone URL has to
name the owner to be an address. Both exceptions now sit together in `AGENTS.md`.
`.gitignore` covers `.DS_Store` and nothing speculative. The urgency this entry
originally claimed was wrong, and the correction is worth keeping: the repo is
private, so no content was unusable by anyone, and the license is there for
whenever the repo opens rather than to fix a problem that exists today.

**Package the repo as a skills-directory plugin.** Originally queued last, as an
open question between symlinks and a plugin. It moved to the front once
recognisability became a requirement: the plugin namespace supplies it, which lets
each skill keep a clean descriptive name instead of encoding ownership into the
directory name. `.claude-plugin/plugin.json` names the plugin `myai`, one symlink
of the repo root into `~/.claude/skills/` replaces one symlink per skill, and no
marketplace is involved. The reasoning is recorded in `AGENTS.md`; the risk that
plugin discovery would not follow a symlink was tested first and does not
materialise.

**Rename the skill to the conventional form.** `paolo-prs-review`, then
`my-pr-reviews`, now `writing-pr-reviews`, invoked as `/myai:writing-pr-reviews`.
Anthropic recommends the gerund form and `superpowers:writing-skills` asks for
active voice, verb first. `writing-` rather than `reviewing-` because the skill
states in its own opening that it holds conventions for writing and delivering a
review, not advice on how to find defects, and a name promising review methodology
would over-claim.
