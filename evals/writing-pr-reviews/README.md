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
two together, which is what the style assertions read. The inline comments come
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
| E4 | No first or second person | `printf '%s' "$text" \| grep -niE '\b(I\|we\|us\|our\|you\|your)\b'` finds nothing outside quoted material |
| E5 | Written in English | by eye, and it is the one assertion here that is |
| E6 | Every repo file named in the text is a markdown link | `printf '%s' "$text" \| perl -pe 's/\[[^]]*\]\([^)]*\)//g' \| grep -onE '(README\.md\|AGENTS\.md\|Dockerfile\|default\.toml)'`, then read what survives |
| E7 | Every permalink is pinned to the head SHA | `printf '%s' "$text" \| grep -oE 'blob/[0-9a-f]{7,40}'` and compare against `gh pr view "$N" --repo "$R" --json headRefOid -q .headRefOid` |
| E8 | No `event` was sent, which E1 already proves | E1 |

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

A baseline run needs the plugin genuinely absent. The symlink loads it in every
session on this machine, so point the CLI at an empty configuration directory:

```bash
export CLAUDE_CONFIG_DIR=$(mktemp -d)
claude plugin list        # expect: No plugins installed
claude -p "$(the prompt)"
```

That first command is the verification, and it is verified rather than assumed:
under a fresh `CLAUDE_CONFIG_DIR` the CLI reports no plugins installed, so the
skill cannot have loaded. Run it every time before trusting a baseline, and note
that whether a fresh configuration directory carries credentials is the one thing
the first baseline run has to confirm.

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
