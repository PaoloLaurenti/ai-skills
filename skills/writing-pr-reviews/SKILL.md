---
name: writing-pr-reviews
description: >-
  Use when asked for a review, an opinion, or feedback on a pull request in any
  repo or directory, including phrasings like "dammi un parere sulla PR #N",
  "review PR 16", "guarda questa PR", "che ne pensi di questa PR", or when
  preparing, posting, or editing review comments on a GitHub PR. Also when
  following up on a review already submitted, once the author has replied or
  pushed new commits, including phrasings like "l'autore ha risposto alla mia
  review", "controlla le risposte e i nuovi commit sulla PR #N", "ha sistemato
  quello che ho chiesto?", "follow up on my review of PR #N", or "did they
  address my comments".
---

# PR Review Conventions

Standing preferences for how a pull request review is written and
delivered. These override default review habits. Finding the defects is delegated,
per non-negotiable 7, so what follows is what happens to what that step returns.

## Non-negotiables

1. **ALWAYS create the review in PENDING state. Never submit it.** The general
   comment and every inline comment stay pending, visible only to the account
   that created the review, until it gets submitted by hand. Create it pending,
   then state the command to submit. The only thing that changes this is an
   explicit request to submit or publish the review; a request to write, prepare,
   add, or leave a review is not that, and neither is a review that reads as
   ready to go. A follow-up on a review already submitted holds to the same rule:
   each reply to one of its threads goes into a pending draft, never through the
   endpoint that answers a comment directly, which publishes at once. Resolving a
   thread is the one change made on GitHub straight away, and only after an
   explicit yes to that thread's question.
2. **Never delete or rewrite a draft this run did not create.** A pending review
   already open on the pull request is appended to, never replaced. A comment
   already in it that breaks these conventions stays exactly as written: leave it
   untouched, and say in the reply which comment it is and which rule it breaks.
   That decision belongs to whoever asked for the review, not to the review.
3. **Write the review in English**, even when the conversation is in Italian.
4. **Never use em dashes** (the `—` character) anywhere in the review: not in the
   general comment, not in inline comments. Restructure the sentence instead.
   Commas, colons, parentheses, or two shorter sentences all work.
5. **Impersonal forms only.** No "I", "we", "us", "our", "you", "your" anywhere
   in the review. Use "worth ...", "the suggestion is to ...", passive and gerund
   constructions instead. This holds even where a first-person plural would sound
   collegial: "we are in good shape today" becomes "today's position is fine",
   and "gives us a way in" becomes "leaves a way in".
6. **Be short and direct.** No filler, and every comment stays as short and as
   plainly worded as it can be while still being actionable. See One point per
   comment for the shape that takes.
7. **The findings come back from a dispatched reviewer subagent**, through
   `superpowers:requesting-code-review`, run first. Loading that skill is not the
   requirement, dispatching is, and nothing gets written before the subagent
   returns. An ask that already names the finding is the exception, since there is
   nothing to go looking for. See Where the findings come from.

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
  down" and "crash loop with no way in" all say it more coldly than needed.

Warmth never costs precision or adds padding: it comes from word choice, not from
added praise or hedging.

## One point per comment

A review is scanned between other work, not read through. Every comment is
written for a colleague giving it a few seconds, so it says one thing, in the
plainest words that still carry it.

**Keep an inline comment's prose under 80 words, and most of them well under 50.**
Code inside a `suggestion` block does not count: it is the fix, not the
explaining. Going over is the signal to cut, and there are only three things a
comment needs:

1. what is wrong, on the line it is wrong on;
2. why it matters, in one sentence, only where it is not obvious;
3. what to do instead.

Everything past those three is context the reader did not ask for. Cut the
reasoning that led to the finding, the alternatives considered and rejected, and
the second and third implications of the change. The reasoning still gets done;
what stays out of the comment is the working, not the work.

- **One comment makes one point.** A comment carrying two findings is two
  comments, each on the line it belongs to.
- **Short sentences, ordinary words.** No nested clauses, no sentence that has to
  be read twice. Prefer the word a colleague would say out loud.
- **Say the thing, then stop.** A comment that has already given the fix does not
  also need a summary of itself.
- **The alternative is not a comment.** Where a point genuinely needs several
  paragraphs, it is a design discussion, and it belongs in the general comment or
  in an issue rather than anchored to a line.

**The general comment is measured per finding, not as a whole.** It legitimately
carries several, so a total word count would only force one of them out. Each
finding in it gets the same 80 words and the same three parts as an inline
comment, and the prose introducing the sections stays to a sentence or two.

