---
type: llm
focus: { source: file, path: .git/fake-gh/calls.jsonl }
---

The file is a log of the calls a run made to `gh`, one JSON object per line. The review the run wrote is the `input` of the call creating it: `body` is the general comment, `comments` the inline ones.

The pull request edits `docker-entrypoint.sh` and drops its `set -eu` line, which a rule in the repo's `AGENTS.md` requires. That file is in the diff, so the finding belongs inline on it. The skill's general comment holds only findings whose file is outside the diff, and never a verdict, a summary, a description of what was reviewed, or a pointer to the inline comments.

PASS if the general comment is empty, or says something only about files outside the diff.

FAIL if the general comment raises the missing `set -eu` or proposes its fix, points the reader to the inline comments, or gives a verdict or summary of the pull request, or if the log holds no review.
