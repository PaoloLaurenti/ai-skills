---
name: writing-pr-reviews
description: >-
  Use when asked for a review, an opinion, or feedback on a pull request in any
  repo or directory, including phrasings like "dammi un parere sulla PR #N",
  "review PR 16", "guarda questa PR", "che ne pensi di questa PR", or when
  preparing, posting, or editing review comments on a GitHub PR.
---

# PR Review Conventions

Standing preferences for how a pull request review is written and
delivered. These override default review habits. They are conventions, not
general advice on how to find defects.

## Non-negotiables

1. **ALWAYS create the review in PENDING state. Never submit it.** The general
   comment and every inline comment stay pending, visible only to the account
   that created the review, until it gets submitted by hand. Create it pending,
   then state the command to submit. The only thing that changes this is an
   explicit request to submit or publish the review; a request to write, prepare,
   add, or leave a review is not that, and neither is a review that reads as
   ready to go.
2. **Write the review in English**, even when the conversation is in Italian.
3. **Never use em dashes** (the `—` character) anywhere in the review: not in the
   general comment, not in inline comments. Restructure the sentence instead.
   Commas, colons, parentheses, or two shorter sentences all work.
4. **Impersonal forms only.** No "I", "we", "us", "our", "you", "your" anywhere
   in the review. Use "worth ...", "the suggestion is to ...", passive and gerund
   constructions instead. This holds even where a first-person plural would sound
   collegial: "we are in good shape today" becomes "today's position is fine",
   and "gives us a way in" becomes "leaves a way in".
5. **Be short and direct.** No filler.

## Tone

Cordial, warm, and constructive throughout, in the general comment and in every
inline comment alike. The reader is a colleague who did real work. The review
should sound like a helpful colleague thinking out loud, never like a verdict
being handed down.

- Frame a finding as something to update, not as a mistake someone made. "This
  section is ready for a refresh" rather than "this is wrong".
- Prefer "worth doing" and "might be" over "must" and "this fails".
- Headings name the topic kindly, not the fault. "Docs to refresh" rather than
  "Docs this change makes wrong".
- Where the change gets something genuinely right, say so on the line that does
  it.
- Drop blunt superlatives and flat verdicts. "The worst is", "nobody wrote this
  down", "crash loop with no way in" all say the same thing more coldly than
  needed.

Warmth never costs precision: the file, the line, and the concrete fix stay
exactly as sharp as they were. It never becomes padding either. Cordiality comes
from word choice, not from added praise or hedging, and the brevity rule holds.

## Check the diff against the repo's own docs

Every review checks whether the change still agrees with the documentation the
repo keeps about itself. This is mandatory and easy to skip, because these files
usually sit outside the diff and nothing in the diff points at them.

Find them first, at any depth, and read the parts the change touches:

```bash
find . \( -name 'README.md' -o -name 'AGENTS.md' -o -name 'CLAUDE.md' \) -not -path '*/node_modules/*'
```

Root and per-package `README.md`, plus `AGENTS.md` and `CLAUDE.md` wherever they
sit. Read them at the PR head SHA, not from the working tree, so the comparison
matches what the PR actually proposes.

Look in both directions:

- **The diff contradicts something the docs state.** A sentence that was true
  before the change and is false after it. Quote the stale claim, link it, and
  say what it should say now.
- **The diff adds behaviour, a convention, or an operational step the docs never
  mention.** Say what to add and where.
- **The diff breaks a rule the repo sets for itself** in `AGENTS.md` or
  `CLAUDE.md`. Those files bind the change, so a conflict is a finding even when
  the code is fine on its own terms. If the change is right and the rule is what
  should move, say that instead.

**Never write "the docs" generically.** Name the files: the root `README.md`, a
package's `README.md`, `AGENTS.md`. A reader cannot act on "the docs do not cover
this" without first guessing which file is meant. Where a `CLAUDE.md` is only an
`@AGENTS.md` include, say so, so nobody edits it twice.

Report wording that has merely become misleading, not only outright
contradictions. When a doc file is itself in the diff, comment inline with a
`suggestion` block carrying the corrected prose. When it is not, the finding goes
in the general comment.

## Verify every factual claim about the architecture

A review states facts about how the system fits together: what runs where, what a
container or image is, what a name refers to, what talks to what, which
credentials are in play. Read the thing that defines each one before writing it.
Never infer it from a crate name, a service name, or the shape of the diff.

