# Evaluations for `writing-pr-reviews`

What the skill is supposed to do, written as scenarios that can be run and graded
without judgement calls. One file per scenario, numbered. This file holds what
they share: the fixture they run against, how a run is driven, and the assertions
every run gets.

The skill body is a list of rules an agent under pressure will negotiate with. A
scenario earns its place by covering one of those, and it is only worth trusting
once the same scenario has been run **without** the skill, so the value of the
rule is visible rather than assumed.

## The fixture

Every scenario needs a real pull request: a diff, a head SHA, and a repo where
`gh api` can create a pending review. `fixture/setup.sh` creates that sandbox: one
repo, one branch and one open pull request per scenario, with file contents fixed
so line numbers in the assertions stay true.

The repo on `main` also carries `.github/workflows/deploy.yml`, which exists for scenario
06 alone: everything else in the fixture calls the service `example-service`, and that
file is the only place saying the deployed image and rollout target are called
`platform-api`. Two constraints hold anything added to it. It triggers on a branch that
does not exist, so pushing to the fixture never runs it, and it names no port, no base
image and no entrypoint, so it cannot become a second finding inside the five scenarios
that predate it. It was added to the live fixture after those five were recorded, which is
why their results mention no workflow.

```bash
evals/writing-pr-reviews/fixture/setup.sh          # creates the repo and the PRs
evals/writing-pr-reviews/fixture/reset.sh          # clears pending reviews between runs
```

The repo defaults to `<the authenticated login>/ai-skills-eval-fixture`, private,
and both scripts take `EVAL_FIXTURE_REPO` to point somewhere else. The owner is
read from `gh api user -q .login` at run time rather than written down, so nothing
here names anyone.

**The reset step is not optional.** GitHub allows one pending review per user per
pull request, so every run leaves one behind and the next `POST` fails while it is
there. A failed second run reads exactly like a skill failure, which is the trap
`reset.sh` exists to remove. Run it before every run, including the first.

Scenarios refer to their pull request by branch name, so no generated state is
kept in this repo. Resolve the number when running:

```bash
number=$(gh pr list --repo "$EVAL_FIXTURE_REPO" --head docs-cross-check --json number -q '.[0].number')
```

## Running a scenario

Each scenario file carries its prompt verbatim. The prompt is written the way the
thing is actually asked for, which is also what the skill's `description` is
matched against, so a scenario doubles as a check that the skill loads at all.

Two scripts do this, and they are the shortest path to a run:

```bash
evals/writing-pr-reviews/run.sh      opus "dammi un parere sulla PR #5"
evals/writing-pr-reviews/baseline.sh opus "dammi un parere sulla PR #5"
```

`run.sh` clones the fixture, clears the previous pending review, runs the prompt, retries
while the API is overloaded, and prints where it left the output, the log and the clone.
`baseline.sh` takes the same two arguments and hands them straight to `run.sh` with the
plugin disabled, so a baseline and the run it is compared against differ in one thing
only. Both leave the clone in place, because the payload the skill posted from is in it.

What each of them does, and why, in case either needs changing.

A run happens from inside a clone of the fixture, the way a real review does, so
that `gh` resolves the repo from the remote and the skill reads the repo's own docs
from the working tree it is reviewing:

```bash
git clone "git@github.com:$EVAL_FIXTURE_REPO.git" /tmp/fixture-clone
evals/writing-pr-reviews/fixture/reset.sh
cd /tmp/fixture-clone
claude -p "$prompt" --model "$model" \
    --allowedTools Read Write Grep Glob "Bash(gh:*)" "Bash(find:*)" "Bash(git:*)"
```

The tool list is deliberate rather than a bypass. `--dangerously-skip-permissions`
would be shorter, and the `gh` token reaches every repo the account can see, so a
sandbox fixture is not a reason to widen what a headless run may do. `Write` is on
the list because the delivery mechanism needs it: the review is posted from a JSON
file, per the skill body.

Confirm the skill loaded before reading anything into the result. A run where it
never loaded is a description failure, not a rule failure, and the two need
different fixes.

**Expect to retry.** Several runs answered `API Error: 529 Overloaded` within seconds
of starting, one of them four times in a row across twenty minutes. Wrap the run in a
loop that retries while the output carries 529 and stops on anything else, so a real
failure is not buried under a backoff. Stopping on anything else is the part that
matters: the run that died with "Not logged in" looked identical from the outside, and
a loop that retried everything would have hidden it behind ten attempts.

## Grading

The assertions below are commands. Set `R` to the fixture repo and `N` to the pull
request number, then read the review the run produced:

```bash
id=$(gh api "repos/$R/pulls/$N/reviews" -q '.[] | select(.state=="PENDING") | .id')
body=$(gh api "repos/$R/pulls/$N/reviews/$id" -q .body)
inline=$(gh api "repos/$R/pulls/$N/reviews/$id/comments" -q '.[].body')
text="$body
$inline"
```

`$body` is the general comment, `$inline` every inline comment, and `$text` the
two together, which is what the style assertions read.

**When nothing was posted, the reply is the text to grade.** Eight of the ten baselines
recorded so far created no review at all, which leaves `$id` empty and the whole chain
above with nothing in it. Read `$out` instead and run every style assertion over that,
since the review is in there. E1 and E2 then record an absence rather than reporting a
failure, and any assertion about where a finding sits, inline or in the body, is
ungradeable rather than failed: there is no body and no inline comment to place it in. The inline comments come
from an endpoint of their own: a pending review's comments are not in
`repos/$R/pulls/$N/comments`, which is why the assertion below expects that one to
be empty.

