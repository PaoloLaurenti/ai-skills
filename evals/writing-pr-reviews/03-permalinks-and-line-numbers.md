# 03. Permalinks and line numbers

**Rule under test:** every mention of a repo file is a markdown link to a GitHub
permalink pinned to the head SHA, including mentions that carry no line number and
including repeated ones. Line numbers are verified at that ref rather than
recalled.

**Fixture:** branch `permalinks`. The diff touches `Dockerfile` and
`config/default.toml`, and the finding needs `README.md`, which is not in the
diff, so the review has to name three files in three different situations.

**Prompt:**

> review PR #N

## Expected outcome

- **M** Every occurrence of `README.md`, `Dockerfile` and `config/default.toml` in
  `$text` sits inside a `[...](...)` link. Count the mentions and the links and
  compare: `printf '%s' "$text" | grep -oE '\[[^]]*(README\.md|Dockerfile|default\.toml)[^]]*\]\([^)]*\)' | wc -l`
  against `printf '%s' "$text" | grep -oE '(README\.md|Dockerfile|default\.toml)' | wc -l`.
- **M** E7 at its sharpest: every `blob/<sha>` in the text carries the head SHA of
  this pull request, and no link uses a branch name.
  `printf '%s' "$text" | grep -c 'blob/main'` is `0`.
- **M** Every line anchor resolves to the line the finding claims. For each
  `#L<n>`, read that line at the head SHA and confirm it is the one the comment
  talks about:
  `gh api "repos/$R/contents/<path>?ref=$sha" -q .content | base64 -d | sed -n '<n>p'`.
- **E** A repeated mention is linked again rather than left bare on the second
  pass, which is the form this rule is usually broken in.
- **E** Nothing that is not a repo file got linked: a bare extension, syntax quoted
  from a file, or a runtime path inside the container stay plain.
- All of the every-run assertions in `README.md`.

## Why the line-number assertion is the valuable one

Wrong line numbers are the failure this scenario exists for, and they are invisible
without going back to the ref: a review with a confident `#L21` reads exactly like
a correct one. The `sed -n` check above is the whole point of the scenario, and it
is worth running even when everything else passes.

## Baseline, without the skill

Not yet recorded. Two predictions to test: bare code spans instead of links for the
files that carry no line number, and at least one line number off by one or two.
Quote the reasoning for the code spans if the run gives one.
