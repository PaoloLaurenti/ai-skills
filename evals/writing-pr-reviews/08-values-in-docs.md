# 08. Values the code owns, kept out of the docs

**Rule under test:** documentation does not carry values that live in the code, so a
review never proposes prose carrying one. Where a doc sentence has drifted because it
copied a value, the fix is removing the value and naming the file that sets it, not
swapping the stale number for the current one.

The rule came out of four review comments a person rejected on one pull request, all of
them factually right and all proposing the wrong repair: a coverage floor, a list of
build task names and a description of what CI covered today, each restated in a package's
own documentation. "You're adding a lot of noise that needs to be updated every time" is
what came back, and that is the failure this scenario is built to catch.

**Fixture:** branch `values-in-docs`. The diff adds `app/limits.py` with
`TIMEOUT_SECONDS = 45` and `BATCH_SIZE = 250`, wires both into `app/migrate.py`, and in
the same change appends a **Batch limits** section to `AGENTS.md` saying the run "times
out after 30 seconds and processes batches of 100". Both numbers are wrong, so the
tempting fix is the corrected pair.

Two choices in the fixture are deliberate. `AGENTS.md` sits **inside** the diff, unlike
scenario 05's `README.md`, so the proposed prose arrives in a `suggestion` block and can
be read directly rather than inferred from a body paragraph. And nothing was added to
`main`: a limits module visible from every branch would become a finding inside the seven
scenarios that predate this one, which is the contamination the note about `deploy.yml`
in `README.md` describes.

**Prompt:**

> dammi un parere sulla PR #N

## Expected outcome

Set `sugg` to the prose the review proposes for `AGENTS.md`:

```bash
sugg=$(gh api "repos/$R/pulls/$N/reviews/$id/comments" \
  -q '.[] | select(.path == "AGENTS.md") | .body' \
  | awk '/^```suggestion/{f=1; next} /^```/{f=0} f')
```

- **M** The finding is inline on `AGENTS.md` rather than in the body, because the file is
  in the diff:
  `gh api "repos/$R/pulls/$N/reviews/$id/comments" -q '[.[] | select(.path == "AGENTS.md")] | length'`
  is at least `1`.
- **M** The proposed prose carries neither the stale pair nor the corrected one:
  `printf '%s' "$sugg" | grep -cE '\b(30|100|45|250)\b'` is `0`. A run proposing "times out
  after 45 seconds and processes batches of 250" has found the right problem and applied
  the banned repair.
- **M** The proposed prose names the file that owns the values:
  `printf '%s' "$sugg" | grep -c 'limits\.py'` is at least `1`. **This is the assertion the
  scenario exists for, and the one to read first.** The baseline recorded below scores `0`
  on the grep above while carrying nothing of the rule, because it proposes aligning the
  two sides rather than naming either pair. Only the pointer separates a repair that ends
  the duplication from one that renews it.
- **E** The comment still says what is wrong. Removing the values is the repair, not a
  reason to stay silent about a section the diff got wrong.
- **E** The proposal is handed over rather than described, per Every observation carries a
  proposal: the replacement sentence itself, not a rule for writing one.
- All of the every-run assertions in `README.md`.

**Where `$sugg` comes back empty, grade the comment's proposal by eye.** Deleting the
**Batch limits** section outright is a legitimate reading, and the skill body asks for a
pure deletion to carry no `suggestion` block at all. That is a pass on the first two
assertions rather than an ungradeable run, but the third still has to hold somewhere in
the comment: a deletion with no pointer at `app/limits.py` leaves the reader nowhere.

## Result: Opus 5, 2026-08-28

**Passed**, every assertion, and the proposal came back as a sentence rather than as a
rule for writing one, which is where the rerun of scenario 05 fell a step short.

The suggestion block, in full:

```
The migration step applies a timeout and a batch size. Both are set in
`app/limits.py`.
```

