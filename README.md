# ai-skills

Skills for AI agents, kept under version control here instead of scattered across
the configuration directories of the individual tools.

## Skills

| Skill | What it covers |
| --- | --- |
| [`my-pr-reviews`](skills/my-pr-reviews/SKILL.md) | How pull request reviews should be written and delivered: pending state, tone, shape of the general comment, permalinks, code suggestions. |

## How it is wired to Claude Code

This repo is a Claude Code plugin named `mine`, declared by
[`.claude-plugin/plugin.json`](.claude-plugin/plugin.json). A single symlink puts
the whole repo under `~/.claude/skills/`, where Claude Code discovers any folder
carrying a plugin manifest and loads it as `mine@skills-dir`. No marketplace and
no install step are involved, and the plugin is read in place rather than copied
into a cache, so the repo working tree stays the only copy of every file: editing
a skill during a session edits the repo directly, and all that remains is
committing it.

```
<repo-path>/          the real directory, holding .claude-plugin/plugin.json
~/.claude/skills/mine   symlink to the above
```

`<repo-path>` is wherever this repo is checked out. It has to be spelled out in
full when creating the symlink, because the link gets resolved from
`~/.claude/skills/` and not from this repo.

Every skill is then invoked under the plugin namespace, so
`skills/my-pr-reviews/` becomes `/mine:my-pr-reviews`. The namespace is
what makes these skills recognisable in a list alongside skills from other
sources.

## Setting up a new machine

Clone the repo, then link it once:

```bash
git clone git@github.com:PaoloLaurenti/ai-skills.git <repo-path>
ln -s <repo-path> ~/.claude/skills/mine
```

The plugin loads on the next session. `claude plugin list` shows it under
"Skills-directory plugins", and `claude plugin validate <repo-path>` checks the
manifest.

## Adding a skill

```bash
mkdir -p skills/<name>
$EDITOR skills/<name>/SKILL.md
```

No symlink work: the plugin already covers every skill under `skills/`.

The frontmatter requires `name` and `description`, and `name` has to match the
directory name, since for a plugin skill it supplies the last segment of the
command. The `description` decides when the skill gets loaded, so it describes the
activation conditions only and not the procedure: if it summarises the workflow,
the agent tends to follow the summary instead of reading the body.

All content in this repo is written in English, and stays free of personal names
and machine-local paths. See [`AGENTS.md`](AGENTS.md).
