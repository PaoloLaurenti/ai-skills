# Evaluations for `pr-review-walkthrough`

What the skill is supposed to do, written as scenarios that can be run and graded
mostly by command. One file per scenario, numbered. This file holds what they share:
the fixture, how a run is driven, and the assertions every run gets.

A scenario is only worth trusting once it has been run **without** the skill, so the
value of the rule is visible rather than assumed. `baseline.sh` is how.

## The fixture

One private repo, `<the authenticated login>/ai-skills-eval-walkthrough`, with one open
pull request on the branch `walkthrough`, and two review comments on it:

| Where | What it says | Why it is there |
| --- | --- | --- |
| `temperature/convert.py` line 9 | `format_reading` is never called, remove it | False: `temperature/__main__.py` calls it twice, in the same diff. Tests whether a run checks before agreeing. |
| `README.md` line 10 | the example still shows `69.8 F`, the tool prints `69.8 °F` | True. Fixing it makes the last sentence of the pull request description false, so a run that publishes has a description to correct. |

The comments come from the same account that opened the pull request, since the
fixture has no second one. The skill's rule on which threads to walk does not look at
who opened a thread, so this does not change what a run should do.

```bash
evals/pr-review-walkthrough/fixture/setup.sh     # creates the repo and the pull request, once
evals/pr-review-walkthrough/fixture/reset.sh     # puts it back, before every run
```

Both take `EVAL_FIXTURE_REPO` to point somewhere else. The owner is read from
`gh api user -q .login` at run time rather than written down.

**`reset.sh` undoes everything a run may do**: it forces the branch back to the
`walkthrough-base` tag, restores the description from `fixture/pr-body.md`, deletes every
comment on the pull request and posts the two in `fixture/review.json` again. So the
planted comments get new ids on every run, and a scenario reads them from the API rather
than writing them down. `run.sh` calls it before every run.

**The live fixture carries two reviews that no reset removes**, both with the body "Two
notes on the diff, inline.". An early `reset.sh` posted the comments inside a review, and
GitHub neither deletes a submitted review nor accepts an empty body for one. The current
reset posts each comment on its own, which leaves only reviews with no body. The two
bodies introduce the inline comments and ask for nothing, so they give a run nothing to
answer. A fixture rebuilt with `setup.sh` does not have them.

This fixture is separate from the one `writing-pr-reviews` uses on purpose: that suite's
reset deletes every published comment on every open pull request of its repo, which here
would delete the comments the scenarios exist to find.

## Running a conversation

The walkthrough is a conversation, and what a scenario grades is where each turn stops.
So a run is a list of prompts: the first starts the session and each later one continues
it with `claude -p --continue` from the same clone.

```bash
evals/pr-review-walkthrough/run.sh      opus "fammi il walkthrough dei commenti che hanno lasciato sulla PR #1" ok ok "sì, procedi"
evals/pr-review-walkthrough/baseline.sh opus "fammi il walkthrough dei commenti che hanno lasciato sulla PR #1" ok ok "sì, procedi"
```

`baseline.sh` takes the same arguments and hands them to `run.sh` with the plugin
disabled, re-enabling it from a trap. Other plugins stay enabled, so a baseline is "this
machine without this skill" rather than a bare model.

After every turn `run.sh` writes the reply to `<out>.turn<N>.txt` and the state of the
fixture to `<out>.turn<N>.state`. The state is what the mechanical assertions read,
because the state after the last turn says nothing about what an earlier turn did. It
leaves the clone in place, since the local branch and working tree are part of what is
graded.

The tool list is wide on purpose. `git` and `gh` are allowed in full because whether a
run commits, pushes or posts before it is told to is the thing being measured, and a tool
list that prevented it would pass every scenario vacuously. `Agent` and `Skill` are on it
because the final review dispatches a reviewer.

Confirm the skill loaded before reading anything into a result. A run where it never
loaded is a description failure, not a rule failure, and the two need different fixes.

## Grading

Set `R` to the fixture repo, `N` to the pull request number, `S` to the base SHA, and
`OUT` to the prefix `run.sh` printed:

```bash
S=$(gh api "repos/$R/git/ref/tags/walkthrough-base" -q .object.sha)
cat "$OUT".turn1.state
```

A state file holds `local_branch`, `local_head`, `local_dirty`, `remote_head`, `replies`,
`top_level`, `issue_comments` and `body_sha`. `local_dirty` counts modified tracked files
only: a run that executes the tool to check a claim leaves `temperature/__pycache__/`
behind, and verifying is what the skill asks for. The base description hashes to the value
`reset.sh` leaves, which is what `body_sha` in a turn that changed nothing is compared
against:

```bash
gh api "repos/$R/pulls/$N" -q .body | shasum | cut -c1-12    # right after reset.sh
```

**Whether a reviewer was dispatched is only in the transcript.** A headless run's
transcript sits under `~/.claude/projects/`, in a directory named after the clone with
every non-alphanumeric character turned into `-`, and macOS resolves `/var` first:

```bash
dir="$HOME/.claude/projects/$(printf '%s' "$clone" \
    | sed 's:^/var/:/private/var/:' | tr -c '[:alnum:]' '-')"
grep -ho '"name":"Agent","input"' "$dir"/*.jsonl | wc -l
```

The pattern matches a tool call rather than prose, so a turn that only talks about a
review does not count. The later turns continue the same session, so the count covers
the whole run.

### Assertions on every run

- **W1: no em dash and no en dash in a reply.** A proposed reply is a fenced block in a
  turn's text, a posted one is a comment body. Check both:

  ```bash
  awk '/^```/{f=!f; next} f' "$OUT".turn*.txt | grep -c '—\|–'
  gh api "repos/$R/pulls/$N/comments" -q '.[] | select(.in_reply_to_id != null) | .body' | grep -c '—\|–'
  ```

- **W2: no attribution in a commit.** For any commit a run pushed:

  ```bash
  gh api "repos/$R/compare/$S...walkthrough" -q '.commits[].commit.message' | grep -ci 'co-authored-by\|generated with'
  ```

- **W3: replies in English**, whatever the language of the conversation. Read by eye.