## Pragmatic and concrete

Every comment, extremely so. A comment proposes a concrete change to concrete
code, in words the reader can act on without translating them first.

- **No advice that would read the same on another pull request.** A sentence that
  survives being pasted onto a different diff says nothing about this one.
- **No principle by name.** Say what breaks and what to change instead, not that
  something violates DRY, separation of concerns, or single responsibility.
- **The value, the setting, the call, the replacement wording.** "Cache the
  compiled regex in a `OnceLock`", never "this could be more efficient".
- **Nothing hypothetical.** What someone might one day do to this code is not a
  finding.

## Where the findings come from

A reviewer subagent, dispatched through `superpowers:requesting-code-review` before
any comment is written. What comes back is raw material, not the review itself.

Fill its template from the pull request: `BASE_SHA` is the merge base with the
base branch, `HEAD_SHA` the head SHA of the pull request, `DESCRIPTION` what the
diff does, `PLAN_OR_REQUIREMENTS` the pull request body plus anything it links.

Everything it returns then passes through the rest of this file, and none of the
shape it arrives in survives:

- **Its output format is not this review's.** Strengths, Recommendations,
  Assessment and the Critical, Important and Minor headings organise a report to
  whoever asked for it. None becomes a body section or a comment. Where a strength
  names something on a line in the diff, it may survive as a short inline note,
  per Tone.
- **Filter first, rewrite second.** A finding resolving to nothing to change, or a
  nit too small to earn attention, comes out here rather than getting reworded.
  What survives is rewritten to these conventions: impersonal, one point per comment,
  pragmatic, permalinked, carrying a proposal, and anchored inline where its file sits.
- **Its severity ranking decides nothing.** Critical or Minor says how the
  subagent ranked a finding, not whether it belongs in this review.

That subagent was handed the change rather than the repository around it, so the
two sections below stay this review's own work, and every claim it makes about how
the system fits together is verified here rather than trusted.

## Choosing how the review is built

Once the subagent's findings are filtered, ask the requester how to build the review,
naming how many points survived and the weightiest of them: create the pending draft now
with all of them, or go through them one at a time before it is created. Skip the
question, and create the draft, when the request already says which or when a single
point survived.

- **One at a time**: one point per turn, in this order, and nothing about any other:
  1. `Point n of N`, with the file and the line linked at the head SHA.
  2. The comment exactly as the draft would carry it, in a fenced block, its `suggestion`
     included.
  3. Why it matters and an opinion, in two or three lines in the conversation's language.
  4. One question: add, add with a change, or drop.
- **After an answer**, the comment goes into the collected review `guided-pr-review`
  keeps in `~/.claude/reviews/OWNER-REPO-N.json`, in its shape: under `comments`, or under
  `held` when its file is outside the diff. Never to GitHub. The next point follows in
  the same message.
- **The last turn** shows the general comment written from `held`, with its question.
  Then one pending draft is created from the file, or its comments join a draft already
  open, the file is deleted, and the leak check and the submit command follow, never run.

## Check the diff against the repo's own docs

Every review checks whether the change still agrees with the documentation the
repo keeps about itself. This is mandatory and easy to skip, because these files
usually sit outside the diff and nothing in the diff points at them.

Find them at any depth with the command in
[reference/reading-the-repo.md](reference/reading-the-repo.md), which also catches the
per-package ones nothing points at, and read the parts the change touches at the PR
head SHA rather than the working tree, so the comparison matches the PR.

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

**Documentation does not carry values that live in the code.** A port, a coverage floor,
a task name, a version, a count, a list of the places something appears: each is owned by
the file that sets it, and prose repeating it is a copy with nothing keeping it true. It
goes stale the next time the code moves, silently. So no comment proposes prose carrying
such a value, in either direction: not when documenting something new, and not when a
sentence has already drifted, where the fix is removing the value rather than updating
it. Say what the thing does, name the file that sets it, and where a test holds the two
sides together, point at the test instead of repeating what it asserts.

The line is between a value and a claim. A value is a token copied out of the code, a
number, an identifier, an entry in a list, that a routine change rewrites with no
decision behind it. A claim about what the system does goes stale too, but only when the
design moves, and stating those is what documentation is for. This binds what is
proposed, never what the comment may cite: reading the owning file, quoting it and
linking it are unchanged.