The failure mode is a claim that sounds obviously right and is wrong. A crate
called `mia-identity` does not make the deployed container mia-identity's: the
container name lives in the deploy workflow and the ECR repository, and the image
may well be one per project rather than one per service. Nothing in the diff
reveals that, and a wrong claim of this kind passes every mechanical check on the
review, so only reading the source catches it.

| Claim about | Where the answer actually is |
|---|---|
| Container or image identity and naming | Dockerfile stages, deploy workflow inputs, ECR repository |
| What a deployed unit runs | `ENTRYPOINT` and `CMD`, not the crate names |
| Local versus deployed topology | compose file for local, deploy workflow for real |
| Infra settings, credentials, roles | the infra repo, not this one |
| Dependency behaviour | upstream source at the pinned version |

When the answer lives somewhere this repo cannot see, write the finding as
something to verify and name where to look, rather than stating it as fact.

## Where each finding goes

Anchored inline to its line whenever the file appears in the diff. Only what
cannot be anchored goes in the general comment.

Before writing a body bullet, check whether its file is in the diff. If it is,
the finding belongs inline.

## Every observation carries a proposal

A finding that only names a problem hands the work back to the reader. Say what
to do about it in the same breath, every time.

The strongest form is a committable `suggestion` block, so reach for that
whenever the fix is a concrete edit to lines in the diff (see the next section).
When a suggestion is not possible, the proposal is still explicit: the
replacement wording for a doc, the setting to look up and how to choose its
value, the code change the real fix needs, the alternative approach worth
weighing.

**Trace the proposed fix against the mechanism before claiming what it buys.**
A fix that sounds like it solves the problem often does not. Walk the code path:
if the entrypoint runs a migration under `set -e` before reaching `exec "$@"`,
then adding a `CMD` cannot give anyone a way past a failed migration, however
plausible that reads. Claiming a benefit the change does not deliver is worse
than raising nothing, because the author acts on it and the problem stays.

Check the direction of a robustness claim too, since it inverts easily. A
relative path follows `WORKDIR`, an absolute one pins a location: proposing
`/app/bin/x` over `./bin/x` "so it does not depend on WORKDIR" gets it backwards,
because the relative form is the one that survives WORKDIR moving. Name the
scenario the change protects against, then check whether anything actually
produces that scenario.

Where two edits only work as a pair, say so in both comments. A suggestion that
breaks the build when committed on its own is worse than no suggestion.

A proposal is finished when the reader knows what to do next without having to
ask. For anything living outside this repo that means naming where to look and
what to look for, not merely flagging that something needs attention. "Needs eyes
on that side" is not a proposal. "Look up the value in Pulumi, bearing in mind a
service behind a load balancer gets 0 when nobody sets it, then size it for the
slowest migration you expect" is.

Where two answers are both reasonable, give both and say what each costs, rather
than leaving the choice unstated.

**A finding that resolves to "nothing to change" does not belong in the review.**
This holds for inline comments exactly as much as for body sections. Verifying
something and discovering it is already fine is good work, but the result is a
message to the author, not a review comment. Either it converts into a real ask,
most often "worth writing this down" folded into a documentation finding, or it
comes out. Say it in the reply instead.

If a comment opens with "nothing to change here", or explains a mechanism without
ever asking for anything, it should not be in the review. Where the explanation
is genuinely worth having, attach it to the comment that does carry an ask, as
the reason behind it, rather than standing it up on its own.

**Quantify a nit before raising it, and drop it when the number is small.** Work
out what the change actually buys: bytes saved, milliseconds, one layer, one
fewer allocation. If the honest answer is "383 bytes and one layer in a
hundred-megabyte image", it does not earn a comment, however correct it is.
Reviews that spend the reader's attention on those lose it for the findings that
matter.

Similarly, a comment is not needed for something the code already documents. If a
comment two lines above the finding already explains the mechanism, a review
comment restating it, or proposing a `TODO` that adds only a wish, is noise. That
belongs in an issue if it belongs anywhere.

## Code suggestions

Whenever the fix is a concrete edit to lines that sit in the diff, write it as a
committable suggestion instead of describing it in prose. Always reach for one
where it makes sense.

