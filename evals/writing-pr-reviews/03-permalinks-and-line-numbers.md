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
  this pull request. A branch name fails, and so does any other commit, the base
  commit of the pull request included, which is the form this was actually broken in:

  ```bash
  head=$(gh pr view "$N" --repo "$R" --json headRefOid -q .headRefOid)
  printf '%s' "$text" | grep -oE 'blob/[0-9a-z]{7,40}' | sed 's|blob/||' | sort -u \
      | while read -r s; do case "$head" in "$s"*) ;; *) echo "not the head SHA: $s";; esac; done
  ```

  The prefix match is what makes it work on the seven-character SHAs a review
  writes. `grep -c 'blob/main'` was the whole assertion here first, and a link
  pinned to the base commit passes it in silence.
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

## Baseline, without the skill: Opus 5, 2026-08-06

Plugin disabled for the run, verified absent before it started, re-enabled from a trap
after. Same model, same prompt, same fixture. Nothing was posted to GitHub: no review,
no comments, the review written into the reply and offered at the end, "I have **not**
posted anything to GitHub, this is chat-only. Want me to submit it as a review on the
PR?"

**One prediction half-held and the other was wrong, and the real failure is one
neither of them named.** It wrote permalinks unprompted, five of them, and every line
number it cited is correct, so the predicted off-by-one did not happen. The bare
mentions did, in miniature.

**Two of the five links are pinned to the base commit rather than the head SHA.** The
three files in the diff got the head, `app/server.py`, `app/migrate.py` and
`Dockerfile#L9`. The two outside it, `README.md#L8-L9` and `AGENTS.md#L7`, got the base
commit of the pull request. Both resolve to the right lines, because neither file
differs between the two commits on this fixture, so nothing about the rendered link
looks wrong. That is the same shape as a wrong line number and quieter: a link that is
correct today because the file happened not to move, and stale the moment it does. The
rule the skill body states, pin to the head SHA when reviewing a pull request, is what
closes it.

**And this scenario's own assertion would have let it through.** E7 in `README.md`
catches it, since it reads every `blob/<sha>` against `headRefOid`, but the sharper
assertion written here was `grep -c 'blob/main'` is `0`, which a base-commit pin passes
without a murmur. Opening the links would have shown five correct pages. The assertion
above is now the mechanical form. That makes three defects a run has exposed in these
assertions, after the two counting mistakes recorded higher up this file and in
scenario 04, and the pattern in all three is the same: an assertion that tests the
failure imagined while writing it rather than the one the rule is about.

Every anchor read back at the head SHA:

| link | line at the head SHA | correct |
|---|---|---|
| `Dockerfile#L9` | `ENV APP_TIMEOUT=60` | yes |
| `README.md#L8-L9` | the "no environment variables" sentence | yes, but pinned to the base commit |
| `AGENTS.md#L7` | documentation moves with the behaviour | yes, but pinned to the base commit |
| `README.md:3`, bare | "Nothing here runs" | yes |

Set against scenario 01's baseline, where `AGENTS.md:8` was cited for a rule on line 7
of a seven-line file, the line-number rule is confirmed as a real failure without the
skill and an intermittent one: this run verified its numbers at the ref and said so in
its opening line, "Verified against the PR ref". One run in two got them right.

**The bare mentions are the predicted failure, in a smaller quantity than predicted.**
Stripping the links leaves three: `README.md:3` carrying a line number and no link at
all, `AGENTS.md` in "satisfies the AGENTS.md minor-version pin", and one
`config/default.toml` inside quoted README text, which is the exception the rule
already carries. So the mentions that got linked are the ones inside the numbered
findings, and the mentions in passing prose stayed bare, which is the form this rule is
usually broken in.

| | baseline | with the skill |
|---|---|---|
| Delivery | nothing posted | one `PENDING` review |
| Permalinks | 5, two pinned to the base commit | every mention linked at the head SHA |
| Bare mentions | 2, plus one legitimate exception | 1, and it should be bare |
| Line anchors correct | 4 of 4 | 5 of 5 |
| Em dashes | 7 | 0 |
| First person | 3 | 0 |
| Language | English | English |

