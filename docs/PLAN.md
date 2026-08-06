# Plan

Work queued for this repo, ordered for execution. It comes out of an audit
against Anthropic's [skill authoring best
practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview),
the `superpowers:writing-skills` guidance, and the [Claude Code plugin
docs](https://code.claude.com/docs/en/plugins).

Three things are open, and only the first is work anyone can start today. Task 2 is
blocked on Anthropic and task 3 is a tripwire rather than work. Everything else this
plan has carried is finished and recorded under Completed, the evaluation suite and the
block-versus-prose question it left behind included.

Already in line, so absent from this plan: the `description` field states
triggering conditions without summarising the workflow, trigger phrasings reach
the loader in both languages, the skill body sits under the 500-line limit, and
the Red flags section gives the skill its self-check list. That second item was
recorded here as fine while it was not, which the entry on the truncated
description below explains.

## 1. A scenario for the rule about verifying architectural claims

One of the two sections of the skill body with no scenario against it, and the one worth
covering, found by listing the body's sections beside the five scenarios rather than by a
run failing.

`Verify every factual claim about the architecture` is the rule that a review reads the
file defining a fact before stating it, and never infers it from a crate name, a service
name, or the shape of the diff. Nothing in the suite exercises it. Scenarios 01 to 05
cover pending state, inline versus body, permalinks and line numbers, suggestion blocks,
and the docs cross-check, and the every-run assertions cover the style rules.

Why this one rather than the other uncovered section. The body says of this failure mode
that it "passes every mechanical check on the review, so only reading the source catches
it", which is precisely the shape a suite of mechanical assertions will not notice going
wrong. Three things in the record already point at it. Scenario 01 recorded a run where
an unverified value went inside a committable block and observed that "no assertion here
catches it, because none of them ask whether a suggestion's content was verified"; that
particular question closed on the rerun, but the general one it exposed did not. The
third Opus run on scenario 04 withdrew a false `main()` finding after reading
`README.md`, which is the rule working and went unrecorded by any assertion. And this
repo broke the same rule itself while grading, in the withdrawn `position` amendment.

- [ ] add `evals/writing-pr-reviews/06-architectural-claims.md` and the fixture branch it
      needs, then run it on both models with a baseline arm each

What the fixture has to plant is a name that invites an inference the repo's own files
contradict, which is the `mia-identity` example the body itself uses: a directory or
package called one thing, and a deploy workflow naming a different container or image.
That keeps the rule under test mechanical, as `evals/writing-pr-reviews/README.md`
requires of at least one assertion per scenario: grep the review text for the inferred
name against the real one read out of the workflow file, rather than judging the prose.

Not queued alongside it: a scenario for `Every observation carries a proposal`. It is the
other section without one, it is already covered by a red flag, and no run has produced a
naked observation, so a scenario for it would be answering an imagined failure.

Worth knowing before starting: a new scenario is not work the migration below throws
away. That entry records that what `claude plugin eval` wants is the fixture, the prompts
and the assertions, which is exactly what a scenario file is.

## 2. Move the suite onto `claude plugin eval`

Blocked on Anthropic, not on this repo.

`claude plugin eval` takes `evals/**/case.yaml`, or `evals/**/prompt.md` plus
`graders/*.md`, resolves a skills-directory plugin by name, and carries
`--ablation with-without` for the baseline arm, `--runs` for repetition,
`--judge-model`, an HTML report and `--max-cost-usd`. That is, off the shelf, exactly
what has been built by hand here, including the two parts that were hardest: the
baseline arm and bounding the spend.

- [ ] migrate once it leaves early access. Still gated at CLI 2.1.223, re-probed on
      2026-08-06 on both entry points: `claude plugin eval init --bare` and a run against
      `myai@skills-dir` each answer "`plugin eval` is currently in early access", while
      `--help` prints the full option list. Probe the version rather than the date next
      time, since the help output has already moved once while the gate did not, and a
      version bump is the thing that can change the answer.

Two details in that help output worth carrying into the migration, because both are
decisions this suite made by hand and could lose by accident. `--allow-tools`, with its
`Tool(pattern:*)` syntax, is where the deliberate tool list goes; the eval README argues
at length against reaching for `--dangerously-skip-permissions`, and that argument has to
survive the move. And `--threshold`, which exits non-zero below a score, is what would
turn the `AGENTS.md` obligation to rerun a skill's scenarios after editing it into
something mechanical rather than a promise. The help also confirms in writing that
skills-dir plugins resolve by name and default to the `with-without` ablation, so the
baseline arm this suite built by hand comes free.

What already exists is the input it wants: the fixture, the prompts, and the assertions
written into each scenario file.

Deliberately not doing: a checker of its own for the mechanical assertions. It was
queued here as the obvious next tool, and `claude plugin eval` shipping graders as
`graders/*.md` with a judge model makes it work to be thrown away. The assertions
written into each scenario file are the specification those graders will want, which is
the part worth having either way.

## 3. Tripwire: progressive disclosure

No action yet. The skill body is at 408 lines against a 500-line limit, so the
next substantial addition forces the split.

Deliberately not splitting now: moving rules out of `SKILL.md` in a skill whose
job is enforcing discipline risks them not being read at all. When the split
becomes unavoidable, the section to move is the delivery mechanics for a pending
review, which is reference material rather than a rule, and reference files stay
one level deep from `SKILL.md`.

- [ ] revisit when a skill body crosses roughly 450 lines

## Completed

**Settle the block-versus-prose boundary with one more Opus run of scenario 04.** Run,
graded, and the answer is that no amendment is warranted. The full record sits in
`evals/writing-pr-reviews/04-suggestion-blocks.md`; what closes this entry is that the
run landed on neither outcome the task predicted.

The two fixes in that fixture separated. The three-line collapse got a `suggestion` block
anchored `start_line: 5`, `line: 7`, spanning every replaced line, which is two for two
on Opus and the assertion the scenario exists for. The conditional fix, adding `USER
1000:1000`, got prose, where the first Opus run gave it a block.

**Why no amendment: no run produced a failure to answer.** The body permits both
readings outright, "Skip the suggestion when the fix is ... anything the reader has to
decide before writing" for the prose and "Always reach for one where it makes sense" for
the block, and `superpowers:writing-skills` asks that a rule answer a failure actually
observed. Four runs on that fix produced two compliant treatments and no violation, so a
new sentence would prevent nothing. That argument holds whichever way this run had gone,
which is why it is the one carrying the decision.

The tally is secondary and deliberately not the reason: three prose against one block,
and what it settles is only that the model-level disagreement the task predicted did not
appear, since Opus landed on the Sonnet side. The remaining split is inside Opus, one
each out of two, and a within-model split is not evidence against ambiguity. Treating a
majority as the argument is the shape of the withdrawn `position` amendment, which is
why the record separates the two.

Two things the run gave for free. The submit command came back correct,
`pulls/4/reviews/REVIEW_ID/events`, which is the first evidence that this task's one
amendment to the skill body holds; run 1 had reported `pulls/4/events`, which is not an
endpoint. And a false finding about a missing `main()` guard was caught mid-run against
`README.md` and withdrawn by deleting and recreating the review, which is the
verify-at-source rule working on the skill's own terms.

Worth carrying forward: E7 passed vacuously on this run, since the review text named no
repo file outside the `suggestion` block and so had nothing to link. That is the same
weakness this scenario already records about its own first assertion, an assertion that
can pass or fail without the rule under test being exercised at all.

**Build an evaluation suite for the skill, and run it.** This was the largest gap in
the repo: nothing tested whether the rules hold. It is now five scenarios in
`evals/writing-pr-reviews/`, one file each, carrying the prompt verbatim and a checklist
of assertions that are commands rather than judgement calls, against a sandbox fixture
of one pull request per scenario. Twenty-four graded runs are recorded: five scenarios
and five baselines on each of Opus 5 and Sonnet 5, plus four reruns. Scenarios live at
the repo root rather than under `skills/`, because everything in a skill directory is
surface the plugin serves.

Anthropic asks for at least three evaluations per skill and a run against every model
the skill is meant for. `superpowers:writing-skills` goes further, asking that each rule
answer a failure actually observed without it, which is what the baseline arm is for.
The model set was chosen deliberately: Opus 5, the model these reviews are actually
asked for on, and Sonnet 5, where a lighter session lands. Haiku is out, because a model
the skill never runs on is not worth the calls.

*What the baselines settle.* Delivery is what the skill contributes, and scenario 01
proves it rather than implying it: without the skill, both models submitted a published
`COMMENTED` review, each having first tried `REQUEST_CHANGES` and been refused on an own
pull request, and neither weighed publishing as a decision at all. The other eight
baselines posted nothing and wrote the review into the reply, which leaves inline-versus-body
placement and the suggestion anchor with no baseline evidence, since both need a review
object before they can be broken. Their scenario files say so rather than scoring them
as vindicated. Every baseline broke every style rule, with numbers rather than
impressions, and two produced nearly identical banned verdict lines on two different
models. Permalinks and line numbers came out intermittent rather than absent: one
baseline wrote five links unprompted and pinned two of them to the base commit instead
of the head SHA, correct today and stale the moment either file moves.

*What the two-model set was worth.* The prediction was that the style rules would give
first on Sonnet, since they fight a model's default writing voice. That was wrong: zero
em dashes, zero first or second person, zero markdown headings and English throughout,
across all five Sonnet reviews. Two other rules gave instead, both costing an extra step
at writing time, and both then got a second run. The bare repeated mentions on scenario
03 did not reproduce. The missing `suggestion` block on scenario 04 did, and turned out
to be measuring a finding never made rather than a rule broken, which the third Opus run
recorded in the entry above then settled. Scenario 02's Opus inconsistency, a body restating a fix its
inline comment already carried, did not reproduce on Sonnet either.

*Net effect on the skill body: almost nothing, which is the point.* One amendment was
made, and it answered a real failure: a submit command rebuilt into `pulls/N/events`,
which is not an endpoint, so the body now states the command rather than leaving it to
be reconstructed, and says how the API reports a pending anchor. An earlier amendment
about `position` was written and then withdrawn, because the failure it answered had not
happened: a pending comment reads back with `position` set and `line` null whatever the
payload sent, and the payload had been right all along. Cost, one wasted run and one
wrong entry in a scenario file, and it is worth keeping in view, since it is the skill
body's own rule about verifying a claim at its source, broken while grading a skill
built to enforce it. Three further amendments were considered off single runs and all
three would have been wrong.

*Four defects in the assertions, all found by runs.* Two reported violations on clean
reviews, both counting where they should have been reading: comparing a count of
filenames against a count of links double-counts every link, since the name appears in
the link text and again in the URL. Two failed the other way, passing a real violation
in silence: a head-SHA check written as `grep -c 'blob/main'`, which a base-commit pin
satisfies, now a prefix match of every `blob/<sha>` against `headRefOid`; and a
first-person check listing six forms while "me", "my" and the contractions went through
it. The pattern in all four is an assertion testing the failure imagined while writing
it rather than the one the rule is about.

*Three harness lessons, each of which would have read as a skill failure.* A machine can
scope its git identity to a directory tree, leaving the fixture's temporary clone
without one, so the fixture derives an identity from the authenticated account. A run
has to happen from inside a clone of the fixture, or `gh` resolves the wrong repository
entirely. And a fresh `CLAUDE_CONFIG_DIR` removes the plugin and the credentials
together, since credentials live in the keychain keyed to the real directory, so a
baseline disables the plugin instead and re-enables it from a trap. The grading recipe
also assumed a review object exists, which eight of the ten baselines never created, so
it now says the reply is the text to grade when nothing was posted.

`AGENTS.md` carries the line that makes the suite a regression suite rather than a
one-off: every edit to a `SKILL.md` runs that skill's scenarios before the change is
offered.

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