**What that endpoint does not tell you: where a comment is anchored.** A pending
review's comments come back with `position` and `original_position` populated and
`line`, `start_line` and `side` all `null`, no matter which of them the payload
carried. Anything about the anchor has to be graded from the payload the run wrote,
which sits in the clone the run happened in:

```bash
cat "$clone"/review*.json
```

This is worth stating plainly because the `null` reads like evidence. Grading it as
evidence once produced a confident, wrong failure, and an amendment to the skill body
answering a failure that had not happened. The rule the skill body sets for reviews
holds for grading them too: read the thing that defines the claim, rather than the
thing that looks conclusive.

### Assertions on every run

These are the rules a model violates most naturally, because they fight its
default writing style. Every review any scenario produces gets all of them.

| | Assertion | Command |
|---|---|---|
| E1 | Exactly one review exists, and it is `PENDING` | `gh api "repos/$R/pulls/$N/reviews" -q '[.[] \| select(.state=="PENDING")] \| length'` is `1` |
| E2 | Nothing leaked as a published comment | `gh api "repos/$R/pulls/$N/comments" -q length` is `0` |
| E3 | No em dash anywhere in the review | `printf '%s' "$text" \| grep -c '—'` is `0` |
| E4 | No first or second person | `printf '%s' "$text" \| grep -niE "\b(I\|I'd\|I'm\|we\|us\|our\|you\|your\|me\|my)\b"` finds nothing outside quoted material |
| E5 | Written in English | by eye, and it is the one assertion here that is |
| E6 | Every repo file named in the text is a markdown link | `printf '%s' "$text" \| perl -pe 's/\[[^]]*\]\([^)]*\)//g' \| grep -onE '(README\.md\|AGENTS\.md\|Dockerfile\|default\.toml\|deploy\.yml)'`, then read what survives |
| E7 | Every permalink is pinned to the head SHA | `printf '%s' "$text" \| grep -oE 'blob/[0-9a-f]{7,40}'` and compare against `gh pr view "$N" --repo "$R" --json headRefOid -q .headRefOid` |
| E8 | No `event` was sent, which E1 already proves | E1 |

**E4 as first written listed six forms**, `I` through `your`, and missed `me`, `my` and
the contractions. The Opus baseline on scenario 01 carried all three of "I'd", "me" and
"my" and would have passed. The pattern above is the corrected one, and it is the fourth
assertion defect a run has exposed.

**E4 is English-only, which makes it useless on a baseline.** Scenario 06 measured this:
on its two baselines the pattern returned two hits and zero, and both numbers are wrong.
The two are the Italian article `i`, folded into `\bI\b` by `grep -i`; the zero is a reply
opening "Ho guardato" and "non la mergerei", first person throughout, invisible to a
pattern looking for pronouns in a language that puts first person in the verb. This costs
nothing on a graded review, which E5 already requires to be English, and everything on a
baseline, where the reply comes back in the language of the prompt. Every baseline in this
suite is Italian, so E4 has never actually caught first person in one: the prose in each
scenario file reporting it came from a person reading the text. Treat E4 on a baseline as
unrun rather than as passed, and grade that row by eye.

E4 will also match an `I` or a `you` inside a claim quoted from the fixture's own
files. That is why it reads "outside quoted material": the check finds candidates,
and a person confirms. E6 is the same shape, since deciding what counts as a file
reference needs the three exceptions the skill body lists, a bare extension,
literal syntax quoted from a file, and a runtime path inside a container. Scenario 03
added a fourth in practice: a filename inside replacement wording drafted for a
document, where a link would corrupt the text being handed over.

Both are stated as "strip, then read" rather than as counts on purpose. Two counts
compared against each other is how E6 was first written, and it reported violations
on a clean review, because every link carries its filename twice, once in the text
and once in the URL.

### Per-scenario assertions

Each scenario file lists its own, marked **M** where a command settles it and
**E** where a person has to look. A scenario whose assertions are all **E** is not
a regression test, and should be rewritten until at least the rule under test is
mechanical.

## Baselines

A rule is worth its place in the skill body only if an agent without the skill
gets it wrong. Record that first, and record the reasoning verbatim, because the
rationalisation is what the rule has to answer.

A baseline run needs the plugin genuinely absent, and the symlink loads it in every
session on this machine. Disable it for the run and put it back afterwards:

```bash
claude plugin disable myai@skills-dir
claude plugin list                    # expect no skills-directory plugin loaded
claude -p "$(the prompt)" --model "$model" --allowedTools ...
claude plugin enable myai@skills-dir
```

Re-enable from a trap rather than a following line, so a run that is killed does not
leave the skill switched off:

```bash
trap 'claude plugin enable myai@skills-dir' EXIT INT TERM
```

**Not with an empty `CLAUDE_CONFIG_DIR`.** That does make the plugin absent, and it
also makes the CLI unauthenticated: `claude plugin list` answers fine without
credentials, so the check passes, and then the run dies with "Not logged in". The
credentials are in the keychain, keyed to the real configuration directory. This is
worth stating because the empty-directory route was written here first, with only the
half that works verified.

Verify the plugin is absent every time before trusting a baseline, and keep that
verification in the log next to the result.

Each scenario file keeps its baseline under a heading of its own, holding what the
run produced and the reasoning it gave, quoted rather than summarised.

## The model set

Every scenario runs against both of these, and a rule is settled only when both
agree:

| Model | `--model` | Why it is in the set |
|---|---|---|
| Opus 5 | `opus` | The model these reviews are actually asked for on. A rule that fails here fails in practice. |
| Sonnet 5 | `sonnet` | Where a lighter session lands. The style rules are the first to give on a less capable model, since they fight its default writing voice. |

Haiku 4.5 is deliberately out: no review is delivered on it, and a model the skill
never runs on is not worth the calls.