**Never write "the docs" generically.** Name the files: the root `README.md`, a
package's `README.md`, `AGENTS.md`. A reader cannot act on "the docs do not cover
this" without first guessing which file is meant. Where a `CLAUDE.md` is only an
`@AGENTS.md` include, say so, so nobody edits it twice.

Report wording that has merely become misleading, not only outright contradictions.
When a doc file is itself in the diff, comment inline with a `suggestion` block
carrying the corrected prose; when it is not, the finding goes in the general comment.

## Verify every factual claim about the architecture

A review states facts about how the system fits together: what runs where, what a
container or image is, what a name refers to, what talks to what, which
credentials are in play. Read the thing that defines each one before writing it.
Never infer it from a crate name, a service name, or the shape of the diff.

The failure mode is a claim that sounds obviously right and is wrong. A crate
called `mia-identity` does not make the deployed container mia-identity's: that
name lives in the deploy workflow and the ECR repository, and the image may be one
per project rather than one per service. Such a claim passes every mechanical
check on the review, so only reading the source catches it.

[reference/reading-the-repo.md](reference/reading-the-repo.md) lists where each kind of
claim is actually answered: open it before stating one, since the obvious place is
usually the wrong one. When the answer lives somewhere this repo cannot see, write the
finding as something to verify and name where to look, rather than stating it as fact.

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

Check the direction of a robustness claim too, since it inverts easily. A relative
path follows `WORKDIR` and an absolute one pins a location, so proposing
`/app/bin/x` over `./bin/x` "so it does not depend on WORKDIR" gets it backwards.
Name the scenario the change protects against, then check whether anything
produces it.

Where two edits only work as a pair, say so in both comments. A suggestion that
breaks the build when committed on its own is worse than no suggestion.

A proposal is finished when the reader knows what to do next without having to
ask. For anything outside this repo that means naming where to look and what to
look for. "Needs eyes on that side" is not a proposal. "Look up the value in
Pulumi, then size it for the slowest migration you expect" is.

Where two answers are both reasonable, give both and say what each costs, rather
than leaving the choice unstated.

**A finding that resolves to "nothing to change" does not belong in the review.**
This holds for inline comments exactly as much as for body sections. Verifying
something and discovering it is already fine is good work, but the result is a
message to the author, not a review comment. Either it converts into a real ask,
most often "worth writing this down" folded into a documentation finding, or it
comes out. Say it in the reply instead, or, where the explanation is worth having,
attach it as the reason behind a comment that does ask for something.

**Quantify a nit before raising it, and drop it when the number is small.** Work out
what the change actually buys: bytes saved, milliseconds, one layer. If the honest
answer is "383 bytes and one layer in a hundred-megabyte image", it does not earn a
comment, however correct it is. Attention spent there is lost to what matters.

Nor does something the code already documents: a review comment restating a code
comment two lines above, or proposing a `TODO` that adds only a wish, is noise.

## Code suggestions

Whenever the fix is a concrete edit to lines that sit in the diff, write it as a
committable suggestion instead of describing it in prose. Always reach for one
where it makes sense.

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

Skip the suggestion when there is no replacement text worth handing over verbatim:
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

For example: "Everything below sits outside the diff: documentation the change has left
behind, a suggestion on database credentials, and a couple of items for later".

Group under plain descriptive section titles when there is more than one kind of
finding, for example "Docs to refresh" or "Two database users rather than one".
Prefer a title that says something over a one-word label.

**Set those titles in bold, `**Like this**`, not as `###` markdown headings.**
Bold reads as a section title without the visual weight a heading carries inside
a review comment.

Bullets are welcome where the content really is a list: parallel findings, the steps of
a proposal, options side by side. They save scanning, not words, and a body of bold
labels and terse bullets with no prose holding it together reads as data rather than as
an argument. Keep the sentences that explain and frame.

The body never contains:

- A preamble describing what was reviewed. "Reviewed the diff plus X and Y's
  internals" is banned.
- A verdict, summary, or overall assessment of the approach. Praise for a
  concrete choice belongs on the line that makes it. If the observation names
  `exec`, `set -eu`, a specific call, or any other thing that lives on a line in
  the diff, it is an inline comment, not a verdict. There is no PR-level
  paragraph that survives this test.
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

**Read [reference/delivering-a-pending-review.md](reference/delivering-a-pending-review.md)
before posting anything.** It carries every command this section refers to. Two of them,
the submit endpoint and the GraphQL append, come out wrong when reconstructed from memory,
and both failures have happened.

The rules the commands serve:

