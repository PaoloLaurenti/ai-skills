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
  `$text` sits inside a `[...](...)` link. Strip the links, then look at what is left,
  rather than comparing two counts: a filename appears in the link text and again
  inside the URL, so counting raw occurrences double-counts every link and reports
  violations that are not there.

  ```bash
  printf '%s' "$text" | perl -pe 's/\[[^]]*\]\([^)]*\)//g' \
      | grep -onE '(README\.md|AGENTS\.md|Dockerfile|default\.toml|server\.py)'
  ```

  What survives is either a violation or one of the exceptions, so read it rather
  than counting it.
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

## Result: Opus 5, 2026-08-05

**Passed.** Five line anchors written, five correct when read back at the head SHA,
which is the assertion this scenario exists for:

| link | line at the head SHA | the comment claimed |
|---|---|---|
| `Dockerfile#L9` | `ENV APP_TIMEOUT=60` | the new environment variable |
| `config/default.toml#L8` | `timeout_seconds = 30` | the new key, and the disagreeing value |
| `README.md#L8-L9` | the "no environment variables" sentence | the claim the change leaves behind |
| `AGENTS.md#L7` | documentation moves with the behaviour | the rule placing the edit in this PR |
| `app/server.py#L3` | `PORT = 8080` | that the file hardcodes the port and reads no config |

Every link carried the head SHA, none pointed at a branch, and the review named five
files rather than the three the fixture was built around: it went looking for the
consumer of the new key and found there is none, which is a finding the scenario did
not plan for.

**One bare mention, and it should be bare.** Stripping the links leaves a single
`config/default.toml` in a code span, inside the replacement wording proposed for
the README. That text is meant to be pasted into a document, so a permalink inside it
would corrupt what it hands over. The rule about linking every occurrence is about
the review's own prose, not about prose the review is drafting for somewhere else.

**The assertion as first written was a false-positive generator**, in the same way
scenario 04's was. Counting occurrences of a filename and comparing against counted
links double-counts every link, because the filename appears in the link text and
again inside the URL: it reported 13 mentions against 6 links on a review where all
but one mention was linked. Strip the links first, then count what is left. The
corrected form is in the assertions above.

## Why the line-number assertion is the valuable one

Wrong line numbers are the failure this scenario exists for, and they are invisible
without going back to the ref: a review with a confident `#L21` reads exactly like
a correct one. The `sed -n` check above is the whole point of the scenario, and it
is worth running even when everything else passes.

## Baseline, without the skill

Not yet recorded. Two predictions to test: bare code spans instead of links for the
files that carry no line number, and at least one line number off by one or two.
Quote the reasoning for the code spans if the run gives one.