A suggestion earns its place when there is replacement text worth handing over
verbatim, so the author neither retypes it nor guesses at it. Where there is no
such text, prose is the better tool.

````markdown
```suggestion
COPY --chmod=755 docker-entrypoint.sh ./
```
````

The block replaces exactly the lines the comment is anchored to, so the anchor
must cover every line being replaced. A suggestion that collapses three lines
into one needs `start_line` and `line` spanning all three, even though the
replacement is a single line.

**Narrow the anchor until the suggested change contains only real changes.**
GitHub renders a suggestion as a diff of the anchored lines against the block, so
every unchanged line reproduced inside the block shows up as a removal plus an
identical addition. A reader then sees "Suggested change" listing lines that are
not changing at all, and stops trusting the suggestion.

- **A pure deletion gets no suggestion block at all.** Anchor the comment to the
  line and say it should go. There is no replacement text to hand over, and a
  "Suggested change" showing a single struck-out line tells the reader nothing
  the anchor already told them.
- To insert a line, anchor the single neighbouring line the insertion attaches to
  and reproduce just that one. Two lines of noise beats five.
- Only when no narrower anchor exists, and surrounding lines genuinely have to be
  reproduced, say in the first sentence which line the finding concerns, so
  "these lines" does not read as a complaint about all of them.

- Reproduce indentation exactly. The block is a literal replacement, not a patch.
- No `+` or `-` diff markers inside the block.
- Prose may sit above or below the block. Keep the reasoning in the prose and
  only the final code in the block.
- Suggestions live happily in a pending review. They become one-click
  committable once the review is submitted.

Skip the suggestion when the fix is not a mechanical edit to the anchored lines:
a change in another file, a design change, or anything the reader has to decide
before writing. Say it in prose instead.

## What the general comment contains

Exactly one thing: the findings whose file does not appear in the diff. Nothing
about the diff itself, which is what the inline comments are for.

**Write it as prose a person reads top to bottom.** Short and concrete, but
connected. Each finding is a sentence or two saying what is wrong, where, and
what it should say instead, and consecutive findings about the same thing belong
in the same paragraph rather than in separate items. Open a group with one
sentence telling the reader what kind of trouble it is about, then give the
detail.

**Keep it to something a reader takes in at a glance.** Four short paragraphs is
plenty. A body long enough to make the reader flinch before reading it has
already failed, however good the content. The evidence lives in the permalinks,
so state the finding and link the proof instead of walking through it: name the
conclusion, not the chain that got you there.

**Open with one sentence introducing the sections that follow.** Always, before
the first section title. It tells the reader what the sections below cover and in
what order, so the body reads as a whole rather than as loose blocks. One or two
sentences, no more.

That opening sentence introduces the findings in the body. It is not a preamble
about what was reviewed and not a verdict on the pull request, and the difference
matters: "Everything below sits outside the diff: documentation the change has
left behind, a suggestion on database credentials, and a couple of items for
later" belongs there, while "The approach is sound" and "Reviewed the diff plus X"
do not.

Group under plain descriptive section titles when there is more than one kind of
finding, for example "Docs to refresh" or "Two database users rather than one".
Prefer a title that says something over a one-word label.

**Set those titles in bold, `**Like this**`, not as `###` markdown headings.**
Bold reads as a section title without the visual weight a heading carries inside
a review comment.

Bullets are welcome wherever they genuinely help the reader: a set of parallel
findings, the concrete steps of a proposal, options to weigh side by side. A list
of four doc fixes reads far better as four bullets than as one dense paragraph.
Reach for one when the content really is a list. Do not expect it to save words:
a list trades the connective tissue between items for a self-contained sentence
each, so the gain is in scanning and comprehension, not in length.

What to avoid is the opposite extreme, a body made entirely of bold labels and
terse bullets with no prose holding it together: readable as data, unreadable as
an argument. Keep the sentences that explain and frame, and let the lists carry
the enumerable parts.

The body never contains:

- A preamble describing what was reviewed. "Reviewed the diff plus X and Y's
  internals" is banned.
- A verdict, summary, or overall assessment of the approach. Praise for a
  concrete choice belongs on the line that makes it. If the observation names
  `exec`, `set -eu`, a specific call, or any other thing that lives on a line in
  the diff, it is an inline comment, not a verdict. There is no PR-level
  paragraph that survives this test.

  This does not license a telegraphic body. The opening sentence of a group
  introduces the findings in that group, never the pull request. "The docs have
  not kept up with this change" frames findings and belongs there. "The approach
  is sound" judges the PR and stays out.