Two more things the baseline did that the skill body bans, left out of the table
because the run with the skill was graded against the assertions above and these were
not among them, so there is no recorded figure to set them against: four markdown
headings where the skill asks for bold titles, and a verdict opening the whole thing,
"**Recommendation: request changes.**"

**The language row proves nothing here, and that is the prompt's doing.** Scenario 03
is the one prompt in the set written in English, "review PR #3", so a review in English
is what any run would produce with or without the rule. Scenarios 01, 02 and 04 are
where that rule is actually under test.

## Result: Sonnet 5, 2026-08-06

**Failed**, on the rule this scenario exists for, and it is the first failure by a run
carrying the skill in either model round. Everything else passed.

| | |
|---|---|
| E1, E2 | one `PENDING` review, `pulls/3/comments` length 0 |
| E3, E4 | no em dash, no first or second person |
| E7 | every link pinned to the head SHA, none on a branch or the base commit |
| Line anchors | 4 written, 4 correct: `README.md#L8-L9`, `AGENTS.md#L7`, `config/default.toml#L8`, `Dockerfile#L9` |
| Body shape | opens with one sentence introducing the section, bold title, no heading |
| **E6** | **four bare mentions, all of them repeated ones** |

**Every bare mention is a second mention.** The first time a file comes up it is
linked; from then on it is plain text:

| where | the bare mention |
|---|---|
| body, opening sentence | "the new Dockerfile environment variable", "what README.md promises" |
| body, after linking `README.md` | "the moment `APP_TIMEOUT` lands in the Dockerfile" |
| inline on `config/default.toml` | "the single-source-of-truth story README.md tells", "dropping the Dockerfile env var" |

That is exactly the shape this scenario's fourth assertion predicted: "a repeated
mention is linked again rather than left bare on the second pass, which is the form
this rule is usually broken in." Written as a prediction about a baseline, it turned
out to describe a run with the skill.

**Not amending the skill body.** The rule is already stated twice and as plainly as it
can be: "Link every occurrence, including repeated ones" in the Code references
section, and a red flag reading "Any repo file named in the review text without being
wrapped in a `[...](...)` link, whether or not it carries a line number." Nothing about
the text left this ambiguous, and a third statement of the same rule answers nothing.
This is the same call scenario 02's Opus entry made: a rule broken rather than a rule
missing.

What it does change is what the model set is for. Opus passed this scenario outright
and Sonnet did not, so the rule is settled as one that holds on the model these reviews
are actually asked for on and gives on a lighter one. That is the difference the
two-model set exists to surface, and this is the first time it has produced one.

**One bare mention is not a violation**, and it is worth separating out: `config/default.toml`
appears inside a paraphrase of the README sentence being quoted back. Same exception the
Opus run's single bare mention fell under, prose belonging to another document rather
than to the review.

## Second run: Sonnet 5, 2026-08-06

**Passed, including E6.** The failure above did not reproduce, so it is variance rather
than the model's default, and per the decision written down before this run the skill
body does not move.

Same prompt, same fixture, same model. Every file mention in both the body and the
inline comment is a link: `README.md:8-9` twice, `AGENTS.md:7`, `config/default.toml:8`,
`app/server.py`, `docker-entrypoint.sh`. Stripping the links leaves nothing at all,
against four survivors on the first run. Every link carries the head SHA, all three line
anchors resolve, no em dash, no first or second person, no heading, and the body opens
with one sentence introducing its single section.

**What differed between the two runs is the shape, not the discipline.** The first
wrote two inline comments and a two-bullet body; this one wrote a single inline comment
on `Dockerfile` and a one-paragraph body. Fewer mentions is part of why fewer went bare,
which is worth saying rather than claiming the rule got stronger: 6 linked mentions here
against 10 mentions and 4 bare ones before.

**So the rule is now 3 for 4 across the set**, passing on both Opus runs and one of two
Sonnet runs. That is an inconsistency worth leaving recorded and watching, not a
systematic failure to answer in the skill body. The one thing it does settle is that the
two-model set was right to exist: the variance is real and only repeated runs expose it.

