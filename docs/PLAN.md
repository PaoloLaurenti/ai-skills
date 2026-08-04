# Plan

Work queued for this repo, ordered for execution. It comes out of an audit
against Anthropic's [skill authoring best
practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview),
the `superpowers:writing-skills` guidance, and the [Claude Code plugin
docs](https://code.claude.com/docs/en/plugins).

Task 1 carries the most value by a wide margin. Everything after it is small.

Already in line, so absent from this plan: the `description` field states
triggering conditions without summarising the workflow, trigger phrasings are
present in both languages, the skill body sits under the 500-line limit, and the
Red flags section gives the skill its self-check list.

## 1. Write evaluation scenarios for the skill

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

## 2. Add a LICENSE and a .gitignore

Cheap, and the first one has real consequences: a public repo without a license
leaves the content unusable by anyone else, whatever the README invites.

- [ ] pick a license and add it at the repo root
- [ ] add a `.gitignore` covering `.DS_Store`

## 3. Validate the skills in CI

The rules worth enforcing mechanically should not consume attention in review.

- [ ] a GitHub Actions workflow checking, for every `skills/*/SKILL.md`: the
      frontmatter parses, `name` matches the directory name, `description` is
      present and under 1024 characters, and the body is under 500 lines
- [ ] run `claude plugin validate` in the same workflow, tolerating its expected
      warning about the root `CLAUDE.md`

The `name` check is load-bearing rather than cosmetic, though not for the reason
first assumed here. In a plain personal skill, `name` is only a display label and
the command comes from the directory. In a plugin skill, which is what these are
now, `name` supplies the last segment of the command, so a drifting `name` means
the command stops matching the directory it lives in.

## 4. Tripwire: progressive disclosure

No action yet. The skill body is at 392 lines against a 500-line limit, so the
next substantial addition forces the split.

Deliberately not splitting now: moving rules out of `SKILL.md` in a skill whose
job is enforcing discipline risks them not being read at all. When the split
becomes unavoidable, the section to move is the delivery mechanics for a pending
review, which is reference material rather than a rule, and reference files stay
one level deep from `SKILL.md`.

- [ ] revisit when a skill body crosses roughly 450 lines

## Completed

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
