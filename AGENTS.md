# AGENTS.md

## What this repo is

Personal skills for AI agents, kept under version control here instead of
scattered across the configuration directories of the individual tools. There is
no application code, no build, and no test suite: the deliverable is the prose in
the skill files themselves.

## Layout

```
.claude-plugin/plugin.json      the plugin manifest, naming the namespace every skill loads under
skills/<skill-name>/SKILL.md    one directory per skill, the skill body lives in SKILL.md
README.md                       what the repo is, how it is wired up, how to add a skill
AGENTS.md                       this file
CLAUDE.md                       an @AGENTS.md include, nothing else
docs/PLAN.md                    queued work, with the reasoning behind each item
LICENSE                         MIT, and one of the two places allowed to name the owner
```

## Language

**All content in this repo is written in English**: skill bodies, code, comments,
commit messages, and documentation alike. This holds whatever language the
conversation happens to be in, and those conversations are often in Italian. A
file authored in another language is a defect to fix, not a style choice to
preserve.

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
  this repo's own clone URL in `README.md`, which is the address the repo
  actually lives at, and the copyright holder line in `LICENSE`, which has to
  identify who holds the copyright for the grant to mean anything. No other URL,
  path, or sentence gets to name anyone.
- **No machine-local paths.** `~/.claude/skills/` is a Claude Code convention and
  belongs. Where this repo happens to be checked out is one machine's choice, so
  it appears as the placeholder `<repo-path>` and the reader substitutes it.

Both rules exist so the content stays true wherever it is read, on another
machine or by someone else.

## How a skill reaches Claude Code

This repo is a Claude Code plugin named `mine`. A single symlink puts the whole
repo under `~/.claude/skills/`, and Claude Code loads any folder there carrying a
`.claude-plugin/plugin.json` as a skills-directory plugin, `mine@skills-dir`. It
is discovered in place rather than copied into a cache, so the repo working tree
stays the only copy of every file: editing a skill during a session edits the repo
directly, and all that remains is committing it.

```
<repo-path>/            the real directory, holding .claude-plugin/plugin.json
~/.claude/skills/mine   symlink to the above
```

The symlink target has to be an absolute path, since the link gets resolved from
`~/.claude/skills/` rather than from this repo. Symlinked discovery is verified,
not assumed: `claude plugin list` reports the plugin as loaded through the link.

Consequences worth knowing before changing any of this:

- **Skills are namespaced.** `skills/<name>/` is invoked as `/mine:<name>`. The
  namespace is deliberate: it groups these skills in a listing that also holds
  skills from other sources.
- **Adding a skill needs no symlink work.** The plugin already covers everything
  under `skills/`. A running session picks up an edit to a `SKILL.md` immediately;
  anything structural, a new or renamed skill directory included, may need
  `/reload-plugins`.
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
activation conditions only, never the procedure. A description that summarises
the workflow makes the agent follow the summary instead of reading the body.
Include the phrasings actually used to ask for the thing, in both English and
Italian, since those are what the description is matched against.

The body is the instructions themselves. It is read by an agent that will act on
it, so it states what to do rather than describing what the skill is about.
