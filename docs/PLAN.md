# Plan

Work queued for this repo, ordered for execution. It comes out of an audit
against Anthropic's [skill authoring best
practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview),
the `superpowers:writing-skills` guidance, and the [Claude Code plugin
docs](https://code.claude.com/docs/en/plugins).

Task 2 carries the most value. Task 1 comes first only because it is cheap and
doing it later means rewriting the scenarios task 2 produces.

Already in line, so absent from this plan: the `description` field states
triggering conditions without summarising the workflow, trigger phrasings are
present in both languages, the skill body sits under the 500-line limit, and the
Red flags section gives the skill its self-check list.

## 1. Rename the skill to the conventional form

**State:** `my-pr-reviews`. **Target:** `reviewing-pull-requests`.

Anthropic recommends the gerund form for skill names; `superpowers:writing-skills`
asks for active voice, verb first, naming what the skill does. A possessive noun
phrase is the weakest of the three forms both guides list. Two concrete reasons
beyond the convention: in a flat namespace shared with dozens of other skills,
`my-` carries no information a selection decision can use, and the possessive
reads against the impersonal rule in `AGENTS.md`.

- [ ] `git mv` the skill directory
- [ ] update `name` in the frontmatter to match the directory
- [ ] repoint the symlink under `~/.claude/skills/`
- [ ] update the references in `README.md` and `AGENTS.md`
- [ ] confirm a running session lists the new name

**Decision needed:** the current name was a deliberate choice, so this task is a
proposal, not a settled item. Nothing downstream depends on it except the naming
of the scenario files in task 2.

## 2. Write evaluation scenarios for the skill

The largest gap. Nothing in the repo tests whether the rules hold.

Anthropic asks for at least three evaluations per skill, plus a run against every
model the skill is meant for, since a body that works for Opus can be too terse
for Haiku. `superpowers:writing-skills` goes further: run the scenario against a
subagent *without* the skill first, record the rationalisations verbatim, and only
then write or amend the guidance, so each rule answers a failure that was actually
observed.

The skill is 392 lines of rules, and the rules most worth testing are the ones an
agent under pressure will negotiate with:

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
- [ ] Record a baseline run without the skill for each scenario, so the value of
      each rule is visible.
- [ ] Run the set against Haiku, Sonnet, and Opus.

Store the scenarios in the repo, one file per scenario, next to the skill they
test. They are the regression suite for every later edit to the skill body.

## 3. Add a LICENSE and a .gitignore

Cheap, and the first one has real consequences: a public repo without a license
leaves the content unusable by anyone else, whatever the README invites.

- [ ] pick a license and add it at the repo root
- [ ] add a `.gitignore` covering `.DS_Store`

## 4. Validate the skills in CI

The rules worth enforcing mechanically should not consume attention in review.

- [ ] a GitHub Actions workflow checking, for every `skills/*/SKILL.md`: the
      frontmatter parses, `name` matches the directory name, `description` is
      present and under 1024 characters, and the body is under 500 lines
- [ ] run `claude plugin validate` against the repo once task 5 lands

## 5. Decide on packaging: symlinks or a plugin

Not a defect to fix. Symlinking into `~/.claude/skills/` is the documented
standalone route, described as the right one for personal workflows, and it keeps
the short invocation name.

What a plugin buys, per the plugin docs: setup on a new machine becomes one
`/plugin marketplace add` instead of an `ln -s` per skill, plus versioning and
updates. What it costs: skills become namespaced, so the invocation turns into
`/ai-skills:<skill>`.

- [ ] decide, weighing the short name against one-command setup
- [ ] if a plugin: add `.claude-plugin/plugin.json` at the repo root with `name`,
      `description`, and `version`; keep the skills where they are, since
      `skills/<name>/SKILL.md` is already the layout a plugin expects
- [ ] if a plugin: add the marketplace manifest so the repo is installable from
      git, and replace the symlink instructions in `README.md`
- [ ] if symlinks: record the decision and the reason in `AGENTS.md`, so it does
      not get revisited from scratch

## 6. Tripwire: progressive disclosure

No action yet. The skill body is at 392 lines against a 500-line limit, so the
next substantial addition forces the split.

Deliberately not splitting now: moving rules out of `SKILL.md` in a skill whose
job is enforcing discipline risks them not being read at all. When the split
becomes unavoidable, the section to move is the delivery mechanics for a pending
review, which is reference material rather than a rule, and reference files stay
one level deep from `SKILL.md`.

- [ ] revisit when a skill body crosses roughly 450 lines
