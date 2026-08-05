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
- **M** The general comment does not mention `docker-entrypoint.sh`:
  `printf '%s' "$body" | grep -c 'docker-entrypoint'` is `0`.
- **E** The inline comment names the rule it breaks and links the `AGENTS.md` that
  sets it, rather than asserting the rule without a source.
- **E** The body, if it says anything at all, says it about something outside the
  diff. A body that only points at the inline comments fails: that pointer is one
  of the things the skill body bans.
- All of the every-run assertions in `README.md`.

## Baseline, without the skill

Not yet recorded. The prediction worth testing is a body bullet naming the file,
with the reasoning that a convention breach is a repo-level concern. Quote what it
actually says.
