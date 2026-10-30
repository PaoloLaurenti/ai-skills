# ai-skills

Skills for AI agents, kept under version control here instead of scattered across
the configuration directories of the individual tools.

## Skills

| Skill | What it is for | Ask for it with |
| --- | --- | --- |
| [`walking-through-and-reviewing-pull-requests`](skills/walking-through-and-reviewing-pull-requests/SKILL.md) | **You** review someone else's pull request, and the agent teaches it to you first. It explains the change one concept at a time, at the level of detail you choose, and suggests review comments along the way, written and delivered by the rules of `reviewing-pull-requests`, which it loads. You approve each one; at the end they are saved on GitHub as a draft review that only you can publish. | "walk me through PR 94, I have to review it" |
| [`addressing-pull-request-reviews`](skills/addressing-pull-request-reviews/SKILL.md) | Answering the review comments left on your own pull request. The agent goes through them one at a time, checks each one against the code, tells you whether it agrees, and drafts a reply. Nothing is posted until you say so. | "help me address the feedback on PR 82" |
| [`developing-with-tdd`](skills/developing-with-tdd/SKILL.md) | Developing code with Test-Driven Development, the tests driving the design. The agent first proposes the list of behaviours to test and waits for your approval, then builds them one at a time: a failing test, the least code that makes it pass, a refactoring. It stops and asks whenever a decision about the design is yours. | "let's TDD this", or any request to add or change a feature or fix a bug |
| [`reviewing-pull-requests`](skills/reviewing-pull-requests/SKILL.md) | **The agent** reviews a pull request for you. It looks for problems, writes short comments with links to the code and suggested fixes, and leaves the review on GitHub as a draft for you to publish; you can also go through its findings one at a time first. It also follows up once the author has replied. | "review PR 16" |

## How it is wired to Claude Code

This repo is a Claude Code plugin named `myai`, declared by
[`.claude-plugin/plugin.json`](.claude-plugin/plugin.json). A single symlink puts
the whole repo under `~/.claude/skills/`, where Claude Code discovers any folder
carrying a plugin manifest and loads it as `myai@skills-dir`. No marketplace and
no install step are involved, and the plugin is read in place rather than copied
into a cache, so the repo working tree stays the only copy of every file: editing
a skill during a session edits the repo directly, and all that remains is
committing it.

```
<repo-path>/            the real directory, holding .claude-plugin/plugin.json
~/.claude/skills/myai   symlink to the above
```

`<repo-path>` is wherever this repo is checked out. It has to be spelled out in
full when creating the symlink, because the link gets resolved from
`~/.claude/skills/` and not from this repo.

Every skill is then invoked under the plugin namespace, so
`skills/reviewing-pull-requests/` becomes `/myai:reviewing-pull-requests`. The namespace is
what makes these skills recognisable in a list alongside skills from other
sources.

## Setting up a new machine

Clone the repo, then link it once:

```bash
git clone git@github.com:PaoloLaurenti/ai-skills.git <repo-path>
ln -s <repo-path> ~/.claude/skills/myai
git config core.hooksPath .githooks
```

The plugin loads on the next session. `claude plugin list` shows it under
"Skills-directory plugins", and `claude plugin validate <repo-path>` checks the
manifest.

The last line makes a commit run `tools/check.sh` first: formatting, lints and the
checks over every skill. `.git/hooks` is not versioned, so each clone has to point
at `.githooks/` itself.

Codex and GitHub Copilot know nothing of Claude Code plugins. They read skills from
`~/.agents/skills/`, one directory each, so every skill gets a link of its own there:

```bash
for skill in <repo-path>/skills/*/; do ln -s "${skill%/}" ~/.agents/skills/; done
```

A link rather than a copy keeps the repo the only copy there too. Those agents see each
skill under its bare name, `reviewing-pull-requests` rather than
`myai:reviewing-pull-requests`. Nothing is linked into `~/.claude/skills/`, where the
plugin already loads every skill and a second link would list each one twice.

## Adding a skill

```bash
mkdir -p skills/<name>
$EDITOR skills/<name>/SKILL.md
```

No symlink work for Claude Code: the plugin already covers every skill under `skills/`.
For Codex and GitHub Copilot the new skill needs its link in `~/.agents/skills/`, as in
the setup above.

The frontmatter requires `name` and `description`, and `name` has to match the
directory name, since for a plugin skill it supplies the last segment of the
command. The `description` decides when the skill gets loaded, so it describes the
activation conditions only and not the procedure: if it summarises the workflow,
the agent tends to follow the summary instead of reading the body.

All content in this repo is written in English, and stays free of personal names
and machine-local paths. See [`AGENTS.md`](AGENTS.md).