- Any pointer to the inline comments, such as "inline comments cover the diff"
  or "see the inline comment on the runtime stage".
- Any bullet restating something already said inline. If an inline comment
  already proposes the follow-up, the body stays silent about it.

Write the inline comments first, then build the body from what is left over.

## Code references

**Every mention of a file in the repo is a markdown link to a GitHub permalink.**
Not just the ones carrying a line number: naming `AGENTS.md`, `Cargo.toml` or
`mia-migration/README.md` in a sentence means linking it. A bare code span makes
the reader hunt for a file the review could have handed them. Link every
occurrence, including repeated ones, and add the line anchor whenever the finding
points at specific lines.

Three things are not file references and stay plain: a bare extension such as
"a new `.sql` file"; literal syntax quoted from a file, such as the `@AGENTS.md`
include line inside a `CLAUDE.md`; and a runtime filesystem path such as
`/app/Cargo.toml` inside a container, which names a location at execution time
rather than a file in the repository and has no permalink to point at.

- Repo files: pin to a commit SHA, the PR head SHA when reviewing a PR, never a
  branch name, so the link keeps pointing at the same line.
  `[runner.rs:21](https://github.com/OWNER/REPO/blob/<sha>/mia-migration/src/runner.rs#L21)`
- Ranges use `#L20-L25`.
- Third-party dependencies: link the upstream repo at the tag matching the
  pinned version, and confirm the line numbers at that tag rather than trusting
  a vendored copy such as `~/.cargo/registry`, since the two can drift.

**Verify every line number against the pinned ref before writing the link.**
Numbers written from memory come out off by one.

## Delivering it as a pending review

`gh pr review` cannot produce a draft: it requires `--comment`, `--approve`, or
`--request-changes`, all of which publish immediately. Use the API and omit
`event`:

```bash
gh api --method POST repos/OWNER/REPO/pulls/N/reviews --input review.json
```

`review.json` holds `commit_id`, `body`, and a `comments` array of
`{path, line, side}`, or `{path, start_line, start_side, line, side}` for a
range. No `event` key means the whole review, general comment included, stays
`PENDING`.

Inline comments only attach to lines inside a diff hunk. Take line numbers from
the head version of the file, not from the diff.

Confirm nothing leaked before reporting back:

```bash
gh api repos/OWNER/REPO/pulls/N/reviews -q '.[] | "\(.id) \(.state)"'  # expect PENDING
gh api repos/OWNER/REPO/pulls/N/comments -q 'length'                   # expect 0
```

To change a pending review, delete it and recreate it. GitHub allows one pending
review per user per PR, so a second POST fails while a draft exists.

```bash
gh api --method DELETE repos/OWNER/REPO/pulls/N/reviews/REVIEW_ID
```

## Red flags

Any of these means the review is not ready:

- An em dash somewhere in the review text.
- Any repo file named in the review text without being wrapped in a
  `[...](...)` link, whether or not it carries a line number.
- A line number written from memory instead of checked at the pinned ref.
- A body bullet whose file appears in the diff.
- A verdict or summary paragraph opening the body.
- An inline comment spelling out a concrete line edit in prose when a
  `suggestion` block would have been committable.
- A `suggestion` block whose anchor range does not cover every replaced line.
- The body opening with a description of what was reviewed.
- A body that jumps straight into its first section title with no sentence
  introducing the sections.
- An `event` field in the payload, or reaching for `gh pr review`.
- A review that ended up submitted without an explicit request to publish it.
- No `README.md`, `AGENTS.md`, or `CLAUDE.md` was read before writing the review.
- A body made entirely of bold labels and terse bullets, with no prose framing
  it. Bullets themselves are fine and often better; the absence of prose is not.
- A blunt or judging phrase anywhere: "this is wrong", "the worst is", "nobody
  did X", or a heading that names a fault instead of a topic.
- An architectural claim inferred from a name instead of read from the file that
  defines it.
- Any "I", "we", "us", "our", "you" or "your" in the review text.
- An observation that names a problem without saying what to do about it, or one
  whose proposal stops at "worth checking" without naming where and what.