**One bare "README" without its extension**, in "then the README sentence needs
rewriting". The first Sonnet run had the same, in "worth updating that README
paragraph", and neither was counted, since E6 matches `README.md`. Whether a bare
"README" in prose is a file mention the rule covers is genuinely unsettled, and both
runs did it, so it is the one thing here with two data points rather than one.

## Baseline, without the skill: Sonnet 5, 2026-08-06

Plugin disabled for the run, verified absent before it started, re-enabled from a trap
after. Nothing posted: no review, no comments, the whole thing written into the reply.

**No permalinks at all**, which is where the two baselines part company. The Opus
baseline wrote five links unprompted and pinned two of them wrongly. Sonnet wrote none:
fifteen bare mentions across six files, every citation a plain `file:line` string.

**Its three line numbers are all correct**, `README.md:8-9`, `AGENTS.md:7` and
`server.py:3`, the last of which it went and found on its own while checking whether
anything reads the new key.

| | Opus baseline | Sonnet baseline | with the skill, Opus | with the skill, Sonnet |
|---|---|---|---|---|
| Delivery | nothing posted | nothing posted | one `PENDING` review | one `PENDING` review |
| Permalinks | 5, two wrongly pinned | 0 | every mention | every first mention |
| Bare mentions | 2, plus an exception | 15 | 1, correctly bare | 4, all repeats |
| Line anchors correct | 4 of 4 | 3 of 3 | 5 of 5 | 4 of 4 |
| Em dashes | 7 | 10 | 0 | 0 |
| Markdown headings | 4 | 6 | 0 | 0 |
| First person | 3 | 1 | 0 | 0 |

**So the rule this scenario tests degrades in two steps rather than one.** Without the
skill, Sonnet links nothing and Opus links most things badly. With the skill, Opus links
everything correctly and Sonnet links every file the first time it appears. Reading the
four runs together, what the skill reliably buys on both models is the head SHA and the
verified line number, and what it buys unevenly is the discipline of linking a file the
second and third time it comes up.

**The line-number rule now has four data points and holds three ways.** Both baselines
here got their numbers right; the Opus baseline on scenario 01 did not. Whatever makes
the difference, it is not the model.

**It found the same unplanned finding the run with the skill found**, that nothing
reads the new key, and went further: it also checked the PR description against the
diff and reported that "stops being hardcoded" describes a hardcoded timeout that never
existed. Both configurations get there, so it is a property of the fixture rather than
of the skill.

**A section the skill body would have removed.** The reply closes with "Two things I
checked and am *not* flagging", the pinned base image and the missing `__main__`
guards. That is the rule about a finding resolving to "nothing to change": good work to
have done, and a message to the author rather than a review comment.

## Rerun: Opus 5, 2026-08-07

Rerun against the delegation change. Every assertion this scenario exists for held, and
the new non-negotiable did not.

| | |
|---|---|
| E1, E2 | one review `PENDING`, nothing published |
| E3, E4 | no em dash, no first or second person |
| E6 | every mention of `README.md`, `AGENTS.md`, `config/default.toml` and `app/server.py` inside a link |
| E7 | every permalink on the head SHA |
| E9 | two inline comments, 58 and 53 words |

All four line anchors were checked against the head SHA and resolve to the lines the
comments talk about: `AGENTS.md#L7` is the rule about documentation moving with
behaviour, `README.md#L5-L6` the port and the migration, `README.md#L8-L9` the claim
about environment variables, `config/default.toml#L7-L8` the `[limits]` key.

**It loaded `superpowers:requesting-code-review` and then reviewed the diff itself.** The
transcript is unambiguous: the skill was invoked, `code-reviewer.md` was read, and the
next tool calls are `git show` of the repo's own files followed by writing the payload.
No subagent was ever dispatched. The rationalisation is not in the text, so what this
records is the behaviour rather than the reasoning, and the superpowers skill names this
exact failure in its own table of excuses.

One of two runs in this rerun to skip the dispatch, out of seven.