| | |
|---|---|
| Values in the proposed prose | `0` of `30`, `100`, `45`, `250` |
| The file that owns them | named, `app/limits.py` |
| The finding's home | inline on `AGENTS.md`, which is in the diff |
| E1, E2 | one review `PENDING`, `pulls/8/comments` length 0 |
| E3, E4 | no em dash, no first or second person |
| E7 | every permalink on `e1cdb0b1` |
| E9 | one inline comment, 55 words |
| E10 | `1` |

**It named the drift as the reason rather than only the symptom**: "Prose carrying the
numbers drifts as soon as the constants move, and this section arrived that way". The
comment then closes the branch the fixture leaves open, that the doc might be right and
the code wrong, by saying where to change the constants if `30` and `100` were intended.
Neither reading needs the numbers in `AGENTS.md`.

**The bare `limits.py` E6 reports is correct.** It sits inside the suggestion block, which
is replacement prose for a documentation file rather than a mention in the review, and a
markdown link inside it would be committed into `AGENTS.md`. Same shape as the exception
for literal syntax quoted from a file, and the fourth time on this suite that what
survives the link strip has to be read rather than counted.

**One unplanted finding, in the body**: the new limits have no key in
`config/default.toml`, so the `README.md` claim that every setting has a key there stops
holding. That is the pre-existing imprecision three earlier runs on scenario 05 reached
independently, arrived at here from a different direction. It gave both ways out with what
each costs, per the rule on two reasonable answers.

## What a baseline is expected to show here

The finding itself is easy and a baseline should reach it: two numbers in a doc disagree
with two constants added in the same diff, and both sit in the diff. What the baseline is
expected to produce is the *repair* the rule bans, `30` and `100` corrected to `45` and
`250`, because that is the obvious reading of "documentation moves in the same change as
the behaviour it describes", which the fixture's own `AGENTS.md` states. A baseline that
proposes dropping the numbers instead would mean the rule is carrying less weight than the
skill body assumes.

**Neither happened, and the section below records what did.** The baseline refused to
choose a pair at all, which the prediction above did not anticipate and which is what
exposed the weakness in the second assertion.

## Baseline, without the skill: Opus 5, 2026-08-28

Plugin disabled for the run, same model, same prompt, same fixture.

**It delivered nothing to GitHub.** No review, pending or otherwise, and no comments. The
whole thing went into the reply, in Italian, with `##` headings, em dashes, first and
second person throughout, no permalink anywhere, and a closing verdict, "Non la approverei
così". The same shape every baseline on this suite has produced.

**It found the planted finding immediately**, quoted both sides and named
`app/limits.py:3-4`. The rule the fixture states about documentation moving with behaviour
was cited, and it noticed the sharper version of the problem unprompted: the commit keeps
the letter of that rule while publishing false numbers, so a reader of `AGENTS.md` has no
reason to suspect them.

**The prediction written above was wrong about the repair, and the way it was wrong is the
result.** It did not propose `45` and `250`. It refused to choose: "Non ti dico quale
coppia sia quella giusta... Va deciso qual è la fonte autorevole e allineata l'altra",
decide which side is authoritative and align the other. So the banned pair never appeared,
and the assertion as written would have passed a run that carries none of the rule.

What it never once considered is removing the numbers. Both ways out it offers keep the
copy in `AGENTS.md` and differ only over which side gets edited, which leaves the doc
needing an update every time a constant moves. That is the distinction the rule is for,
and it is visible here: the gap is not stale values against fresh ones, it is a
duplication that survives against one that is gone.

**So the assertion needs reading, not just running.** `grep -cE '\b(30|100|45|250)\b'`
returning `0` is necessary and not sufficient, since prose proposing to align the doc with
the code scores `0` while carrying nothing of the rule. The assertion that discriminates
is the third one, that the proposal names `app/limits.py` as the owner, and it is the one
to read first.

**It found a defect neither the fixture planted nor the run with the skill reported.**
`AGENTS.md` says both limits "apply to the migration step", while `app/migrate.py` only
interpolates the constants into a `print()` and `docker-entrypoint.sh` wraps nothing, so
the documented timeout does not exist at any value. That is a property of the fixture, and
it is left in place rather than repaired: it predates the run recorded above, so changing
it now would invalidate that result. Worth knowing that this branch carries a second, real
finding for anything graded on it later.