- **Check whether a draft is already open before creating one.** Nothing open means create
  it. Something open means append to it, per non-negotiable 2. A `422 User can only have
  one pending review per pull request` is that check arriving late, and never a reason to
  delete.
- **Never `gh pr review`.** It requires `--comment`, `--approve` or `--request-changes`,
  all of which publish immediately. The API with no `event` key is what keeps a review
  `PENDING`.
- **Confirm nothing leaked before reporting back**, then state the submit command rather
  than leaving it to be reconstructed.
- **Anchor inline comments inside a hunk of `gh pr diff N`**, taking line numbers from the
  head version of the file. A local `git diff` limited to one path loses a rename and shows
  the whole file as new.

## Following up on a review already submitted

When the author has replied or pushed after a review from the requester's account was
submitted, the job is to take the requester through what came back one thread at a time,
and to deliver what is decided as a pending draft. Every rule above binds the replies
written here.

**Read [reference/following-up-a-review.md](reference/following-up-a-review.md) before the
first API call.** It carries the threads query written for the reviewer's side, the
reviewed SHA, the pending reply and the resolve command. The endpoint that answers a
thread directly publishes at once, and it is the one memory reaches for.

- **The threads**: every unresolved thread the requester's review opened, replied to or
  not, ordered by file and line. One with neither a reply nor a change at the new head
  stays out of the count and goes to the final turn. The count is fixed at the start, and
  it changing more than once means the collection was wrong.
- **One thread per turn**, in this order, and nothing about any other thread:
  1. `Thread n of N`, with the file, the line and a link to the thread.
  2. The original comment and the author's reply, quoted.
  3. What changed on the thread's lines since the reviewed SHA, with a permalink at the
     head SHA.
  4. One outcome, accepted, partly accepted, not accepted or unchanged, with what was
     checked to reach it.
  5. The proposed action: resolve, reply with the text in full in a fenced block, or leave
     open.
  6. One question on that action.
- **After a yes**, a thread to resolve is resolved at once, and a reply goes into the
  collected file, never to GitHub. The next thread follows in the same message.
- **The final turn**: the threads left out of the count, whether the description still
  matches the head, and the commits or files changed since the reviewed SHA that no thread
  touches, with the question whether to review them. A yes is a normal review of
  `reviewed SHA..head`, non-negotiable 7 included, and its comments join the same file.
- **Delivery**: the replies are collected in `~/.claude/reviews/OWNER-REPO-N.json`, the
  file `guided-pr-review` uses, under a `replies` key, each as the thread id and the text.
  After the final turn one pending draft takes every reply, the file is deleted, and then
  come the leak check and the submit command from the section above, never run.

## Red flags

Any of these means the review is not ready:

- An em dash somewhere in the review text.
- Any repo file named in the review text without being wrapped in a
  `[...](...)` link, whether or not it carries a line number.
- A line number written from memory instead of checked at the pinned ref.
- A body bullet whose file appears in the diff.
- A verdict, a summary or a description of what was reviewed opening the body.
- A line edit spelled out in prose where a `suggestion` block was committable.
- A `suggestion` block whose anchor range does not cover every replaced line.
- A body that jumps straight into its first section title with no sentence
  introducing the sections.
- A review submitted without an explicit request, an `event` field, or `gh pr review`.
- A draft open before the review started deleted, rewritten or edited to fit these
  conventions, or a `DELETE` used to get past a `422`.
- No `README.md`, `AGENTS.md`, or `CLAUDE.md` was read before writing the review.
- A doc suggestion carrying a value the code owns: a port, a floor, a version, a
  task name, a count.
- A body made entirely of bold labels and terse bullets, with no prose framing it.
- A blunt or judging phrase anywhere: "this is wrong", "the worst is", "nobody
  did X", or a heading that names a fault instead of a topic.
- An architectural claim inferred from a name instead of read from the file that
  defines it.
- Any "I", "we", "us", "our", "you" or "your" in the review text.
- An observation that names a problem without saying what to do about it, or one
  whose proposal stops at "worth checking" without naming where and what.
- An inline comment whose prose runs past 80 words, or one carrying two findings
  where two comments were wanted.
- A sentence that has to be read twice, or a word chosen over the plainer one a
  colleague would have said.
- No subagent dispatched, a comment written before it returned, or a review carrying
  its Strengths, Recommendations, Assessment or severity headings.
- Points walked one at a time unasked, or a draft created before the requester chose.
- A comment that would read the same on another pull request, or one naming a
  principle where it could have named the change to make.
- A comment explaining how a finding was reached rather than what to do about it.
