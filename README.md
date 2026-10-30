# ai-skills

Agent skills for everyday software development: reviewing pull requests, answering the
reviews you receive, and developing code with Test-Driven Development.

A skill is a set of instructions that an AI coding agent loads when a request matches it,
so the agent works the same way every time instead of improvising. These work with Claude
Code, Codex, GitHub Copilot and the other agents the [`npx skills`](https://skills.sh/)
installer supports. You trigger them by asking in plain words, as the table below shows;
each one stops and asks you before any step that is yours to decide.

## Skills

| Skill | What it is for | Ask for it with |
| --- | --- | --- |
| [`reviewing-pull-requests`](skills/reviewing-pull-requests/SKILL.md) | When you want **the agent** to review a pull request for you. It looks for problems, writes short comments with links to the code and suggested fixes, and leaves the review on GitHub as a draft for you to publish. It can show you its findings one at a time before the draft is made, and it follows up once the author has replied. | "review PR 16"; "review PR 16, one point at a time"; "the author replied to my review on PR 16" |
| [`walking-through-and-reviewing-pull-requests`](skills/walking-through-and-reviewing-pull-requests/SKILL.md) | When **you** have to review someone else's pull request and want to understand it first. The agent explains the change one concept at a time, at the level of detail you choose, and suggests review comments along the way. You approve each one; at the end they are saved on GitHub as a draft review that only you can publish. | "walk me through PR 94, I have to review it" |
| [`addressing-pull-request-reviews`](skills/addressing-pull-request-reviews/SKILL.md) | When your own pull request has been reviewed and you have to answer. The agent goes through the comments one at a time, checks each one against the code, tells you whether it agrees, and drafts a reply and the change. Nothing is posted until you say so. | "help me address the feedback on PR 82" |
| [`developing-with-tdd`](skills/developing-with-tdd/SKILL.md) | When you develop code with Test-Driven Development, the tests driving the design. The agent first proposes the list of behaviours to test and waits for your approval, then builds them one at a time: a failing test, the least code that makes it pass, a refactoring. It stops and asks whenever a decision about the design is yours. | "let's TDD this". Asked to add a feature or fix a bug without a word about TDD, it first asks whether you want to work in TDD, and does nothing else until you answer. |

## Using the skills

### Requirements

- [Node.js](https://nodejs.org/) 22.20 or later, for `npx skills`.
- The three pull request skills act on GitHub through the
  [GitHub CLI](https://cli.github.com/), `gh`, signed in with `gh auth login` to an
  account that can read the repository and comment on it.
- They also load two skills from [superpowers](https://github.com/obra/superpowers):
  `requesting-code-review`, which finds the problems in a change, and
  `receiving-code-review`, which checks a comment before agreeing with it. Install
  superpowers in the same agents, as
  [its installation guide](https://github.com/obra/superpowers#installation) explains
  for each one.

`developing-with-tdd` needs nothing beyond the agent.

### Installing

```bash
npx skills add https://github.com/PaoloLaurenti/ai-skills -g
```

[`npx skills`](https://skills.sh/) asks which agents to install into, Claude Code,
Codex, GitHub Copilot and many others, and `-g` makes the skills available in every
project rather than the current one.

You use a skill by asking in plain words, as the table shows. To call one by name, it is
the name in the table, with no prefix: `/reviewing-pull-requests` in Claude Code.

Installing every skill is the safe choice: `walking-through-and-reviewing-pull-requests`
loads `reviewing-pull-requests` for its rules on writing comments, so the first does not
work fully without the second. To install only some of them, name each with `--skill`,
as in `npx skills add https://github.com/PaoloLaurenti/ai-skills --skill developing-with-tdd -g`.

### What they do on your behalf

- **On GitHub**, the review skills only ever create draft reviews:
  `reviewing-pull-requests` and `walking-through-and-reviewing-pull-requests` never
  submit or publish one unless you ask in so many words. `addressing-pull-request-reviews`
  commits, pushes and posts its replies only after a final yes to a list of exactly what
  it will do.
- **In your code**, `developing-with-tdd` edits files but never commits on its own.
- **Language**: ask in any language and they answer in it; what they write on GitHub,
  comments and replies, is always in English.
- **Tested** with Claude Code. Codex, GitHub Copilot and the other agents read the same
  files, but the skills have not been run there.

### Updating

```bash
npx skills check        # says whether any installed skill has an update
npx skills update -g    # installs them, for the skills installed with -g
```

Every change to a skill reaches you as a release: whenever the `main` branch changes a
skill, it is tagged and released with a few lines on what changed. The CLI sends no
notification, so to hear about one, watch this repository on GitHub and pick "Releases"
under "Custom". `npx skills check` and `npx skills update -g` work whether you watch the releases or not.

### Uninstalling

```bash
npx skills remove -g
```

It lists the skills installed globally and removes the ones you pick; `-g` matches the
`-g` they were installed with.

## Developing the skills

Whoever edits the skills works on a clone, linked into the places agents read skills
from rather than installed, so the working tree stays the only copy of every file: a
skill edited during a session is the skill the next prompt uses, and all that remains
is committing it.

### Setting up

```bash
git clone https://github.com/PaoloLaurenti/ai-skills.git <repo-path>
<repo-path>/tools/link-skills.sh
```

`<repo-path>` is wherever you want the clone. Remove an installation made with
`npx skills` first, with `npx skills remove -g`: the script leaves its copies alone and
reports them as conflicts.

`tools/link-skills.sh` creates the links that are missing and removes the ones a
renamed skill left behind, so it is run again after a skill is added or renamed. It
links the whole repo as `~/.claude/skills/myai`, so Claude Code loads it as a plugin
called `myai`, declared by [`.claude-plugin/plugin.json`](.claude-plugin/plugin.json),
and each skill gets the prefix `myai:`, as in `/myai:reviewing-pull-requests`. It links
each skill into `~/.agents/skills/` for Codex and GitHub Copilot, which use the skill's
name with no prefix. It leaves alone every link and directory there that belongs to
something else. And it points git at `.githooks/`, so a commit runs `tools/check.sh`
first.

`claude plugin list` shows the plugin under the heading "Skills-directory plugins", and
`claude plugin validate <repo-path>` checks its manifest.

### Adding a skill

From the root of your clone:

```bash
mkdir -p skills/<name>
$EDITOR skills/<name>/SKILL.md
tools/link-skills.sh
```

`tools/link-skills.sh` gives the new skill its link for Codex and GitHub Copilot; Claude
Code already loads it through the plugin. A `SKILL.md` starts from this shape:

```markdown
---
name: <name>
description: >-
  Use when <the situations that call for it>, including phrasings like
  "<how people ask for it>".
---

# <Title>

<What the agent must do, written as rules it can follow.>
```

The frontmatter requires `name` and `description`, and `name` has to match the
directory name, since for a plugin skill it supplies the last segment of the
command. The `description` decides when the skill gets loaded, so it describes the
activation conditions only and not the procedure: if it summarises the workflow,
the agent tends to follow the summary instead of reading the body.

The skill also needs a row in the table at the top of this file, which `tools/check.sh`
enforces, and its scenarios under `evals/<name>/`, run before the change is proposed, as
the next section explains.

### Evaluations

`tools/check.sh` checks the shape of a skill, never whether its rules hold. That is what
`evals/<name>/` is for: one file per scenario, each giving a prompt, the rule it puts
under pressure, and the outcome expected, checked by command wherever possible.

Every scenario is run twice, once with the skill and once without it, as a baseline: a
rule that the agent follows anyway adds nothing, and only the pair shows the difference
the skill makes. The results of both runs are recorded in the scenario file, with
quotes from the transcripts.

A run costs model calls, and most suites drive a test repository on GitHub, with pull
requests prepared for the scenarios, so the scenarios stay out of `tools/check.sh` and
the commit hook: whoever changes a skill runs them. Each
`evals/<name>/README.md` holds the fixture, the commands and how a run is graded.

### Contributing

Pull requests are welcome. Every rule a change follows is in [`AGENTS.md`](AGENTS.md);
these are the ones a first change meets:

- everything is written in English, and names no person and no path that exists on one
  machine only; the repository's own GitHub URL, in the commands above, is the one
  exception;
- `tools/check.sh` passes before a commit, which the hook set by `tools/link-skills.sh`
  enforces;
- a change to a skill comes with its scenarios under `evals/<name>/`, run with and
  without the skill, and their results recorded;
- a change to a skill bumps the version in `.claude-plugin/plugin.json`; once it is
  merged, the maintainer tags it and publishes the release, as "Releasing" in `AGENTS.md`
  explains.

## License

MIT, see [LICENSE](LICENSE).
