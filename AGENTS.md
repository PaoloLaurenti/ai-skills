# AGENTS.md

## What this repo is

Personal skills for AI agents, kept under version control here instead of
scattered across the configuration directories of the individual tools. The
deliverable is the prose in the skill files themselves. The only code is a small
Rust crate under `tools/`, which checks the shape of those files and the rules
this file states about the tree they sit in, never the quality of what they say,
plus the few lines of shell that run it from a hook and from CI.

## Layout

```
.claude-plugin/plugin.json      the plugin manifest, naming the namespace every skill loads under
skills/<skill-name>/SKILL.md    one directory per skill, the skill body lives in SKILL.md
skills/<skill-name>/reference/  material a skill body points at rather than carries
evals/<skill-name>/             the scenarios that test a skill's rules, one file each
README.md                       what the repo is, how it is wired up, how to add a skill
AGENTS.md                       this file
CLAUDE.md                       an @AGENTS.md include, nothing else
docs/PLAN.md                    queued work, with the reasoning behind each item
LICENSE                         MIT, and one of the two places allowed to name the owner
tools/skill-check/              the checks over every SKILL.md and over the tree, as cargo tests
tools/check.sh                  formatting, lints and those tests, in one command
tools/link-skills.sh            links the skills where agents read them, for whoever develops them
.githooks/pre-commit            runs that command before a commit, once a clone points at it
.github/workflows/checks.yml    the same command on every push, plus the plugin validation
.tool-versions                  the Rust version asdf resolves for this repo
.vscode/settings.json           points rust-analyzer at the crate, which is not at the root
```

## Language

**All content in this repo is written in English**: skill bodies, code, comments,
commit messages, and documentation alike. This holds whatever language the
conversation happens to be in, and those conversations are often in Italian. A
file authored in another language is a defect to fix, not a style choice to
preserve.

## Committing

Never commit unasked. Changes stay in the working tree until a request says to
commit them, and what closes a piece of work is a summary of what changed rather
than a commit. Committing on the agent's own initiative removes the reading that
was going to happen before it.

## Commit messages

Imperative mood, opening in lower case, and English like everything else here.
One line is the target. Two or three only when a single line genuinely cannot
carry the change, and never a body paragraph arguing for it: the reasoning
belongs in the code, in the docs, or in a plan file.

Never add a `Co-Authored-By` trailer for Claude, or any other agent attribution.

## No personal names, no machine-local paths

Nothing in this repo names the person it belongs to, and nothing hardcodes a path
that exists on one machine only. Both rules cover every file, skill bodies
included.

- **No personal names.** A skill addresses the agent impersonally: "Use when
  asked for a review", never "Use when <name> asks". Where a sentence has to
  refer to the person making the request, an impersonal construction covers it:
  "the requester", "an explicit request to publish", the passive voice. Two
  places are exceptions, both because naming the owner is what makes them true:
  this repo's own GitHub URL in `README.md`, in the clone and the install
  commands alike, which is the address the repo actually lives at, and the copyright holder line in `LICENSE`, which has to
  identify who holds the copyright for the grant to mean anything. No other URL,
  path, or sentence gets to name anyone.
- **No machine-local paths.** `~/.claude/skills/` is a Claude Code convention and
  belongs. Where this repo happens to be checked out is one machine's choice, so
  it appears as the placeholder `<repo-path>` and the reader substitutes it.
  Editor and tool configuration is no exception, which is the one place this bites
  in practice: a setting that only works as an absolute path belongs in
  `.gitignore` rather than in the tree, and `.vscode/settings.json` is committed
  only because it names its target relatively.

Both rules exist so the content stays true wherever it is read, on another
machine or by someone else.

## How a skill reaches Claude Code

This repo is a Claude Code plugin named `myai`. A single symlink puts the whole
repo under `~/.claude/skills/`, and Claude Code loads any folder there carrying a
`.claude-plugin/plugin.json` as a skills-directory plugin, `myai@skills-dir`. It
is discovered in place rather than copied into a cache, so the repo working tree
stays the only copy of every file: editing a skill during a session edits the repo
directly, and all that remains is committing it.

