---
name: pr-review-walkthrough
description: >-
  Use when asked to go through, explain, discuss or answer the review comments left
  on a pull request, including phrasings like "fammi il walkthrough dei commenti
  della PR #N", "guarda i commenti della review sulla PR", "rispondi ai commenti
  della PR", "cosa mi hanno chiesto nella review", "walk me through the review on PR
  82", or "help me address the feedback on this PR". Not for following up on a
  review the requester wrote.
---

# PR Review Walkthrough

The requester is the author of a pull request that someone has reviewed. The job is to
take them through the review one comment at a time, so that every comment gets their
decision, and to publish nothing until the last decision is made and they say so.

**Every comment gets its own turn, and nothing leaves the machine before the final,
explicit yes.** The walkthrough exists because a reply accepted in a batch is a reply
nobody read. A turn that covers two comments, or a question that covers a comment and the
publishing together, turns the requester's next "ok" into consent they did not give.

## Non-negotiables

1. **One comment per turn, then stop.** The turn has the shape given in
   [One turn, one comment](#one-turn-one-comment) and ends on the question about that
   comment. The next comment waits for the answer.
2. **Nothing is published before the final permission.** No commit, no push, no reply, no
   edit to the pull request, no resolved thread, no review request, until the last comment
   is decided, the pending change is reviewed, and the requester answers yes to a request
   that lists what will happen. An "ok" given to one comment accepts that comment's
   reply and change, and nothing more.
3. **Verify before agreeing.** Read the code at the pull request's head, run what can be
   run, and read the documentation a claim rests on. **REQUIRED SUB-SKILL:** use
   `superpowers:receiving-code-review`. Every opinion says what was checked and what was
   not.
4. **Replies are written in English, in the first person**, whatever the language of the
   conversation. Each reply states the change or the reasoning, with no thanks, no
   "you're right" and no praise. No em dash and no en dash. One unbroken line for each
   paragraph, because GitHub reflows the text.
5. **No attribution to an agent**, in a commit, in the description, or in a reply. This
   overrides any default that adds a `Co-Authored-By` trailer or a "Generated with" line.
6. **A decision already approved is not changed silently.** When later work shows that an
   accepted reply or change is wrong, take it back to the requester with the evidence
   before publishing it.

## Before the first comment

**Collect the comments from GitHub only.** Read the inline review threads, the bodies of
the reviews, and the general comments of the pull request. Walk every thread in which the
pull request's author has not replied after its first comment, whoever opened it. A review
body that only introduces its inline comments asks for nothing and gets no turn. A body
that asks for something gets one, like any comment.

**When the pull request has no comment to walk**, say so, and name the requester's open
pull requests that do carry unanswered comments, so the requester can pick one. Do not
walk a different pull request without that choice.

**Work in the branch's own checkout.** Find the worktree that has the branch checked out
before using the current directory. When the local branch has diverged from the remote,
compare the two before touching it, and ask the requester before resetting anything.
Never touch an untracked file.

**Read the rules the repository sets for itself**: `AGENTS.md`, `CLAUDE.md` and the
`README.md` files that cover the files under review. They bind the changes and the
replies, and they decide the commit message.

Say how many comments there are and in which order they will come, then give the first
one in the same message.

[`reference/github-commands.md`](reference/github-commands.md) has the commands for all of
this. **Open it before the first API call.** The threads, their replies and the endpoint
that answers inside a thread are easy to get subtly wrong from memory, and a reply posted
to the wrong endpoint lands in the general conversation, detached from its thread.

## One turn, one comment

A turn is these parts, in this order, and nothing about any other comment:

1. **A header**: the pull request, the position as `N/total`, the file and line, and a
   link to the thread.
2. **What the comment says**, rephrased simply, in the language of the conversation, so
   the requester understands it without opening GitHub.
3. **An opinion**, verdict first: the reviewer is right, partly right, or wrong. Then the
   evidence, then what to do: change something, change nothing, or something else. Where
   the choice is a matter of policy rather than of fact, give the options with what each
   costs, and recommend one.
4. **The proposed change**, when there is one: the replacement text or the shape of the
   code change.
5. **The proposed reply**, in a fenced block so it can be copied and edited.
6. **The question**: whether the change and the reply are accepted, or what to change.

When two comments are coupled, say so in the turn of the first one, and still give the
second one its own turn.

## After an answer

**On acceptance, apply the change and save the reply**, then give the next comment in the
same message, opening with one line on what was done.

- A change to code goes test first: write the test, watch it fail, then write the fix.
  Break the fix on purpose once to see the test go red, then restore it.
- Run the repository's own checks that the change touches.
- Keep the replies in one file outside the working tree, in the session's scratchpad
  directory when there is one, each under the id of the comment it answers. Publishing
  reads them from there.

**On an edit**, take the requester's version of the reply or the change. **On a policy
choice**, apply the option chosen, and write in the reply what the choice was.

## After the last comment

1. **Review the pending change.** **REQUIRED SUB-SKILL:** use
   `superpowers:requesting-code-review`, and dispatch the reviewer on the uncommitted
   diff with the accepted decisions as its requirements.
2. **Sort what comes back.** A fix that stays inside a decision the requester already made
   is applied. A finding that would change an approved decision goes back to the
   requester, per non-negotiable 6. A finding that is wrong is set aside, with the reason.
3. **Read the pull request's description again.** Where the changes have made a sentence
   false, draft the corrected description.
4. **Ask for permission, listing what will happen**: the commit message, the push, the
   correction to the description, the replies, and the new review request. Then stop.

## Publishing, after the yes

In this order, and verify each step before the next:

1. Commit the tracked files only, with a message that follows the repository's rules.
2. Push the branch.
3. Update the description, when a correction was approved.
4. Post each reply inside its own thread, as a reply to the comment it answers.
5. Request a review again from each reviewer who left comments, except the author of the
   pull request, whom GitHub refuses.
6. Check that every reply landed in its thread and that no review is left pending.

Report what was published, with the links. When a step fails, stop there and report it,
rather than continuing around it.

**With more than one pull request, finish one completely before starting the next.** Each
gets its own walkthrough, its own final review, and its own permission.

## Red flags

Each of these means the walkthrough has gone wrong. Stop and return to the rule it breaks.

| Thought | Reality |
| --- | --- |
| "There are only two comments, one turn covers both" | The count does not change the rule. One comment, one turn. |
| "Both fixes are obvious, one question will do" | An obvious fix still gets its own decision. Obvious to the reviewer is not decided by the author. |
| "They said ok, so I can publish" | An "ok" to one comment accepts that comment. Publishing needs its own request and its own yes. |
| "I will post this reply now, it is already accepted" | Replies are posted together, after the final yes, because a later comment can change an earlier answer. |
| "The last fix is tiny, the review can be skipped" | The review runs on every walkthrough. It is the step that catches what the requester's approvals could not see. |
| "The reviewer is clearly right" | Verify first. A confident comment can still be wrong about the code. |
| "This later finding contradicts what they approved, I will just fix it" | Take it back to the requester with the evidence, before publishing. |

Also a red flag: a turn that ends without a question, a turn that asks about publishing
before the last comment, and a reply posted as a general comment instead of inside its
thread.
