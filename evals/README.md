# Writing an eval case

What a case author needs to know about `claude plugin eval` beyond its
[docs](https://code.claude.com/docs/en/plugin-evals.md). Every fact here was measured on
a run, at CLI 2.1.322 to 2.1.329, and the entry of `docs/PLAN.md` that moved
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
  called and their input, the files Claude created, a file's contents after the run, or
  the calls a mock received.
  The state a command would report after the run is out of reach.
- **The trace holds what the tools printed**, so the outcome of a command Claude ran,
  such as a test run, is read from the `trace`, never from what Claude says about it. A
  `trace` regex matches anywhere in the run, though: to read the last run of a command,
  add a lookahead allowing no later failure. The trace is one JSON event a line, and
  what a tool printed sits on a line opening with `{"type":"user"`, Claude's own words on
  one opening with `{"type":"assistant"`, so a pattern reads tool output only by
  anchoring there, as `developing-with-tdd`'s `last-test-run-is-green` does.
- **`tool_used` counts attempts, not results.** A failed or refused call counts like a
  successful one. That proves "Claude tried to commit", never "a commit exists". It
  names one tool exactly, with no alternation: a check on `Write` or `Edit` is two
  graders, or a `trace` regex on `"name":"(?:Write|Edit)"`.
- **A regex on the reply matches the words Claude uses to report a failure too.** One
  looking for a commit message matched the sentence explaining why the commit failed.
  Look for something only success prints.
- **A grader's frontmatter ends at the first three dashes anywhere in the file**, in a
  comment or inside a pattern too. Inside a pattern, the case fails to load with
  "invalid YAML frontmatter". In a comment, it loads and the lines after the comment
  are dropped: a `not_contains` grader then fails every run with "pattern not found",
  which is how `writing-cards`' `fits-on-one-screen` failed unnoticed. Write three dashes
  in a pattern as `-{3}`, and never in a comment, at CLI 2.1.332.
- **A regex minds case unless the grader says `flags: i`**, which CLI 2.1.328 honours
  and earlier versions lacked, and an inline `(?i)` is not, by the docs. The patterns
  written before it name both cases of each letter instead, which works as well.
- **A refused command can stop a run from testing.** A run refused one Bash command took
  it as all of Bash and closed without testing. Where the rule under test does not ask
  for a test run, read the outcome from the files instead.
- **A `Skill` grader without `arm: both` scores nothing once the baseline arm runs**:
  the report marks it "with-only, not scored" in the arm with the skill too, and leaves
  it out of the baseline. Only `--ablation none` scores it. So a case testing whether
  the description fires needs `arm: both`, and so does one testing that it stays quiet,
  with `min: 0` and `max: 0` and the skill's own `input_match`, never "no `Skill` call
  at all", since the plugin's other skills may rightly fire. That grader fails on a
  load, and passes in the baseline arm whatever happens, as no plugin is loaded there,
  so a negative case's Δ says nothing. Measured at CLI 2.1.332.

## The judge

- **An `llm` grader's judge answers PASS or FAIL in one word**, with no reasoning first
  and thinking off; three votes, the majority wins. It reads the final reply unless the
  grader's `focus` says otherwise.
- **Write the criteria from the current `SKILL.md`**, never stricter: a criterion asking
  for a word the skill does not require, or leaving a term open to two readings, fails
  correct replies. Say what counts as present ("anywhere, in any form and any order")
  and what is not a reason to fail, and ask nothing a one-word answer cannot carry, such
  as naming what is missing.
- **A rubric of many conditions fails replies that pass each one.** A judge asked for
  six parts and the absence of three faults failed nearly every correct reply, while ten
  judges asking one condition each passed all of them. Split a rubric into a few judges
  of one concern each, and replay to find which condition a failing one trips on.
- **The judge is told where the text came from**, as "Agent output (last_message)" or
  "Agent output (file reply.txt)", and nothing else differs: the criterion, the text,
  three votes. That label alone moves a reply near the criterion's line. The same reply,
  identical to the character, passed twelve votes of twelve read from a file and four of
  twelve read as the last message. A replay therefore keeps the grader's own focus.
- **To find out why a judge failed a reply, replay it under the same focus.** A
  throwaway case on haiku whose scaffold writes the reply into `reply.txt`. For a
  `focus` on a file, a copy of the grader pointed at that file. For the last message,
  a prompt asking to read `reply.txt` and answer with its exact contents, and a copy of
  the grader as it is: every run of eight came back identical to the character, which
  the report's `evidence`, the text each judge saw, confirms. Run it four times or so,
  since one run's three votes can land either way. A `claude -p` replay of the judge's
  prompt is no proxy.

## The sandbox

- **Every run is isolated**: a temporary home, and no user settings, hooks,
  `CLAUDE.md`, memory or other plugins, unless the case loads one, as below. The
  baseline arm runs the same way without the skill.
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
- **The run's home is `/private/tmp/e-XXXX/home`, and the workspace is its `cwd`.**
  The scaffold sees the same `$HOME`, so it can place a file the skill expects under
  the home. A run reads there once `Read(//private/tmp/e-*/home/**)` is granted, and
  writes into a directory of the home once `Edit` is granted on it, the same pattern
  followed by that directory, such as `.local/state/reviews/**`.
- **A `Write` into `~/.claude/` or into the workspace's `.git/` is refused whatever the
  grant**, at CLI 2.1.328. A file a skill keeps there cannot be written in a run. A
  program writing there through Bash, such as `tools/fake-gh`, is not affected.
- **A file grader reads inside the workspace only**: a path with `..` or an absolute one
  is refused as escaping the sandbox. What a run wrote elsewhere is read from the
  `Write` call in the trace.
- **A skill's reference files are refused unless `Read` is granted on the plugin**, as
  they sit beside the skill, outside the workspace. `tools/eval.sh` grants it on the copy
  it evaluates.

## A second plugin beside the skill

A skill that follows another plugin's skill where it is installed, such as
`writing-cards` with `humanizer` and `grilling`, is tested on that path by a case loading
both. `docs/PLAN.md`, entry 11, records the probes behind each point, at CLI 2.1.332.

- **A case's `plugins` key loads several plugins**, paths relative to the case
  directory: `plugins: ["../../..", "../../../.eval-plugins/humanizer"]`, the first entry
  naming the plugin under test.
- **Only from inside the plugin under test**: an entry outside it, the installed copy
  under `~/.claude/plugins/` included, fails the case at load. So a case names
  `.eval-plugins/<plugin-name>`, and `tools/eval.sh` copies the plugin of that name
  installed on this machine there, inside the copy it evaluates, at the version
  installed. Nothing of it is committed.
- **A plugin that is not installed fails the cases naming it at load**, and the command
  exits 1; the cases of other skills are not affected.
- **The baseline arm loads no plugin at all**, the second one included.
- **Prove the second skill ran from the trace**, `Launching skill: <plugin>:<skill>`,
  which only a successful call prints: `tool_used` passes on `Unknown skill` too.

## Mocks of an MCP server

A skill that works through an MCP server, such as `writing-cards` with a tracker, runs
against a mock of it that the command starts itself. `docs/PLAN.md`, entry 10, records
the probes behind each point, at CLI 2.1.332.

- **A case's own `mocks/<server>/` directory is enough, for a server the plugin does not
  declare too.** The run sees its tools as `mcp__<server>__<tool>`, behind `ToolSearch`,
  with no grant needed. Never use `evals/mocks/`, which the command would apply to every
  case of every skill here.
- **`mocks` must be a real directory**, which the command checks. Cases sharing a mock
  link theirs to one under `fixture/`, and `tools/eval.sh` copies through the link.
- **`_tools.json` gives the tools their description and schema**, as a saved
  `tools/list` answer.
- **A `type: agent` mock remembers the run.** The judge model plays the server from the
  body's instructions, with every earlier call of the run in view, so a card created
  shows up in a later read. Its answers vary in fields the body leaves open, so a grader
  reads only the fields the body fixes, such as an id it numbers. A `_server.md` answering
  several tools lists them in `tools: [...]`, or the case fails to load. A `fixed` mock
  answers the same every time, templated from the call with `{{input.<field>}}`.
- **A mock answers in the baseline arm too, and in a resumed case**, where `mock_calls`
  holds the new turn's calls only.
- **`mock_calls` is one JSON line per call**, `{"tool":"mcp__<server>__<tool>","input":{…},
  "output":"…","verdict":"ok"}`, in the order made, and its graders score in both arms.
  The tool is the full name. `not_contains` passes on a run with no call at all. To read
  a write's input and not its answer, stop at `"output":`, as `writing-cards`' graders do
  with `"input":\{(?:(?!"output":)[^\n])*`.

## The stand-in for `gh`

The pull request suites run against `tools/fake-gh`, a Rust crate whose binary is named
`gh`, built and put first on `PATH` by `tools/eval.sh` under its own name, since a run
that sees `fake-gh` in the path of the `gh` it calls takes itself to be in a test.

- **It answers about one pull request**, the one the workspace describes in
  `.git/fake-gh/pull-request.json`, looked for from the current directory up, and saves
  there what a write changes, so a later read sees it. A grader reads that file for the
  state a run left, such as a review still `PENDING`.
- **It logs every call** in `.git/fake-gh/calls.jsonl`, one JSON object per line: its
  `args`, and as `input` the payload a review is created from. A field read from a file,
  `-F body=@reply.md`, is logged with the file's contents, as `gh` sends it. A grader
  reads the review a run wrote from there, with none of the fixture's own text around it.
- **A call it does not know is logged as `"unhandled"`**, and answered with an error.
  Every case requires none, so a skill reaching for a new call fails a case instead of
  passing on a wrong answer. Adding a call is test-first, in `tools/fake-gh/tests/`.
- **Its answers come from the real `gh`, never from memory.** Each suite's
  `fixture/photograph.sh` resets its pull requests on GitHub and photographs them with
  `tools/fake-gh/photograph.sh`, which keeps what `gh pr view` answers with every field
  and as text, and scrubs the account out. `tools/fake-gh/record.sh` records the answers
  the stand-in must give as GitHub does, its errors included, into
  `tools/fake-gh/recorded/`, which `tests/recorded.rs` replays. Both write on GitHub,
  so both run on request only.
- **A photograph ties the histories to it.** A reset posts the planted comments again
  under new ids, which a recorded conversation names, so photographing a suite again
  means recording its histories again.
- **The commit hook checks it once a day**: `tools/fake-gh/check-against-github.sh`
  records again into a temporary directory and replays, failing when GitHub or `gh`
  now answers otherwise. It skips with a warning where the owner's credentials or
  fixture repo are out of reach.
- **A scaffold may write through it**, as `reviewing-pull-requests`' does to open a
  draft that GitHub would not photograph. It runs as the caller, with `tools/eval.sh`'s
  `PATH`, so it checks first that the `gh` it finds is the stand-in, since the real one
  would write to GitHub, and empties the log after.

## Launching

`tools/eval.sh <skill-name> [options]` is the one way to launch. It evaluates a
temporary copy of the files git sees, uncommitted edits included, since the command
refuses a plugin holding a file with two names and cargo's build output is full of hard
links. The copy follows symbolic links, so a case's `mocks` linked to a
shared one arrives as a real directory. It puts the toolchain on `PATH`, fills in the plugin's directory in the
histories, passes the fixed options, and writes the report to `evals/results/`.

- **Every run is kept**, as `--keep-temp` does, in `/private/tmp/e-XXXX/`: its home and
  workspace under `sealed/`, the stand-in's log included, and its trace under `out/`,
  which the report's `tracePath` names. The command leaves `sealed/` with no
  permissions at all, so open it with `chmod u+rwx` first. A failing grader is explained by
  reading what the run wrote, so it is read there rather than paid for again; macOS
  clears that directory on its own.
- **Options after the skill override its defaults**, since the last value of a repeated
  option wins. While writing a case, run it alone and cheap:
  `--model haiku --runs 1 --case <case-name>`.
- **`--case` takes a glob without character classes**: `0[345]-*` matches nothing.
- **The threshold reads the arm with the skill only.** A case whose baseline scores 0.2
  still passes, so a rule the model follows unaided does not fail the suite.

## Reading a score

Each point was read off the reports of real runs, at CLI 2.1.327 and 2.1.328: the
`aggregate-result.json`, the HTML report and the table the command prints at the end.

- **A run's score is the share of its graders that passed**, each counted by its
  `weight`, 1 unless the grader sets one. `walking-through-and-reviewing-pull-requests`'
  `02-step-one-turn2`, on 2026-11-19, has eight graders; each of its three runs failed
  `step-one-only` alone, and scored 7 of 8, 0.875.
- **A case's score is the mean of its runs' scores**: 0.88 for that case.
- **PASS% is the share of runs that passed every grader**, the "perfect runs" of the
  HTML report: 0% for that case, since no run passed all eight. The two numbers tell
  different things. `developing-with-tdd`'s `01-opening-questions-turn1`, on 2026-11-18,
  scored 0.87 with runs of 1.0, 0.8 and 0.8, a PASS% of 33%: a score near 0.9 can mean
  that every run fell short or that one run in three was perfect, and only PASS% tells
  them apart.
- **The terminal's table shows PASS% only when no case of the run had a baseline arm**,
  in the columns `SCORE PASS% RUNS`. As soon as one case had it, every row takes
  `WITH W/OUT Δ RUNS` instead, a later turn run with one arm included, with dashes for
  the baseline and Δ: the score of each arm, their difference, and the runs of both arms
  together, 6 for three runs per arm. The HTML report shows the share of perfect runs
  for every arm.
- **A case passes when its score reaches the threshold, 1.0 by default**, which
  `tools/eval.sh` leaves as it is. At 1.0 a case passes only if every run passed every
  grader, so passing the threshold and a PASS% of 100% are the same thing.
- **The suite's numbers sum up its cases.** The HTML report's suite score is the mean of
  the cases' scores, its "perfect runs" the runs where every grader passed, and the mean
  Δ covers only the cases that ran a baseline arm. The TDD suite on 2026-11-18: twelve
  cases of three runs each, nine at 1.0, a suite score of 97.6%, perfect runs 86.1%, 31
  runs of 36, and a mean Δ of +0.39 over seven cases, since the five later turns run one
  arm.
- **Three runs out of three proves less than it seems.** The runs are a sample of what
  the skill does, and a perfect sample of three is compatible, at 95% confidence, with a
  true rate of perfect runs as low as 29%; ten out of ten raises that floor to 69%. The
  floor for n out of n is 0.025 raised to 1/n, the exact two-sided interval. So a case at
  1.00 over three runs says the rule usually holds, not that it always does.
- **A score is only as true as its graders.** Each of them is a classifier, PASS or FAIL
  on a reply, and can be wrong both ways; `docs/PLAN.md`, entry 16, measures how often.

## Later-turn cases

- **A resumed case runs one arm, with the skill, and its graders see the new turn
  only**: a `Skill`, `tool_used` or `trace` grader finds nothing from the recorded
  turns. "The skill loaded" cannot be asked of a later turn.
- **A history is rebuilt from a run, never edited by hand.** Run the earlier turn's
  case with `--keep-temp --ablation none`, then rebuild the conversation from its trace
  with `tools/history-from-trace.sh`, which also appends a turn to an earlier history
  for a third turn. A session recorded with `claude -p` carries the recording machine's
  environment and language instead.
- **A background agent's report wakes the session inside the turn.** An agent
  dispatched with `run_in_background` that finishes inside the turn makes the session
  reply once more, and that reply, not the one before it, is the turn's `last_message`.
  The report reaches the trace only as a `task_notification` event, which
  `tools/history-from-trace.sh` turns into the message a session receives for it, so a
  resumed case finds the report and does not dispatch the agent again.
- **Paths in a history are relative or placeholders.** Claude copies a recorded path as
  it is, so a placeholder for the workspace sends its edits into a directory of that
  literal name. The script makes workspace paths relative, writes `<home>`, `<tmp>` and
  `<owner>` for the rest, and `<plugin-root>` for the plugin's directory, which
  `tools/eval.sh` fills in. `tools/check.sh` proves nothing machine-local is left.
- **Files the recorded turn wrote** go in the case's own directory, for its scaffold to
  copy over the fixture.
- **A history records the skill as it was.** After an edit to `SKILL.md` that changes
  what an earlier turn says, record it again.