```
<repo-path>/            the real directory, holding .claude-plugin/plugin.json
~/.claude/skills/myai   symlink to the above
```

The symlink target has to be an absolute path, since the link gets resolved from
`~/.claude/skills/` rather than from this repo. Symlinked discovery is verified,
not assumed: `claude plugin list` reports the plugin as loaded through the link.

The namespace comes from the `name` field in the manifest, not from the name of
the link, which is also verified rather than assumed: with the link renamed and
the manifest left alone, the plugin still loaded as `myai@skills-dir`, and the
link name appeared only as the reported path. Keeping the two equal is tidiness,
so a rename costs that one field plus the mentions in the prose.

Consequences worth knowing before changing any of this:

- **Skills are namespaced.** `skills/<name>/` is invoked as `/myai:<name>`. The
  namespace is deliberate: it groups these skills in a listing that also holds
  skills from other sources.
- **Adding a skill needs no symlink work for Claude Code.** The plugin already covers
  everything under `skills/`. Codex and GitHub Copilot read each skill from its own link
  in `~/.agents/skills/`, so a new or renamed skill needs that link made or moved, as the
  `README.md` setup section shows. A running Claude Code session picks up an edit to
  a `SKILL.md` immediately; anything structural, a new or renamed skill directory
  included, may need `/reload-plugins`.
- **`claude plugin validate <repo-path>` warns that the root `CLAUDE.md` is not
  loaded as plugin context.** That warning is expected and must not be "fixed":
  `CLAUDE.md` exists for agents working *on this repo*, and is nothing the plugin
  ships to its users.

**Why a plugin rather than one symlink per skill:** the namespace, and a rename
that no longer costs an intervention on every machine. Chosen over a marketplace
because a skills-directory plugin needs no marketplace and no install step, which
keeps the working tree as the single copy.

## Writing a SKILL.md

The frontmatter requires `name` and `description`, and `name` has to match the
directory name: for a plugin skill it supplies the last segment of the command,
so a mismatch means the command stops matching the directory it lives in.

The `description` decides when the skill gets loaded, so it describes the
activation conditions only, never the procedure. A description that summarises the
workflow makes the agent follow the summary instead of reading the body. Include
the phrasings actually used to ask for the thing, in both English and Italian,
since those are what the description is matched against. A skill whose main
condition covers a request in any language may carry English phrasings only, once
a scenario shows it firing on an Italian request without them:
`developing-with-tdd` is one, and its scenario 1 is that evidence.

The body is the instructions themselves. It is read by an agent that will act on
it, so it states what to do rather than describing what the skill is about.

**Name another skill of this repo by its bare name** inside a skill body,
`reviewing-pull-requests`, adding the plugin form `myai:reviewing-pull-requests` beside
it where the body asks to load it. The namespace exists only when the repo is loaded as
a Claude Code plugin: Codex, Copilot and an `npx skills` install see the bare name, and
a body naming `myai:` alone sends them looking for a skill they do not have.

**Quote a description that contains a `#`.** In an unquoted YAML scalar a `#`
preceded by a space opens a comment, so everything after it is silently dropped
from the value the loader sees. A description listing a trigger such as `PR #12`
loses that phrase and every phrase after it. A block scalar keeps the text
exactly as written:

```yaml
description: >-
  Use when asked for a review, including phrasings like "review PR #12".
```

**Rules stay in the body, commands may move to `reference/`.** The 500-line limit forces a
split eventually, and what gets extracted decides whether the split is free or costly. A
command is reference material: it is looked up at the moment it is needed, and a body
carrying twenty of them is mostly lookup table. A rule is not, because an agent that never
opens the file never obeys it, and the rules most worth having are the ones an agent under
pressure would rather skip.

So the body keeps the non-negotiables, the red flags and every sentence stating what must
or must not happen, and `reference/<topic>.md` takes the invocations behind them. Two
things make the split safe. The reference file sits one hop from `SKILL.md`, linked from
the section that used to hold its contents, never from another reference file. And the
pointer is written as an instruction with its reason attached, since a footnote gets
skipped: an agent has to know that opening it is part of the job and what goes wrong
otherwise.

## Checking a skill mechanically

Run this over any change to the repo, before it is offered for review:

```bash
tools/check.sh
```

