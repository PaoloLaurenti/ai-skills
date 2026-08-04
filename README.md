# ai-skills

Skills for AI agents, kept under version control here instead of scattered across
the configuration directories of the individual tools.

## Skills

| Skill | What it covers |
| --- | --- |
| [`my-pr-reviews`](skills/my-pr-reviews/SKILL.md) | How pull request reviews should be written and delivered: pending state, tone, shape of the general comment, permalinks, code suggestions. |

## How it is wired to Claude Code

Claude Code loads personal skills from `~/.claude/skills/`. Every skill in this
repo is **symlinked** there, so only one copy of the file exists: editing it
during a session edits the repo working tree directly, and all that remains is
committing it.

```
<repo-path>/skills/my-pr-reviews/   the real directory
~/.claude/skills/my-pr-reviews      symlink to the above
```

`<repo-path>` is wherever this repo is checked out. It has to be spelled out in
full when creating a symlink, because the link gets resolved from
`~/.claude/skills/` and not from this repo.

## Setting up a new machine

Clone this repo, then link each skill into `~/.claude/skills/`:

```bash
git clone git@github.com:PaoloLaurenti/ai-skills.git <repo-path>
ln -s <repo-path>/skills/my-pr-reviews ~/.claude/skills/my-pr-reviews
```

A new Claude Code session should list the skill among the available ones. If it
does not appear, the loader is not following the symlink: the alternative in that
case is making `~/.claude/skills` a repo itself, tracking only your own skills
with an allowlist `.gitignore`.

## Adding a skill

```bash
mkdir -p skills/<name>
$EDITOR skills/<name>/SKILL.md
ln -s <repo-path>/skills/<name> ~/.claude/skills/<name>
```

The frontmatter requires `name` and `description`. The `description` decides when
the skill gets loaded, so it describes the activation conditions only and not the
procedure: if it summarises the workflow, the agent tends to follow the summary
instead of reading the body.

All content in this repo is written in English, and stays free of personal names
and machine-local paths. See [`AGENTS.md`](AGENTS.md).
