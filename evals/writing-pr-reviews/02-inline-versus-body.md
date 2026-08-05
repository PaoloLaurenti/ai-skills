# 02. Inline versus general comment

**Rule under test:** a finding whose file appears in the diff is anchored inline.
Only what cannot be anchored goes in the general comment, and the body contains
nothing about the diff itself.

**Fixture:** branch `inline-versus-body`. The diff edits `docker-entrypoint.sh`
and drops its `set -eu` line, which breaks a rule the fixture's own `AGENTS.md`
sets for the repo.

**Prompt:**

> dammi un parere sulla PR #N

The trap is that the finding feels like a repo-level observation. It is a breach of
a convention written in another file, so the natural shape is a body section
titled after the rule. The file carrying the breach is in the diff, so the rule
says inline, and the body should end up with nothing to say about it at all.

## Expected outcome

- **M** An inline comment exists whose `path` is `docker-entrypoint.sh`.
- **M** Its `line` falls on the hunk the change touches, so it is anchored to the
  breach rather than to the top of the file. The removed `set -eu` has no line in
  the head version, so either anchor counts: the added line on `RIGHT`, or the
  removed one with `side: LEFT`. What fails is an anchor outside the hunk.
- **M** The general comment carries no finding about `docker-entrypoint.sh`, and no
  restatement of the fix the inline comment proposes:
  `printf '%s' "$body" | grep -niE 'set -eu|restor|docker-entrypoint'`, then read the
  hits. Naming the diffed file as the *cause* of a documentation finding is allowed
  and often unavoidable, since a stale sentence has to be attributed to whatever made
  it stale. What fails is the body proposing the fix a second time.
- **E** The inline comment names the rule it breaks and links the `AGENTS.md` that
  sets it, rather than asserting the rule without a source.
- **E** The body, if it says anything at all, says it about something outside the
  diff. A body that only points at the inline comments fails: that pointer is one
  of the things the skill body bans.
- All of the every-run assertions in `README.md`.

## Result: Opus 5, 2026-08-05

**Mixed.** The rule this scenario tests held. A narrower rule, in the same section of
the skill body, did not.

What held: the `set -eu` removal was reported inline on `docker-entrypoint.sh`,
anchored inside the hunk, with a `suggestion` block restoring the line above the new
log statement. The trap worked as designed and the review did not fall into it: the
breach of a rule written in another file did not become a body section. The body's own
finding is genuinely outside the diff, `AGENTS.md#L5` for the rule and
`README.md#L5-L6` for the promise that a failed migration stops the container, and the
inline comment traces the mechanism rather than asserting it: without `set -e`, a
non-zero exit from the migration falls through to `exec "$@"` and traffic is served
against an unmigrated database. Every style assertion passed, no bare mentions, all
links pinned to the head SHA.

What did not: the body reads "Restoring the line in `docker-entrypoint.sh` is the
smaller of the two ways out", which repeats the fix the inline comment already
proposes. The skill body says the body stays silent about a follow-up an inline
comment already carries. The pivot the sentence is making is legitimate, since the
docs finding only matters if the removal is deliberate, but it can be made without
restating the fix or naming the diffed file: "either the change comes back into line
with the rule, or the rule and the promise move with it" says the same thing and
leaves the fix where it is anchored.

This is a failure of a rule the skill body already states, not a gap in it, so
nothing here calls for an amendment. What it calls for is a second data point: whether
Sonnet 5 does the same thing.

**The assertion needed narrowing, again.** As first written it read "the body does not
mention `docker-entrypoint.sh`, `grep -c` is 0". That is too strict to be right:
scenario 03 passed with a body naming a diffed file as the *cause* of a documentation
finding, which is unavoidable, since a stale sentence has to be attributed to whatever
made it stale. Naming the file is not the violation. Repeating the inline proposal is.
The corrected assertion is above.

**No payload was left in the clone this run**, where earlier runs left one. The anchor
was therefore graded from the API alone, which for a pending review answers `position`
only. It was enough here, since there is no multi-line collapse to check, and it is
the case scenario 04 flags as inconclusive when there is.

## Baseline, without the skill

Not yet recorded. The prediction worth testing is a body bullet naming the file,
with the reasoning that a convention breach is a repo-level concern. Quote what it
actually says.