Formatting, then lints as errors, then the test suite. One script, because the
hook and CI run it too and the three should never come to disagree about what
passing means.

A commit runs it as well, once each clone has been pointed at the hook:

```bash
git config core.hooksPath .githooks
```

`.git/hooks` is not versioned, which is why the hook lives in `.githooks/` and
costs that one command per clone. `git commit --no-verify` skips it where there is
a reason to. What it reads is the working tree rather than what is staged, the
usual imprecision of a hook this shape: a fix left unstaged still counts.

The suite is both the gate and the specification: every rule is a named test, and
two of those tests run the checks over this repo itself. On a skill file it covers
the shape only, whether the frontmatter parses, whether `name` matches the
directory, the description limit, the body length, and whether every relative link
in the body resolves to a file that exists. Nothing it reports is about the
quality of the prose.

That last one is what makes a `reference/` file safe to rely on. Before it, moving
or renaming one left the pointer in `SKILL.md` dangling with nothing to catch it,
and the first sign would have been a real review failing to be delivered.

**Its scope is a `SKILL.md`, not the files one points at.** A reference file that
links a second reference file is unchecked, which costs nothing today because none
of them link anything, and would need the check made recursive the day one does.

Two things it has to get right, and both were found by running it rather than by
reasoning about it. A link is only checked when it is relative, since an external
URL names nothing in the tree. And markdown inside a code span or a fenced block is
illustration rather than a pointer: a body about writing markdown quotes markdown,
and this repo's skill both names the shape a link should take, in a code span, and
shows a suggestion block as four backticks wrapping three. A fence tracker that
ignores run length lets that inner fence close the outer one.

The same suite carries the rules this file states about the whole tree: no
machine-local paths, no owner name outside the two places allowed to carry it, and
a `README.md` skills table that matches `skills/` in both directions, so neither a
skill with no row nor a row with no skill survives. The name to look for comes out
of the copyright line, because a check that spelled it out would break the rule it
enforces. Two things follow for anyone editing these
rules. A check matching a bare `/Users/` would flag the sentence stating the rule,
so a path only counts with a directory name after the prefix. And a fixture
holding a literal machine-local path would be a violation in itself, so the tests
assemble those paths at run time.

`.github/workflows/checks.yml` runs that same script on every push and pull
request, and adds the one check that cannot live inside it: `claude plugin
validate`, which needs the CLI. That step reads the exit status, since validation
passes with warnings and matching their text would break on the next CLI release.

The Rust version comes from `.tool-versions`, so asdf resolves it for this
directory with nothing installed globally, and the workflow installs the version
that file names.

## Releasing

`main` is what users get: `npx skills update` installs whatever each skill folder holds
there, so only finished, checked work is merged into it. A GitHub release is how users
hear about it, since watching a repo's releases notifies and a commit does not.

One version covers the whole repo, and it lives in `.claude-plugin/plugin.json` alone.
It is read from the point of view of whoever uses the skills:

- **major**: a user has to change something, because a skill was renamed or removed, or
  the way to work with one changed, such as an opening question added or dropped;
- **minor**: something new that breaks nothing, a new skill or a new rule in one;
- **patch**: a fix that leaves behaviour as it was, wording, a typo, a reference.

The pull request that earns a new version bumps it in `plugin.json`. Once it is merged,
the merge commit gets the tag `vX.Y.Z`, and the release carries a few lines on what
changes for a user. No changelog file: it would be a second copy of the release notes.

## Evaluating a skill

Those checks say nothing about whether a skill's rules hold. That is what
`evals/<skill-name>/` is for: one file per scenario, each carrying the prompt
verbatim and a checklist of assertions, most of them commands rather than
judgement.

**An edit to a `SKILL.md` runs that skill's scenarios before the change is
offered.** A rule nobody exercised is a rule nobody knows is working, and a rule
that used to work can stop through an edit elsewhere in the same file.

They sit outside `tools/check.sh` and the hook on purpose: a run costs a model call
per scenario and reaches GitHub, so it belongs to whoever makes the change rather
than to a gate firing on every commit. The procedure, the fixture and the reset a
rerun needs are in `evals/<skill-name>/README.md`.
