# AGENTS.md

## What this repo is

Personal skills for AI agents, kept under version control here instead of
scattered across the configuration directories of the individual tools. There is
no application code, no build, and no test suite: the deliverable is the prose in
the skill files themselves.

## Layout

```
skills/<skill-name>/SKILL.md    one directory per skill, the skill body lives in SKILL.md
README.md                       what the repo is, how it is wired up, how to add a skill
CLAUDE.md                       an @AGENTS.md include, nothing else
```

## Language

**All content in this repo is written in English**: skill bodies, code, comments,
commit messages, and documentation alike. This holds whatever language the
conversation happens to be in, and those conversations are often in Italian. A
file authored in another language is a defect to fix, not a style choice to
preserve.

## Commit messages

Imperative mood, and English like everything else here. One line is the target.
Two or three only when a single line genuinely cannot carry the change, and never
a body paragraph arguing for it: the reasoning belongs in the code, in the docs,
or in a plan file.

Never add a `Co-Authored-By` trailer for Claude, or any other agent attribution.

## No personal names, no machine-local paths

Nothing in this repo names the person it belongs to, and nothing hardcodes a path
that exists on one machine only. Both rules cover every file, skill bodies
included.

- **No personal names.** A skill addresses the agent impersonally: "Use when
  asked for a review", never "Use when <name> asks". Where a sentence has to
  refer to the person making the request, an impersonal construction covers it:
  "the requester", "an explicit request to publish", the passive voice. The one
  exception is this repo's own clone URL in `README.md`, which carries the owner
  name because that is the address the repo actually lives at. No other URL, path,
  or sentence gets to name anyone.
- **No machine-local paths.** `~/.claude/skills/` is a Claude Code convention and
  belongs. Where this repo happens to be checked out is one machine's choice, so
  it appears as the placeholder `<repo-path>` and the reader substitutes it.

Both rules exist so the content stays true wherever it is read, on another
machine or by someone else.

## How a skill reaches Claude Code

Claude Code loads personal skills from `~/.claude/skills/`. Each skill directory
in this repo is symlinked there, so only one copy of every file exists: editing a
skill during a session edits the repo working tree directly, and all that remains
is committing it.

```
<repo-path>/skills/my-pr-reviews/   the real directory
~/.claude/skills/my-pr-reviews      symlink to the above
```

The symlink target has to be an absolute path, since the link gets resolved from
`~/.claude/skills/` rather than from this repo.

A newly added skill needs its symlink created before any session can see it.
Once the symlink exists, a running session picks up a new or renamed skill
without restarting.

## Writing a SKILL.md

The frontmatter requires `name` and `description`.

The `description` decides when the skill gets loaded, so it describes the
activation conditions only, never the procedure. A description that summarises
the workflow makes the agent follow the summary instead of reading the body.
Include the phrasings actually used to ask for the thing, in both English and
Italian, since those are what the description is matched against.

The body is the instructions themselves. It is read by an agent that will act on
it, so it states what to do rather than describing what the skill is about.
