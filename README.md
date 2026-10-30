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

## Installing

```bash
npx skills add https://github.com/PaoloLaurenti/ai-skills -g
```

[`npx skills`](https://skills.sh/) asks which agents to install into, Claude Code,
Codex, GitHub Copilot and many others, and `-g` makes the skills available in every
project rather than the current one. Installed this way each skill carries its bare
name, `reviewing-pull-requests`.

## Updating

```bash
npx skills check     # says whether any installed skill has an update
npx skills update    # installs them
```

An update is whatever reaches the `main` branch here. The CLI sends no notification, so
to hear about one, watch this repository on GitHub with "Watch, Custom, Releases": every
release says what changed for whoever uses the skills.

## Developing the skills

Whoever edits the skills works on a clone, linked into the places agents read skills
from rather than installed, so the working tree stays the only copy of every file: a
skill edited during a session is the skill the next prompt uses, and all that remains
is committing it.

```bash
git clone git@github.com:PaoloLaurenti/ai-skills.git <repo-path>
<repo-path>/tools/link-skills.sh
```

`tools/link-skills.sh` makes what is missing and repairs what is stale, so it is run
again after a skill is added or renamed. It links the whole repo as
`~/.claude/skills/myai`, where Claude Code loads it as the plugin `myai`, declared by
[`.claude-plugin/plugin.json`](.claude-plugin/plugin.json), and invokes each skill as
`/myai:<skill>`. It links each skill into `~/.agents/skills/` for Codex and GitHub
Copilot, which read skills one directory each, under their bare names. It removes a
link there left by a renamed skill, and leaves alone every link and directory that
belongs to something else. And it points git at `.githooks/`, so a commit runs
`tools/check.sh` first.

`claude plugin list` shows the plugin under "Skills-directory plugins", and
`claude plugin validate <repo-path>` checks its manifest.

## Adding a skill

```bash
mkdir -p skills/<name>
$EDITOR skills/<name>/SKILL.md
```

Then run `tools/link-skills.sh`, which gives the new skill its link for Codex and GitHub
Copilot; Claude Code already loads every skill under `skills/` through the plugin.

The frontmatter requires `name` and `description`, and `name` has to match the
directory name, since for a plugin skill it supplies the last segment of the
command. The `description` decides when the skill gets loaded, so it describes the
activation conditions only and not the procedure: if it summarises the workflow,
the agent tends to follow the summary instead of reading the body.

The skill also needs a row in the table at the top of this file, which `tools/check.sh`
enforces, and its scenarios under `evals/<name>/`, run before the skill is offered for
review, as the next section explains.

## Evaluations

`tools/check.sh` checks the shape of a skill, never whether its rules hold. That is what
`evals/<name>/` is for: one file per scenario, each giving a prompt, the rule it puts
under pressure, and the outcome expected, checked by command wherever possible.

Every scenario is run twice, once with the skill and once without it, as a baseline: a
rule that the agent follows anyway adds nothing, and only the pair shows the difference
the skill makes. The results of both runs are recorded in the scenario file, with
quotes from the transcripts.

A run costs model calls, and most suites drive a fixture repo on GitHub, so the scenarios
stay out of `tools/check.sh` and the hook: whoever changes a skill runs them. Each
`evals/<name>/README.md` holds the fixture, the commands and how a run is graded.

All content in this repo is written in English, and stays free of personal names
and machine-local paths. See [`AGENTS.md`](AGENTS.md).
