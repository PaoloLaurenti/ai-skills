# Plan

Work queued for this repo, ordered for execution. It comes out of an audit
against Anthropic's [skill authoring best
practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview),
the `superpowers:writing-skills` guidance, and the [Claude Code plugin
docs](https://code.claude.com/docs/en/plugins).

Task 2 carries the most value by a wide margin, and task 1 still runs first: the
checks are mechanical, self-contained, and testable offline, while the evaluation
scenarios cannot start until a fixture repo exists. Task 3 is a tripwire rather
than work.

Already in line, so absent from this plan: the `description` field states
triggering conditions without summarising the workflow, trigger phrasings reach
the loader in both languages, the skill body sits under the 500-line limit, and
the Red flags section gives the skill its self-check list. That second item was
recorded here as fine while it was not, which the entry on the truncated
description below explains.

## 1. Validate the skills in CI

The rules worth enforcing mechanically should not consume attention in review.
Each check is an assertion that fails before it passes, so the work goes
test-first.

The checks over `SKILL.md` exist and run locally. What is left is the job that
runs them on every push, plus the repo-wide rules that belong in that same job.

- [x] the checks themselves, for every `skills/*/SKILL.md`: the frontmatter
      parses, `name` matches the directory name, `description` is present and
      under 1024 characters, and the body is under 500 lines. They live in
      `tools/skill-check/` as a cargo test suite, so the suite is both the gate
      and the specification, and one test runs the checks over this repo's own
      `skills/`. Two further rules came out of writing the tests: a description
      cut short by a YAML comment, which the shipped skill was, and a directory
      holding no `SKILL.md`
- [ ] a GitHub Actions workflow running that suite
- [ ] the greppable rules from `AGENTS.md`, in the same workflow: no
      machine-local paths (`/Users/`, `/home/`) and no owner name outside the two
      places allowed to carry it, the clone URL in `README.md` and the copyright
      line in `LICENSE`. This plan file spells those patterns out in order to
      describe them, so the path check has to match an actual path, a prefix
      followed by a directory, rather than the bare string, or the first thing it
      flags is its own specification.
- [ ] a check that the README skills table carries a row for every directory
      under `skills/`
- [ ] run `claude plugin validate` in the same workflow, tolerating its expected
      warning about the root `CLAUDE.md`. The CLI has to be installed on the
      runner, and tolerating the warning means matching its text, which can
      drift between CLI versions: if this job flakes anywhere, it flakes here.
- [x] update the `AGENTS.md` claim that there is "no build, and no test suite",
      which this task makes false, and add the checks to its Layout block

The `name` check is load-bearing rather than cosmetic, though not for the reason
first assumed here. In a plain personal skill, `name` is only a display label and
the command comes from the directory. In a plugin skill, which is what these are
now, `name` supplies the last segment of the command, so a drifting `name` means
the command stops matching the directory it lives in.

## 2. Write evaluation scenarios for the skill

The largest gap. Nothing in the repo tests whether the rules hold.

Anthropic asks for at least three evaluations per skill, plus a run against every
model the skill is meant for. `superpowers:writing-skills` goes further: run the
scenario against a subagent *without* the skill first, record the rationalisations
verbatim, and only then write or amend the guidance, so each rule answers a
failure that was actually observed.

Three decisions make the task executable at all, so they come before any
scenario gets written:

- [ ] **A fixture the scenarios run against.** Every scenario needs a real pull
      request: a diff, a head SHA, a repo where `gh api` can create a pending
      review. Set up a sandbox repo with one open PR per scenario, plus a reset
      step between runs, since GitHub allows one pending review per user per PR
      and every run leaves one behind.
- [ ] **A run procedure and pass criteria.** Each scenario is a file carrying
      the prompt and a checklist of observable assertions, because almost every
      expected outcome checks mechanically: review state `PENDING` via `gh api`,
      the finding inline rather than in the body, permalinks containing the head
      SHA, the `suggestion` anchor covering every replaced line. A suite graded
      by eye is not a regression suite.
- [ ] **A clean baseline environment.** The symlink makes the skill loadable in
      every session on this machine, and a scenario prompt is written to match
      the skill's description. A baseline run needs the plugin genuinely absent:
      disabled, or a session under an alternate configuration directory. Verify
      the skill did not load before trusting the run as a baseline.

The skill runs to 388 lines of rules, and the ones most worth testing are those
an agent under pressure will negotiate with:

- [ ] **Pending state.** A scenario where the review reads as finished and the
      request is ambiguous about publishing. Expected: the review stays `PENDING`
      and the submit command gets reported back.
- [ ] **Inline versus general comment.** A finding whose file sits in the diff.
      Expected: it lands inline, not as a body bullet.
- [ ] **Permalinks and line numbers.** A review naming several repo files.
      Expected: every mention is a link pinned to the head SHA, with line numbers
      checked against that ref rather than recalled.
- [ ] **Suggestion blocks.** A fix that is a concrete edit to diffed lines.
      Expected: a `suggestion` block whose anchor covers every replaced line, and
      no block at all for a pure deletion.
- [ ] **Docs cross-check.** The skill's own text calls this step mandatory and
      easy to skip, which is exactly what a baseline exposes. A diff that
      contradicts a claim in the fixture repo's `README.md`. Expected: the
      finding names the file, links it, and quotes the stale claim.
- [ ] **Style rules as assertions on every scenario, not a scenario of their
      own.** Em dashes, first and second person, English: the rules a model
      violates most naturally, because they fight its default writing style, and
      each one greps. Every review any scenario produces gets these checks.
- [ ] Record a baseline run without the skill for each scenario, so the value of
      each rule is visible.
- [ ] Decide the model set deliberately: the models that actually deliver these
      reviews, not the Haiku-Sonnet-Opus trio the docs use as an example. A
      model the skill never runs on is not worth a run.
- [ ] Wire the suite into `AGENTS.md`: every edit to a `SKILL.md` runs that
      skill's scenarios before the commit. Without that line a future session
      has no way to know the suite exists, and "regression suite for every later
      edit" stays an intention.

Store the scenarios in `evals/<skill-name>/`, one file per scenario, at the repo
root rather than inside `skills/`. Everything under a skill directory is surface
the plugin serves, and support files there are for tools and heavy reference,
not tests.

## 3. Tripwire: progressive disclosure

No action yet. The skill body is at 388 lines against a 500-line limit, so the
next substantial addition forces the split.

Deliberately not splitting now: moving rules out of `SKILL.md` in a skill whose
job is enforcing discipline risks them not being read at all. When the split
becomes unavoidable, the section to move is the delivery mechanics for a pending
review, which is reference material rather than a rule, and reference files stay
one level deep from `SKILL.md`.

- [ ] revisit when a skill body crosses roughly 450 lines

## Completed

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
directory name. `.claude-plugin/plugin.json` names the plugin `mine`, one symlink
of the repo root into `~/.claude/skills/` replaces one symlink per skill, and no
marketplace is involved. The reasoning is recorded in `AGENTS.md`; the risk that
plugin discovery would not follow a symlink was tested first and does not
materialise.

**Rename the skill to the conventional form.** `paolo-prs-review`, then
`my-pr-reviews`, now `writing-pr-reviews`, invoked as `/mine:writing-pr-reviews`.
Anthropic recommends the gerund form and `superpowers:writing-skills` asks for
active voice, verb first. `writing-` rather than `reviewing-` because the skill
states in its own opening that it holds conventions for writing and delivering a
review, not advice on how to find defects, and a name promising review methodology
would over-claim.
