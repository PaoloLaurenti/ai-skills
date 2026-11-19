# Writing an eval case

What a case author needs to know about `claude plugin eval` beyond its
[docs](https://code.claude.com/docs/en/plugin-evals.md). Every fact here was measured on
a run, at CLI 2.1.322 to 2.1.328, and the entry of `docs/PLAN.md` that moved
`developing-with-tdd` onto the command records how. A later CLI may change any of them:
when a run contradicts one, probe it in a throwaway plugin, with haiku, one run and a low
`--max-cost-usd`, and correct it here.

## A case

- **A case is a directory** under `evals/<skill-name>/` holding `case.yaml`, a scaffold
  script building the workspace, and `graders/`, one file per check. Its `tags` name the
  skill, which is what `tools/eval.sh` selects on; a tag matching no case fails the run,
  so a misspelt skill never passes as an empty suite.
- **One prompt per case.** A `turns:` key is rejected. A later turn is a case of its own
  that resumes a recorded conversation through `context.history_file`, as below.
- **A scaffold runs where it lives**, with the run's empty workspace as the current
  directory, so it reaches a generator shared by the suite through
  `$(dirname "$0")/../fixture/`.
- **Set `execution.max_turns`.** It defaults to 10, and reaching it is a run error that
  fails the case wherever Claude had got to. `developing-with-tdd` sets 40, which costs
  nothing when Claude stops on its own.
- **Tell the run that shell loops are refused.** The sandbox refuses any `for` loop
  whatever the grants, Opus opens most runs with one, and a refused loop can make it
  conclude it cannot run anything at all. A line in `execution.append_system_prompt`
  saying so, and naming the command that does run, keeps the run going. A `CLAUDE.md`
  in the workspace is not read. The line is repeated in every case, as the command has
  no option to set it once. A command opening with a variable assignment, such as
  `R=owner/repo; gh api …`, matches no `Bash(gh:*)` grant either and is refused the same
  way, which the line names too where a skill's commands use variables.

## What a grader can read

- **No grader runs a command.** A grader reads the final reply, the trace, the tools
  called and their input, the files Claude created, or a file's contents after the run.
  The state a command would report after the run is out of reach.
- **The trace holds what the tools printed**, so the outcome of a command Claude ran,
  such as a test run, is read from the `trace`, never from what Claude says about it. A
  `trace` regex matches anywhere in the run, though: to read the last run of a command,
  add a lookahead allowing no later failure.
- **`tool_used` counts attempts, not results.** A failed or refused call counts like a
  successful one. That proves "Claude tried to commit", never "a commit exists". It
  names one tool exactly, with no alternation: a check on `Write` or `Edit` is two
  graders, or a `trace` regex on `"name":"(?:Write|Edit)"`.
- **A regex on the reply matches the words Claude uses to report a failure too.** One
  looking for a commit message matched the sentence explaining why the commit failed.
  Look for something only success prints.
- **A regex minds case unless the grader says `flags: i`**, which CLI 2.1.328 honours
  and earlier versions lacked, and an inline `(?i)` is not, by the docs. The patterns
  written before it name both cases of each letter instead, which works as well.
- **A refused command can stop a run from testing.** A run refused one Bash command took
  it as all of Bash and closed without testing. Where the rule under test does not ask
  for a test run, read the outcome from the files instead.
- **A `Skill` grader is an indicator under the baseline arm**, unscored, unless it says
  `arm: both`. A case testing whether the description fires needs it.

## The judge

- **An `llm` grader's judge answers PASS or FAIL in one word**, with no reasoning first
  and thinking off; three votes, the majority wins. It reads the final reply unless the
  grader's `focus` says otherwise.
- **Write the criteria from the current `SKILL.md`**, never stricter: a criterion asking
  for a word the skill does not require, or leaving a term open to two readings, fails
  correct replies. Say what counts as present ("anywhere, in any form and any order")
  and what is not a reason to fail, and ask nothing a one-word answer cannot carry, such
  as naming what is missing.
- **To find out why a judge failed a reply, replay it from a file.** A throwaway case,
  on haiku with one run, whose scaffold writes the reply into `reply.txt`, graded by a
  copy of the grader with `focus: { source: file, path: reply.txt }`. A `claude -p`
  replay of the judge's prompt is no proxy, and asking haiku to repeat the reply word
  for word sometimes alters it.

## The sandbox

- **Every run is isolated**: a temporary home, and no user settings, hooks,
  `CLAUDE.md`, memory or other plugins. The baseline arm runs the same way without the
  skill.
- **Bash reaches no network but granted domains**, and the credentials in the real home
  are out of reach, which is why a suite driving a pull request through `gh` runs against
  `tools/fake-gh` instead, put first on `PATH` by `tools/eval.sh`.
- **The sandbox reads no directory of the home but those on `PATH`.** A toolchain
  installed there, such as asdf's, needs its `bin` and, for `rustc`, its `lib` on
  `PATH`; `tools/eval.sh` puts them there.
- **A directory outside the home is skipped on `PATH`**, so the executable found is the
  next one, as if the first were not there.
- **Apple's git runs only through `tools/eval.sh`.** `/usr/bin/git` is a shim asking
  `xcrun` for the real git, and `xcrun` needs a cache the sandbox denies, so every command
  exits 72. Putting the real git's directory first on `PATH` does not help, being outside
  the home; `tools/eval.sh` writes three scripts handing `git`, `git-receive-pack` and
  `git-upload-pack` to it into `evals/results/.bin/`, which git ignores, and puts
  that first instead. A push needs a remote the sandbox reaches, such as a bare
  repository the scaffold creates inside the workspace, and a grader reads what arrived
  from a file of it, such as `origin.git/refs/heads/main`.
- **A skill's reference files are refused unless `Read` is granted on the plugin**, as
  they sit beside the skill, outside the workspace. `tools/eval.sh` grants it on the copy
  it evaluates.

## Launching

`tools/eval.sh <skill-name> [options]` is the one way to launch. It evaluates a
temporary copy of the files git sees, uncommitted edits included, since the command
refuses a plugin holding a file with two names and cargo's build output is full of hard
links. It puts the toolchain on `PATH`, fills in the plugin's directory in the
histories, passes the fixed options, and writes the report to `evals/results/`.

- **Options after the skill override its defaults**, since the last value of a repeated
  option wins. While writing a case, run it alone and cheap:
  `--model haiku --runs 1 --case <case-name>`.
- **`--case` takes a glob without character classes**: `0[345]-*` matches nothing.
- **The threshold reads the arm with the skill only.** A case whose baseline scores 0.2
  still passes, so a rule the model follows unaided does not fail the suite.

## Later-turn cases

- **A resumed case runs one arm, with the skill, and its graders see the new turn
  only**: a `Skill`, `tool_used` or `trace` grader finds nothing from the recorded
  turns. "The skill loaded" cannot be asked of a later turn.
- **A history is rebuilt from a run, never edited by hand.** Run the earlier turn's
  case with `--keep-temp --ablation none`, then rebuild the conversation from its trace
  with `tools/history-from-trace.sh`, which also appends a turn to an earlier history
  for a third turn. A session recorded with `claude -p` carries the recording machine's
  environment and language instead.
- **Paths in a history are relative or placeholders.** Claude copies a recorded path as
  it is, so a placeholder for the workspace sends its edits into a directory of that
  literal name. The script makes workspace paths relative, writes `<home>`, `<tmp>` and
  `<owner>` for the rest, and `<plugin-root>` for the plugin's directory, which
  `tools/eval.sh` fills in. `tools/check.sh` proves nothing machine-local is left.
- **Files the recorded turn wrote** go in the case's own directory, for its scaffold to
  copy over the fixture.
- **A history records the skill as it was.** After an edit to `SKILL.md` that changes
  what an earlier turn says, record it again.
