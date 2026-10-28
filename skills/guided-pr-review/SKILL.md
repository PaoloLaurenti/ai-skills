---
name: guided-pr-review
description: >-
  Use when asked to explain a pull request step by step, to walk through its changes
  before reviewing it, or to understand a change well enough to comment on it, including
  phrasings like "guidami nella PR #N", "spiegami la PR 94 passo passo", "fammi capire
  questa PR, devo farne la review", "walkthrough guidato della PR", "walk me through
  this PR", "help me review PR N", or "what does this PR really change". Not for
  answering the review comments already left on a pull request.
---

# Guided PR Review

The requester is about to review a pull request and wants to understand it first. The job
is to explain the change one concept per turn, at the level the requester picked, and to
turn each finding into a proposed review comment that is collected only when the requester
says yes to that comment. The collected review reaches GitHub once, at the last step, as a
pending review holding every comment and the general comment.

**One step per turn. No comment is collected without its own yes. Nothing is ever
published.** A walkthrough exists so that every concept is understood and every comment is
decided. A message that explains the whole change, or a "go on" read as consent, removes
exactly the reading the requester asked for.

## Non-negotiables

1. **One step per turn, then stop.** The turn has the shape given in
   [One step](#one-step) and ends on its questions, or on the invitation to go on.
2. **A comment is collected only after an explicit yes to that comment.**
   A word that moves the walkthrough on, such as "avanti", "go", "ok, next", approves
   nothing. A yes covers the comments it names, or all the comments of the current step
   when it says so, and never a comment not yet shown.
3. **GitHub sees the review once, at the last step, and nothing is published.** An
   approved comment goes into [the collected review](#the-collected-review) and that is
   all. At the last step the pending review is created in one call, holding every
   collected comment and the general comment, and it stays `PENDING`. No submitted review,
   no published comment, no reply, no general comment on the conversation, no edit to the
   pull request. Submitting happens only on an explicit request to submit or publish the
   review.
4. **The code is read, never changed.** Read it, run it to check a claim, and leave it as
   it is: no edit, no commit, no push, even where the fix is obvious and small. The
   requester is reviewing someone's change, not taking it over.
5. **Every comment follows `myai:writing-pr-reviews`.** **REQUIRED SUB-SKILL:** load it
   before the first step. It decides where the findings come from, how each comment is
   written, how it is anchored, and how the review reaches GitHub. This skill does not
   restate those rules, and where the two seem to disagree, that skill wins on the
   comment and this one on the conversation.
6. **A point already raised is cited, not proposed again.** When a comment on the pull
   request, pending or published, already covers a point, the step names that comment
   and proposes nothing new for it.

## Before step 1

In the first turn:

1. **Dispatch the reviewer**, in the background, as `writing-pr-reviews` requires. Step 1
   carries no comment, so it does not wait for the reviewer. When a later step needs the
   findings and the report is not back, wait for it there. When the report is lost, a
   restarted session for example, dispatch it again.
2. **Read what the steps will stand on**: the pull request's description and linked issue,
   the list of changed files, the checks, every review and comment already on it, pending
   and published, and the `README.md`, `AGENTS.md` and `CLAUDE.md` files covering what the
   change touches.
3. **Ask for the level, and nothing else in that turn**, as a closed menu:
   - **new to the repository**: every technical term explained where it first appears,
     and a step of basics when the change rests on patterns a newcomer would not know;
   - **knows the repository, not the technology**: a step of basics for the technology
     only;
   - **expert**: what changes and why, no basics.

Use the session's question widget where there is one. Otherwise ask in chat with lettered
options. The level decides how every step is written, so no step comes before the answer.

## The collected review

The review is built in a local file until the last step. Created on GitHub only once it is
whole, a draft never has to be deleted and recreated to take its general comment, which a
draft created with an empty body cannot be given. Kept in a file rather than in the
conversation, it survives a compacted context or a restarted session.

- **Where**: `~/.claude/reviews/OWNER-REPO-N.json`, outside any session so that a later
  one finds it. Written at the first approved comment and after every later one, and
  deleted once the review is on GitHub.
- **Shape**: the payload `writing-pr-reviews` posts. `commit_id` is the head SHA read
  before step 1, `body` stays empty until the last step, and `comments` holds each approved
  comment as `{path, line, side, body}`, with `start_line` and `start_side` for a range,
  anchored as that skill anchors one.
- **Held findings**: an approved finding outside the diff goes under a `held` key, its text
  as approved. The last step turns them into the general comment and removes the key.
- **A file left by an earlier session** for the same pull request is the review so far.
  Step 1 says what it holds, and the walkthrough adds to it. When its `commit_id` is not the
  current head, each anchor is checked against the new head before the last step uses it.

## Planning the steps

- **One concept per step**, named the way a reviewer would name it: "the bar now remembers
  whether it is visible", not "`Navbar.kt`". A concept may span several files and a file
  may feed several concepts.
- **In the order a reader needs**: the context, then the basics when the level calls for
  them, then the core change, then what builds on it, then the platforms, the tests, the
  documentation.
- **The total is fixed at step 1.** When the reviewer's findings need a step the plan does
  not have, add it once, and say so with the new total.
- **Each finding goes to the step of its concept**, whether it comes from the reviewer or
  from reading the change. Filter and check it first, as `writing-pr-reviews` says: a
  finding that resolves to nothing to change is not proposed.

## Step 1

Step 1 is these parts, in this order, and proposes no comment:

1. **Header**: `1/TOT` and a title, in the language of the conversation.
2. **Why the pull request exists**, from its description and issue, in plain words.
3. **What it touches**: the changed files grouped by role, with the size of the change.
4. **What is already known about it**: the checks, what the author says was tested by
   hand, and the review so far: an open pending draft and what it holds, the published
   comments and who left them, and a collected review left by an earlier session.
5. **The route**: every step, numbered, with its title.
6. **The invitation** to go on, naming the next step.

## One step

Every later step is these parts, in this order:

1. **Header**: `N/TOT` and the concept, in plain words.
2. **The explanation**, at the level picked: what changes, where (`file:line`), and why.
   Bold leads, short paragraphs and lists, never a wall of text. Explain; do not quiz the
   requester.
3. **Points of attention**, one block each:
   - the point, in plain words, in the language of the conversation;
   - the proposed comment, in full, with the line it anchors to, written per
     `writing-pr-reviews`;
   - or, when a comment already covers the point, that comment cited instead, pending or
     published, and no proposal.
4. **Considered and set aside**: one line for each candidate dropped, with the reason.
   Leave the part out when nothing was dropped.
5. **One question per proposed comment**, each a closed menu: *add* / *add with this
   change* / *drop* / *dig in*.
6. **The next step**, its number and title, and the word that moves on.

A step with no proposal has parts 1, 2, 4 when there is something set aside, and 6.

A finding about a file outside the diff is shown in its step like any other, with the
same menu, but it cannot be anchored: an approved one is held for the general comment. See
[The last step](#the-last-step).

## After an answer

- **Add**: write the comment into [the collected review](#the-collected-review), under
  `comments`, or under `held` when its file is outside the diff. Read the file back, then
  say in one line that the comment is collected and how many the review now holds.
- **Add with a change**: apply the requester's change, check the result against
  `writing-pr-reviews` again (English, impersonal, no em dash, a proposal in it), show the
  final text, and add that.
- **Drop**: add nothing, and list the point as set aside.
- **Dig in**: read further, run what can be run, present what was found, and ask again.
- **A word that moves on while a comment of the step is undecided**: add nothing. Name the
  undecided comments and ask about them again. The next step comes when every comment of
  this one has an answer.
- **"Stop"**, or its equivalent: go straight to the last step.
- **A question about the code**: answer it, then ask again whatever is still undecided.

Once every comment of the step is decided, give the next step in the same message,
opening with one line on what was added.

## The last step

The last step is the only one that touches GitHub. It is these parts, in this order:

1. **The general comment**, when findings were held: written once, now, from them, as
   `writing-pr-reviews` shapes it, and shown in full.
2. **What the review will hold**: each collected comment, one line each with its file and
   line, and the total.
3. **What was left alone**: a draft that was already open, and the comments it holds,
   untouched.
4. **The question on the general comment**, when there is one, closing the step like any
   other: *add* / *add with this change* / *drop*.

Once the general comment is decided, or at once when nothing was held, the review is
delivered as `writing-pr-reviews` delivers one, in this order:

1. **Settle the body**: the approved general comment, or empty when it was dropped or never
   needed. Remove `held` from the file.
2. **No draft open**: one `POST` of the file, which creates the pending review with every
   comment and the body together. A `422` on an anchor creates nothing: fix that anchor
   and send again.
3. **A draft already open**: append each collected comment to it, and the body after its
   own when it has one. When its body is empty, it cannot take one: say so and hand over
   the text.
4. **Read it back**, delete the local file, confirm that nothing was published, and state
   the submit command, never run. Submit only on an explicit request to submit or publish.

## Red flags

Each of these means the walkthrough has gone wrong. Stop and return to the rule it breaks.

| Thought | Reality |
| --- | --- |
| "They said 'avanti', so the comments I proposed are approved" | "Avanti" moves on. It approves nothing, and the step does not move on while a comment is undecided. |
| "They said add, so I send the comment to GitHub now" | Add writes it into the collected review. GitHub sees the review once, at the last step, still `PENDING`. |
| "The general comment changed the draft, so I delete it and create it again" | The draft is created once, after the general comment is settled, so there is nothing to recreate. A deleted draft leaves its inline comments behind as orphans. |
| "The review looks complete, I will submit it" | Never. State the submit command. |
| "The fixes are obvious, I will commit them for the author" | The requester is reviewing. The code is read and never changed. |
| "The change is small, one message covers it" | Size does not change the rule. One concept per step. |
| "I will close with a verdict" | The requester decides. The walkthrough explains and proposes comments. |
| "Asking questions will teach the method better" | Explain. The only questions are the level and the decisions on comments. |
| "The level is obvious from the request" | Ask it. The answer decides how every step is written. |
| "This point matters, I will propose it even though a comment covers it" | Cite the existing comment. A second one on the same point is noise. |

Also a red flag: a step that ends without its questions while a comment is proposed, a
comment collected before its yes, a call to GitHub that writes before the last step, and a
total that changes more than once.
