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
| [`walking-through-and-reviewing-pull-requests`](skills/walking-through-and-reviewing-pull-requests/SKILL.md) | **You** review someone else's pull request, and the agent teaches it to you first. It explains the change one concept at a time, at the level of detail you choose, and suggests review comments along the way, written and delivered by the rules of `reviewing-pull-requests`, which it loads. You approve each one; at the end they are saved on GitHub as a draft review that only you can publish. | "walk me through PR 94, I have to review it" |
| [`addressing-pull-request-reviews`](skills/addressing-pull-request-reviews/SKILL.md) | Answering the review comments left on your own pull request. The agent goes through them one at a time, checks each one against the code, tells you whether it agrees, and drafts a reply. Nothing is posted until you say so. | "help me address the feedback on PR 82" |
| [`developing-with-tdd`](skills/developing-with-tdd/SKILL.md) | Developing code with Test-Driven Development, the tests driving the design. The agent first proposes the list of behaviours to test and waits for your approval, then builds them one at a time: a failing test, the least code that makes it pass, a refactoring. It stops and asks whenever a decision about the design is yours. | "let's TDD this". It also offers itself when you ask to add or change a feature or to fix a bug, and does nothing until you accept. |
| [`reviewing-pull-requests`](skills/reviewing-pull-requests/SKILL.md) | **The agent** reviews a pull request for you. It looks for problems, writes short comments with links to the code and suggested fixes, and leaves the review on GitHub as a draft for you to publish; you can also go through its findings one at a time first. It also follows up once the author has replied. | "review PR 16" |

## Using the skills

### Requirements

- [Node.js](https://nodejs.org/), for `npx skills`.
- The three pull request skills act on GitHub through the
  [GitHub CLI](https://cli.github.com/), `gh`, signed in with `gh auth login` to an
  account that can read the repository and comment on it.
- They also load two skills from [superpowers](https://github.com/obra/superpowers):
  `requesting-code-review`, which finds the problems in a change, and
  `receiving-code-review`, which checks a comment before agreeing with it. Install
  superpowers in the same agents.

`developing-with-tdd` needs nothing beyond the agent.

### Installing

```bash
npx skills add https://github.com/PaoloLaurenti/ai-skills -g
```

[`npx skills`](https://skills.sh/) asks which agents to install into, Claude Code,
Codex, GitHub Copilot and many others, and `-g` makes the skills available in every
project rather than the current one.

Installing every skill is the safe choice: `walking-through-and-reviewing-pull-requests`
loads `reviewing-pull-requests` for its rules on writing comments, so the first does not
work fully without the second.

### Updating

```bash
npx skills check     # says whether any installed skill has an update
npx skills update    # installs them
```

Every change to a skill reaches you as a release: whenever the `main` branch changes a
skill, it is tagged and released with a few lines on what changed. The CLI sends no
notification, so to hear about one, watch this repository on GitHub and pick "Releases"
under "Custom". `npx skills check` and `npx skills update` work the same either way.

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
Code already loads it through the plugin.

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
  machine only;
- `tools/check.sh` passes before a commit, which the hook set by `tools/link-skills.sh`
  enforces;
- a change to a skill comes with its scenarios under `evals/<name>/`, run with and
  without the skill, and their results recorded;
- a change to a skill bumps the version in `.claude-plugin/plugin.json`, as "Releasing"
  in `AGENTS.md` explains.

## License

MIT, see [LICENSE](LICENSE).
