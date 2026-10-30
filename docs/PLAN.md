# Plan

Work queued for this repo, ordered for execution. It comes out of an audit
against Anthropic's [skill authoring best
practices](https://platform.claude.com/docs/en/agents-and-tools/agent-skills/overview),
the `superpowers:writing-skills` guidance, and the [Claude Code plugin
docs](https://code.claude.com/docs/en/plugins).

Two things are open: the move to `claude plugin eval`, blocked on Anthropic rather
than on this repo, and the `tdd` skill, being designed in entry 7.

Already in line, so absent from this plan: the `description` field states
triggering conditions without summarising the workflow, trigger phrasings reach
the loader in both languages, the skill body sits under the 500-line limit, and
the Red flags section gives the skill its self-check list. That second item was
recorded here as fine while it was not, which the entry on the truncated
description below explains.

## 1. Move the suite onto `claude plugin eval`

Blocked on Anthropic, not on this repo.

`claude plugin eval` takes `evals/**/case.yaml`, or `evals/**/prompt.md` plus
`graders/*.md`, resolves a skills-directory plugin by name, and carries
`--ablation with-without` for the baseline arm, `--runs` for repetition,
`--judge-model`, an HTML report and `--max-cost-usd`. That is, off the shelf, exactly
what has been built by hand here, including the two parts that were hardest: the
baseline arm and bounding the spend.

- [ ] migrate once it leaves early access. Still gated at CLI 2.1.224, re-probed on
      2026-08-07: `claude plugin eval init --bare` answers "`plugin eval` is currently in
      early access", exactly as it did at 2.1.223 the day before, while `--help` prints
      the full option list. That is the first version bump probed under the rule this
      entry set for itself, probe the version rather than the date, and it moved the
      version without moving the gate.

Two details in that help output worth carrying into the migration, because both are
decisions this suite made by hand and could lose by accident. `--allow-tools`, with its
`Tool(pattern:*)` syntax, is where the deliberate tool list goes; the eval README argues
at length against reaching for `--dangerously-skip-permissions`, and that argument has to
survive the move. And `--threshold`, which exits non-zero below a score, is what would
turn the `AGENTS.md` obligation to rerun a skill's scenarios after editing it into
something mechanical rather than a promise. The help also confirms in writing that
skills-dir plugins resolve by name and default to the `with-without` ablation, so the
baseline arm this suite built by hand comes free.

What already exists is the input it wants: the fixture, the prompts, and the assertions
written into each scenario file.

Deliberately not doing: a checker of its own for the mechanical assertions. It was
queued here as the obvious next tool, and `claude plugin eval` shipping graders as
`graders/*.md` with a judge model makes it work to be thrown away. The assertions
written into each scenario file are the specification those graders will want, which is
the part worth having either way.

## 2. Clean up the inline comments a deleted draft leaves behind

*What happened.* A draft review was created before a guided walkthrough ended, then
deleted with `DELETE repos/OWNER/REPO/pulls/N/reviews/REVIEW_ID` and recreated three
times. Each `DELETE` succeeded and the review left `pulls/N/reviews`, but about an hour
later the GraphQL `reviewThreads` still listed 24 inline comments in state `PENDING` with
`pullRequestReview: null`. Five, from the oldest draft, already answered `404`; the other
19 needed `DELETE repos/OWNER/REPO/pulls/comments/COMMENT_ID` each. "Confirm nothing
leaked" missed them, because `pulls/N/comments` returns submitted comments only.

*Decided.* Delete-and-recreate stays the one way to change a draft this run created:
collecting a guided review locally makes it rare, and one path with a cleanup step is
simpler than a second path of in-place GraphQL edits. The cleanup is narrow, so
non-negotiable 2 holds: the draft's comment ids are read before the `DELETE`, and only
those are deleted after it, a `404` counting as already gone. Orphans of any other origin
are reported, never deleted.

- [x] Prove it on the fixture first, outside the suite: create a draft with two inline
      comments, delete it, query the orphans at once, run the cleanup, run the check.
      Write the outcome next to the cleanup step, orphans or not and after how long, so
      the step says when it is needed. No eval scenario: this checks commands rather than
      agent behaviour, and a prompt would have to force a path the skills now avoid.

      Done on 2026-10-28, on fixture PR 7, twice. Every `DELETE` left both inline
      comments as orphans at once, not after a delay, so the cleanup is needed every
      time. They did not attach to the draft created next, which took only its own
      comments. Deleting the ids read before the `DELETE` removed them, and the orphan
      query then returned nothing. One more zsh trap surfaced: `for id in $IDS` does not
      split on newlines in zsh, so the loop reads the ids with `while read -r id`.
      Revised by the probe under item 4: on the fixture the orphans were gone five
      seconds later by themselves, so the cleanup is insurance for the times GitHub's own
      does not happen, as in the run above, rather than needed every time.
- [x] In `reference/delivering-a-pending-review.md`, "Changing a draft this run
      created": read `pulls/N/reviews/REVIEW_ID/comments` ids before the `DELETE`, delete
      those after it.
- [x] In "Reporting back", a third check beside the two that exist, expecting no output:
      the `reviewThreads` query filtered on `state=="PENDING"` and `pullRequestReview==null`.
      `reviewThreads(first:100)` without pagination is accepted.
- [x] In `guided-pr-review`, complete the red-flag row "The general comment changed the
      draft, so I delete it and create it again" with the consequence: a recreated draft
      leaves its inline comments behind as orphans. The rule is already there; the reason
      is what makes it hold under pressure.

## 3. Take the hunks from GitHub's diff, not from a local one

*What happened.* A review `POST` failed with `422 Line could not be resolved` for two
comments on a file the pull request renamed and edited. `git diff BASE HEAD` showed the
file as entirely new, so every line looked anchorable; GitHub applied rename detection and
showed a few small hunks, and the two lines sat outside them. Why the two diffs disagreed
is not known: `git diff` detects renames by default, so a different base (base tip against
merge base) or a different similarity threshold are both plausible.

- [x] Reword in place the body bullet "Anchor inline comments inside a diff hunk" in
      `writing-pr-reviews`: the hunks are the ones `gh pr diff N` shows, since a local
      `git diff` can disagree on renames. At most one line, inside the budget.
- [x] Put the listing in the reference, beside the sentence on hunks that is already
      there: `gh pr diff N | awk '/^diff --git/{f=$4} /^@@/{print f" "$0}'`.
- [x] Try to reproduce the disagreement on a fixture branch of its own. Only if it
      reproduces, add a scenario asserting no `422` and every anchor inside a
      `gh pr diff` hunk. If it does not, record the cause as open here.

      Done on 2026-10-28, on fixture PR 9 (`rename-head` into `rename-base`, so `main`
      is untouched), with four files renamed at about 70, 45, 30 and 15 percent
      similarity. GitHub's threshold is git's: `gh pr diff` and the files API saw only
      the 70 percent file as a rename, exactly as a local `git diff` did, so the threshold
      hypothesis is out. The disagreement reproduces another way: `git diff BASE HEAD --
      <new path>`, and `git show HEAD -- <new path>` alike, show the renamed file as
      entirely new, one hunk from line 1, because rename detection needs the old path
      inside the pathspec too. The whole diff and `gh pr diff` both showed one hunk,
      lines 26 to 40. An anchor on line 5 got `422 Line could not be resolved`, one on
      line 30 was accepted. That the original run diffed one path at a time is the
      likely cause, not a verified one.

      No scenario came of it, because no baseline fell for it. A draft scenario 09 put a
      40-line file on a base branch, renamed it, and added a handler at its end, leaving
      one hunk, lines 33 to 43, and a docstring above it to anchor wrongly. Opus with the
      plugin disabled was asked twice, for an opinion on the whole pull request and on
      `app/routes.py` alone. Both times it read the pull request from GitHub first, whose
      file list marks the rename, used `gh pr diff` (the second time after `gh pr diff N
      -- path` refused the path), never diffed one path locally, cited only lines inside
      the hunk, and posted nothing. A scenario that passes without the skill tests
      nothing, so the draft was dropped and its fixture branches deleted. The original
      failure came deep into a guided review, reading code locally one file per step; a
      `guided-pr-review` scenario with a renamed file is the place to try next, only if
      the rule ever proves fragile. The run with the skill anchored both findings inside
      the hunk, using the listing from the reference.

## 4. Follow up on a review already submitted

*What happened.* After a review was submitted, the request was to check the author's
replies and the new commits. `pr-review-walkthrough` loaded, matched by phrasings such as
"guarda i commenti della review sulla PR", but it is written for the author answering a
review, and its turn shape did not fit. The work was done by hand.

*Decided.* No new skill: the follow-up writes text that ends up on GitHub, under the same
rules, by the same role, so it belongs in `writing-pr-reviews`.

- [x] Descriptions: add follow-up phrasings to `writing-pr-reviews`, in both languages
      ("l'autore ha risposto alla mia review", "controlla le risposte e i nuovi commit",
      "follow up on my review of PR #N"), and to `pr-review-walkthrough` an exclusion,
      "Not for following up on a review the requester wrote", on the model of
      `guided-pr-review`.
- [x] Non-negotiable 1 extended to the follow-up explicitly. Replies go into a pending
      draft through `addPullRequestReviewThreadReply` with a `pullRequestReviewId`, to be
      verified on the fixture, never through `pulls/N/comments/ID/replies`, which
      publishes at once. Resolving a thread is the only immediate action, done right
      after an explicit yes to that thread's question; `unresolveReviewThread` undoes it.

      Verified on 2026-10-29, on fixture PR 11 (`followup-probe-head` into
      `followup-probe-base`, both deleted afterwards), against a submitted review holding
      one inline thread. A draft made with `addPullRequestReview` took a reply through
      `addPullRequestReviewThreadReply` with its `pullRequestReviewId`: the reply came back
      `PENDING`, sat second in the thread with `replyTo` on the original comment, was
      listed by `pulls/N/reviews/REVIEW_ID/comments` with `in_reply_to_id`, and was absent
      from `pulls/N/comments`, which shows submitted comments only. `resolveReviewThread`
      and `unresolveReviewThread` both worked on the requester's own thread while the
      reply was pending. Two consequences for the items below. A reply cannot travel in
      the REST payload that creates a draft, whose comments carry no `in_reply_to`, so the
      draft is created first and each reply added to it by one GraphQL call. And the leak
      check from item 2 reads `comments(first:1)`, the first comment of each thread only,
      which a reply never is, so it would miss an orphaned reply; it has to read every
      comment of every thread.

      The same probe revisited item 2. Deleting a draft orphaned its reply and its
      comments at once, exactly as on PR 7, but three times out of three, for drafts made
      by GraphQL and by the REST payload alike, the orphans were gone five seconds later
      without any cleanup, and stayed gone at two minutes. The PR 7 probe ran its cleanup
      right after the query, so it never saw that. The run that opened item 2 still found
      19 orphans an hour after its deletes, so GitHub's own cleanup is not reliable, only
      usually quick.
- [x] Scope: every unresolved thread opened by the requester's own review, replied to or
      not, ordered by file and line, is one turn, and N counts them. A thread with a
      change and no reply gets its turn, since it may be resolvable. Threads with neither
      a reply nor a change stay out of N.
- [x] Turn shape, one thread per turn, opening with "Thread n of N": the original comment
      and the reply quoted; what changed on the thread's lines at the new head, with a
      head-SHA permalink; one outcome among accepted, partly accepted, not accepted or
      unchanged, with what was checked; the proposed action, resolve, reply with its text,
      or leave open; one question on that action. The total changing more than once is a
      red flag, as in `guided-pr-review`.
- [x] Final turn: the threads with neither reply nor change, whether the pull request
      description still matches the head, and the commits or files changed since the
      reviewed SHA that touch no thread, with the question whether to review them. A yes
      means a normal review of `reviewed SHA..head`, non-negotiable 7 included.
- [x] Replies collected in the same local file `guided-pr-review` uses, in the same
      payload shape, and turned into one pending draft after the last turn, then the leak
      check and the submit command stated, never run.

      Changed on the way, after the probe on PR 11: the file is the same and so are
      `commit_id`, `body` and `comments`, but replies sit under a `replies` key as a
      thread id and a text, since the REST payload cannot carry them. Delivery is the
      `POST` without that key, then one GraphQL reply per entry.
- [x] Commands in a new `reference/following-up-a-review.md`, linked from the section by
      an instruction with its reason: the reviewed SHA from the submitted review's
      `commit_id`, the threads query rewritten for the reviewer's role rather than copied
      from `pr-review-walkthrough`, `resolveReviewThread`, the pending reply. A file read
      at a pinned SHA is written `git show "${SHA}:path"`, with half a line on why: in
      zsh `$SHA:a` expands as a path modifier.

      Every command in it ran in zsh on 2026-10-29 against fixture PR 12 (a submitted
      review with two threads, a commit fixing one, a reply on it; closed and its branches
      deleted afterwards): the reviewed SHA, the threads query returning both with the
      fixed one `isOutdated`, a draft created from an empty body and no comments, both
      replies landing `PENDING` in their threads with newlines intact, resolve and
      unresolve. It links no other reference file, so the delivery checks are named
      rather than linked.
- [x] Body budget: write the section first, measure the overflow, then move out in this
      order: the "Claim about / Where the answer actually is" table to a reference of its
      own, the `find` command for the docs, and only then prose that repeats the
      delivery reference. The `suggestion` example stays, being the model of the shape.

      Done on 2026-10-29. With the follow-up written the body stood at 540. The table
      and the `find` went to `reference/reading-the-repo.md`, linked from both sections.
      The rest came from repetition inside the body rather than of the reference: the
      section "Where each finding goes", restated by "What the general comment contains"
      and a red flag; a second telling of "The approach is sound"; the bullets on
      delete-and-recreate and on a draft reading back with `position` only, restating
      non-negotiable 2 and the delivery reference; and four closing or overlapping
      paragraphs compressed. The body is now exactly 500, the limit, so the next rule
      added to it costs another move first.
- [x] A new scenario with a `seed-submitted-review.sh`: a submitted review with three
      threads (reply and change, change and no reply, neither), a commit fixing the
      first two, and a changed file no thread touches. After the first turn: no thread
      resolved, no comment published, no non-pending review, and "1" of a total of 2 in
      the text. The final turn graded separately.

      Done on 2026-10-29 as scenario 09, with its baseline and result recorded there.
      The count assertion now accepts "di" as well as "of", since the turn follows the
      prompt's language.

## 5. Ask how a review is built

The 2026-10-29 rerun of scenario 04 created no draft. With three findings it walked
them one at a time, following a user-level instruction to raise one point per message,
while the seven other runs with the skill created the draft at once. The skill set no
shape for that turn, so the same prompt produced two behaviours.

- [x] Decided: once the findings are filtered, the skill asks whether to create the
      draft now or go through the points one at a time, naming how many there are and
      the weightiest. The question is skipped when the request already says which, or
      when a single point survived. One at a time reuses `guided-pr-review`'s collected
      review and last step, so a session left halfway is picked up by either skill.

      Done on 2026-10-29 as the section "Choosing how the review is built" plus one red
      flag, 23 lines. The same 23 came out of passages repeating a rule stated
      elsewhere in the body: the paragraph on a comment justifying itself, the one on
      when a suggestion earns its place, the one on what the opening sentence is not,
      four pairs of red flags, a sentence restating non-negotiable 7, and two shortened
      paragraphs. The body is back at 500.
- [x] Scenarios 01 to 08 run on a single prompt, so the question would stop every run
      before a draft exists. Their prompts need to name the approach.

      Done on 2026-10-29: each ends on ", con tutti i punti in una volta", a phrase that
      names neither the draft nor pending, so scenario 01 still grades a pending state
      nobody asked for. The scenarios `README.md` says so.
- [x] A scenario for the question itself: a prompt naming no approach on a branch with
      several findings, expecting the question with the count and no draft created.

      Done on 2026-10-29 as scenario 10, on scenario 04's branch and original prompt.
- [x] Rerun the scenarios after the edit, as `AGENTS.md` asks.

      Done on 2026-10-29: 10 first, then 01 to 08 on their new prompts, then 09. Every
      one passed, 08 graded by eye as its file allows, and each records the run.

## 6. Grade the turns after the first in writing-pr-reviews

`run.sh` takes a single prompt, so two rules have never been exercised past their first
turn: the follow-up's final turn and its replies delivered into a pending draft (scenario
09), and the one-at-a-time walk once it is chosen (scenario 10). Both stop on a question,
and grading what follows needs an answer to it.

- [x] Give `run.sh` the shape `pr-review-walkthrough`'s has: `run.sh <model> <prompt>
      [<next prompt>...]`, the first turn starting the session and each later one
      continuing it with `--continue` from the same clone, and the fixture's state
      written after every turn, since the state after the last turn says nothing about
      what an earlier one did. A single prompt keeps working as today.

      Done on 2026-10-29. Several prompts write `run.turn<N>.txt` and `.state`, the
      pull request taken from the first prompt's `#<number>`; one prompt still writes
      `run.txt`. `baseline.sh` takes the same arguments. `Edit` and `Bash(jq:*)` joined
      the tool list, and `reset.sh` now deletes the fixture's collected reviews in
      `~/.claude/reviews/`, which a later run would otherwise read as the review so far.
      Checked with a two-turn run on haiku: the second turn remembered the first, and
      both states were written.
- [x] Scenario 09, extended: answer yes to both threads, then to the final turn's question.
      Graded after each turn: the first thread resolved only after its yes, no reply
      published at any point, one pending draft holding the replies after the final
      turn, and the submit command stated, never run.

      Done on 2026-10-29 with four fixed turns, the last one answering no to the review
      of the new commits and giving the reply's text. The first run left the collected
      file behind after delivering it, so both delivery bullets in `SKILL.md` now delete
      it, and the rerun passed with that asserted. The rerun also showed the leak check
      in `reference/delivering-a-pending-review.md` expecting 0 published comments,
      which a follow-up never meets; it now compares with the counts read before the
      delivery.
- [x] Scenario 10, extended: choose one at a time, then add, drop and add with a change.
      Graded after each turn: nothing on GitHub before the last turn, the collected file
      growing by one comment per add, and one pending draft at the end holding exactly
      the comments added.

      Done on 2026-10-29 with five fixed turns: one at a time, drop, add without the last
      sentence, add. The draft is graded at the turn it appears, since the reviewer's
      points vary between two and three on this branch. The run kept three and passed;
      the two-point branch, where the fifth turn must change nothing, is still
      unexercised.

## 7. Add the `tdd` skill

A skill teaching an agent the practice of test-driven development as it is done here,
not as a generic red-green loop. Two sources: the slides of the "TDD & Hexagonal
Architecture" workshop, and a written description of the practice given in the session
that opened this item. The decisions are taken one at a time in an interview and
recorded below as they are made, so the skill can be written from this entry alone.

Decided:

- [x] The skill replaces `mattpocock-skills:tdd` rather than coexisting with it. The two
      disagree on the core of the practice: that one keeps refactoring out of the loop
      and treats tests written ahead as horizontal slicing, while this practice
      refactors in every cycle and starts from a plan of empty tests. Two skills firing
      on the same phrasings would leave the agent to reconcile them. Outside this repo,
      the line in `~/.claude/CLAUDE.md` that mandates `mattpocock-skills:tdd` moves to
      the new skill once it exists.
- [x] The skill says nothing about hexagonal architecture. The surface under test is
      the public API in generic terms, what calling code can see and touch, so the
      rule holds in a codebase with no ports. Ports and adapters, and the integration
      and acceptance tests that go with them, are left to a separate skill, not queued
      yet.
- [x] The skill is self-contained: it restates every rule on tests it relies on
      rather than pointing to a user-level file such as `~/.claude/code-principles.md`,
      which Codex and Copilot never read. The rules stay in the body; the catalogue of
      test doubles may move to a `reference/` file linked from it. The cost is the same
      rules living in two places, so a change to one has to be carried to the other.
- [x] A test's name, which is also its entry in the plan, states the behaviour in
      domain words, as short as still tells it apart from the other entries. No class,
      method or other technical detail, and no concrete value, which belongs in
      Arrange. The condition goes in only when it is what distinguishes the scenario,
      as in `rejects a booking outside opening hours` against `accepts a booking within
      opening hours`. The casing and phrasing follow the project and its test framework.
      Names are reread at every refactoring step like the rest of the code. The slides'
      bad example, `Should_cart_aggregate_apply_20_percent_discount_based_on_promo_when_updating_with_its_code`,
      fails on the technical name, the value and the mechanism, not on length alone.
- [x] Test doubles: the slides' definitions, without their "order of complexity",
      since a fake is often more complex than a stub and the order would imply a rule
      that does not hold. The usage and naming rules come from `code-principles.md`:
      replace only boundaries the project owns and keep real domain and value objects;
      a dummy proves nothing, so a call that must not happen is checked with a spy that
      received none; a spy is the usual choice for outgoing commands, a mock only when
      the exact protocol of calls is the behaviour under test; a fake when a working
      lightweight implementation reads better than a pile of stubbed answers; a double
      is named after
      its kind, with the `Double` suffix when it plays more than one role. The usage
      rules stay in the body; the definitions and naming conventions go to
      `reference/test-doubles.md`.
- [x] Where the agent stops and asks. Always: after writing the plan of empty tests,
      before the first red; whenever it wants to change the plan (add, delete, reorder
      or rewrite a test); when green does not come from a small, direct change, saying
      what blocks it and proposing how to change the test or the plan; and when a
      refactoring would touch structure (a new type, an abstraction, moving a
      responsibility) rather than names or local duplication, proposing it, or
      proposing to wait for the next tests, instead of doing it. At the start the
      requester also chooses whether the agent pauses after every red-green-refactor
      cycle; the question is skipped when the request already says. A pause on every
      cycle stays opt-in because on a plan of ten or fifteen tests it turns into a
      confirmation given without reading.
- [x] The plan lives in the test file as pending tests: each entry is a test with its
      final name and an empty body, marked with the framework's own mechanism
      (`it.todo`, `@Disabled`, `#[ignore]`, `t.Skip`, `pytest.mark.skip`). Adding or
      deleting an entry happens where the tests are, the approved name is already the
      test's name, and the state of the plan is read from the runner, so a new session
      picks it up without memory. A pending test carries only a title, which is why
      the plan is not horizontal slicing. No fallback for a framework without pending
      tests until one turns up.
- [x] The skill is named `tdd`: `skills/tdd/`, invoked as `/myai:tdd`, and seen as
      `tdd` by Codex and Copilot. The namespace tells it apart from
      `mattpocock-skills:tdd` in Claude Code, and `~/.agents/skills/` holds no other
      `tdd`, so the link there collides with nothing.
- [x] The skill is independent of language and framework: it speaks of pending tests,
      the runner and assertions, and leaves the translation to the project's stack.
      One or two short pseudocode examples are allowed where prose alone does not
      carry a rule, such as a bad and a good test side by side showing how a value in
      Assert traces back to a name in Arrange. They sit in the body next to the rule
      they illustrate, since they are part of it.
- [x] How the plan is built. First a few lines of upfront design, presented with the
      plan for the same approval: the entry point the tests call in Act, the external
      dependencies that become test doubles, and the observable reactions (returned
      value, state read through the public API, calls to dependencies). Then the
      happy paths, simplest first: fewer moving parts means fewer inputs that vary,
      fewer dependencies involved, fewer business rules applied together, and on a tie
      the one whose green needs less production code. Degenerate or empty cases, such
      as an empty cart totalling zero, close the happy paths rather than open them,
      even though they have the fewest moving parts: with nothing moving they drive no
      design, and they are often green before any production code exists. After the
      happy paths come domain errors, then input validation, then edge cases.
- [x] The red step. The agent takes the first pending test of the plan and only that
      one. It writes the body by wishful thinking, in final form rather than as a
      draft: Act calls the API as it would like to use it, Arrange holds only the data
      that matters to the behaviour, Assert compares the whole expected value. It makes
      the test compile with the least production code, signatures and types, whose
      body returns a trivial wrong value so the test fails on the assertion; never a
      thrown "not implemented", which proves nothing about the assertion. In a dynamic
      language the same holds: the first failure is the assertion, not a missing
      function. It runs the tests and reads the output: the new test fails on its
      assertion, for the expected reason, with a message that says what is missing,
      and every other test stays green. An unclear message is fixed before moving on.
      A new test that passes at once was revised after the first draft, see below.
- [x] The green step. The least production code that makes this test pass, sins
      allowed: hardcoded values, duplication, specific branches, provisional names.
      Nothing for behaviour that belongs to a later pending test, even when the agent
      can see it coming: code turns generic only when a later test forces it, because
      a hardcoded answer no longer satisfies two tests at once, or in refactoring, when
      the duplication between test data and code is removed. The obvious
      implementation is allowed when it is as short as the hardcoded one and covers no
      behaviour named by a pending test, and the agent says so to the requester each
      time it takes it, naming the pending tests it checked it against, since a pending test has an empty body and cannot be run to
      prove the implementation stayed narrow. All tests run: the new one green, every
      other one still green. An old test turning red is fixed in production code; a
      change to a test already green is a change to the plan. Green that does not come
      from a small, direct change triggers the stop already decided.
- [x] The refactor step, after every green. In production code: duplication,
      including between test data and code, which is where a hardcoded value turns
      generic; names; the rules of simple design. In tests: names, Arrange-Act-Assert,
      the link from Assert back to Arrange, no logic. Refactoring changes no behaviour:
      nothing new in production code, and a test changes shape, never what it checks.
      Small steps, with every test run after each one rather than at the end. A step
      that turns anything red is undone back to the last green, never fixed forward,
      and retried smaller or by a longer path that stays green (add the new beside the
      old, move one caller at a time, remove the old). Fixing forward would work
      without the safety net exactly when the step has shown it was needed, and
      "small and obvious" is a judgement an agent stretches; back to green is
      checkable.
- [x] The review of the plan that closes each cycle. Missing tests: a behaviour the
      cycle brought out that no entry covers, proposed with its name and its place in
      the plan's order. Redundant tests: one is redundant when another checks the same
      behaviour and would fail for the same reason, so deleting it loses nothing; the
      usual case is an early simple test that later, more specific ones include. The
      deletion is proposed naming the test that already covers it. With nothing to
      change the agent moves to the next pending test without stopping, unless the
      requester chose a pause on every cycle. A refactoring deferred by agreement is
      not recorded anywhere: the smell stays visible in the code, every later refactor
      step reconsiders it, and the last refactor step of the plan proposes it, a stop
      since a deferred refactoring is always structural; a local one is never
      deferred. A `TODO` or a list would be a second
      copy that outlives the code it describes.
- [x] Bug fixes. The plan opens with a test that reproduces the bug through the public
      surface, named after the correct behaviour (`applies the discount to discounted
      items too`), never after the bug or an issue number, and placed in the test file
      of the behaviour it concerns. It goes through the plan approval like any other
      plan, even with one entry. Red must show the bug: the test fails on its
      assertion, the actual value being the wrong behaviour reported. A test that
      passes means the bug is not reproduced: no production code is touched, and the
      agent stops, says what it tried, and asks for details or proposes another
      scenario. Needing internals to reproduce it, a private method or the database
      instead of the API, is a signal about the surface or the design and a reason to
      stop, never a licence to test the inside. Then the cycle is the usual one, and
      the plan review also asks whether the same defect can show up in nearby cases.
- [x] The anatomy of a test. Three visible blocks in order, Arrange, Act, Assert, with
      a single Act; no branch, loop or computation in a test. Arrange keeps the data
      that matters to the behaviour, and irrelevant setup goes to builders or object
      mothers named for what they prepare. By default one whole expected value,
      compared by equality; several assertions only when the behaviour has distinct
      observable reactions that do not fit one value, such as a returned value and a
      call recorded by a spy, each about the same behaviour. Literals are welcome when
      their link to Arrange shows inside the test: the expected value is built from
      Arrange's names, or is a literal whose link is plain at a glance, a price of 100
      with a 20% discount expecting 80. Never an expected value computed with the
      production formula, which passes by construction, and never a literal repeating
      data hidden inside a helper. This is where the pseudocode bad and good example
      goes. Failure messages are specific. No comments by default, unless the
      project's rules ask for them, so the blocks are told apart by blank lines rather
      than `// Arrange` markers; a comment that survives either clarifies a detail the
      test's code cannot express, or is a very short description of the scenario.
- [x] No commit on the agent's own initiative. Undoing a refactor step means reversing
      the edits of that step, which the agent has just made and which the rules keep
      small; the undo is checked by every test being green again, and anything else
      stops the cycle. When the requester or the project asks for commits, there is one
      per completed cycle, after refactoring with every test green, in the project's
      message format.
- [x] When the skill applies. It activates on any change to production code that adds
      or alters behaviour, a feature, a behaviour change or a bug fix, whether or not
      the request names TDD; not on refactoring that keeps behaviour, documentation,
      configuration, or tests added to code already written. Its first turn asks
      whether to proceed with TDD at all, and a no sends the agent back to working
      without the skill. Only after a yes, in a turn of its own, comes the question on
      a pause after every cycle decided above. Two questions rather than one with
      three answers: the first decides whether the skill runs, the second how. Either
      is skipped when the request or the project's rules already answer it. The
      description:

      ```yaml
      description: >-
        Use when about to write or change production code that adds or alters behaviour,
        such as a feature, a behaviour change or a bug fix, and when asked for TDD,
        including phrasings like "facciamo TDD", "sviluppiamolo in TDD", "partiamo dai
        test", "scrivi prima il test", "fixa il bug partendo da un test che fallisce",
        "implementa questa funzionalità", "red-green-refactor", "let's TDD this",
        "test-first", "drive this with tests", or "reproduce the bug with a failing test
        before fixing it". Not for refactoring that keeps behaviour, documentation,
        configuration, or adding tests to code that already exists.
      ```
- [x] The fixture for `evals/tdd/`: a small local Rust crate, no GitHub, created by
      `fixture/setup.sh` in a temporary directory and rebuilt from scratch before
      every run. The domain is the slides' cart with a promo code discount. Two states:
      an empty module with only its test file, for scenarios on a new feature, and an
      implemented module carrying one known bug, for scenarios on bug fixes. Rust
      because every clone already has the toolchain `tools/skill-check` needs, pinned
      by `.tool-versions`, and because `cargo test` counts passed, failed and ignored
      tests, so the state of the plan is graded by command rather than judgement.
      The crate ignores `target/`, since the skill runs the suite before any plan and
      scenario 1's clean `git status` must read only what the agent changed.

- [x] The scenarios under `evals/tdd/`, each also run without the skill so the value
      of its rule is seen rather than assumed. M marks an assertion graded by command,
      E one graded by judgement.

      1. The two opening questions. "implementa lo sconto con codice promozionale sul
         carrello", naming no TDD: turn 1 asks whether to proceed with TDD, turn 2,
         "sì", asks about the pause on every cycle in a turn of its own. M: `git status`
         clean after both.
      2. The plan and its stop. A prompt answering both questions. M: `cargo test`
         reports 0 passed, 0 failed, N ignored, and nothing outside the test file
         changed. E: behaviour names with no technical detail, the empty cart closing
         the happy paths, errors and validation after them.
      3. One cycle at a time, nothing anticipated. As 2, with the pause on every cycle,
         and a second turn "ok, procedi". M: 1 passed, 0 failed, N-1 ignored. E: an
         obvious implementation, if taken, is reported.
      4. A bug that does not reproduce. Implemented module, a reported bug the code
         does not have. M: the production file unchanged. E: the agent says the test
         passed and stops.
      5. A bug that reproduces. Implemented module, the known bug. M: every test green
         at the end, and the new test's name holds none of `bug`, `issue`, `fix`,
         `regression`. E: the red shown carries the wrong value reported.
      6. The skill does not fire. "rinomina `total` in `total_price`". M: the reply
         never mentions TDD, the rename is done, every test green.
      7. The plan review closing a cycle. A third fixture state, halfway through a
         plan: `totals the price of a single item` green, `totals the prices of all
         items` pending next, production returning the first item's price, the rest
         pending. Turn 1, "continuiamo in TDD il carrello, il piano è nel file di
         test", expects the pause question; turn 2, "no, fermati solo dove serve",
         expects the resumed plan shown with each entry's state and a confirmation
         asked, M: still 1 passed and no file changed; turn 3, "ok". After turn 3: M,
         the first test still exists; M, 2 passed, 0 failed, the rest still ignored,
         which is what proves the stop, since skipping the review would have moved on
         to the third test; E, the proposal names `totals the prices of all items` as
         already covering the first, and the turn closes on a question.

      Left out until a real run shows them broken: a refactor step undone after a red,
      a structural refactoring proposed rather than done, and a missing test found by
      the plan review, whose content depends too much on the code the agent writes.

- [x] A plan found already written, from an earlier session or someone else, is shown
      with the state of each entry, done or pending, along with any entry the code has
      since made stale or missing, and the agent waits for a go before the first cycle.
      The file cannot tell whether the plan was ever approved, since a session may have
      ended on the plan's own stop, and the code may have moved since. One stop per
      session.

- [x] Outside this repo, once the skill is written and its scenarios pass:
      `"mattpocock-skills:tdd": "off"` joins `skillOverrides` in
      `~/.claude/settings.json`, beside `superpowers:test-driven-development`.
      `mattpocock-skills:implement` and `implement-spec` both say to call the Skill
      tool with "tdd", which would load the replaced skill and its opposite rules on
      refactoring and the plan; an instruction in `CLAUDE.md` is weighed against that,
      an override removes the skill from the list.

- [x] Outside this repo, at the same time: the "Test-driven development" section of
      `~/.claude/CLAUDE.md` becomes

      ```markdown
      - For TDD, **always use `myai:tdd`**, even when another skill points to a different one: for
        example `mattpocock-skills:implement` and `mattpocock-skills:implement-spec` call "tdd", and
        `superpowers:executing-plans`, `superpowers:systematic-debugging` or `superpowers:writing-skills`
        point to `superpowers:test-driven-development`.
      - `mattpocock-skills:tdd` and `superpowers:test-driven-development` are deliberately disabled in
        `~/.claude/settings.json` (`skillOverrides`): do not look for them and do not re-enable them.
      ```

      It names which skill wins and nothing about when it fires, which the description
      decides.

- [x] Outside this repo, at the same time: the link `~/.agents/skills/tdd`, as the
      `README.md` setup describes, for Codex and Copilot. No other `tdd` lives there,
      so they need no override. Inside the repo the `README.md` skills table gains a
      `tdd` row, which `tools/check.sh` enforces.

- [x] A test that does not write simply from the public surface is not forced: no long
      complicated test, no reaching for a piece of the inside. The agent stops, names
      what makes it hard (an oversized Arrange, too many doubles, a reaction that
      cannot be observed, an Act in several steps), and proposes one of three ways: a
      different surface, the API the test wishes it had; splitting the plan's entry
      into smaller behaviours; or a preparatory refactoring that makes the test easy to
      write. That refactoring happens on green: the test being written goes back to
      pending first, the refactor step's rules all hold (small steps, every test run
      after each, a red undone), and the red resumes afterwards.

- [x] The skill names nothing from one person's configuration, `code-principles.md`
      included, and depends on no particular agent: others may use it without that
      file, and without Claude Code at all. So no user-level path, no tool name of one
      harness, no other skill by name; where the body takes a position another TDD
      skill contradicts, such as refactoring inside the loop or a plan of empty tests
      not being horizontal slicing, it states the position and its reason without
      naming the other skill. The one guard against drift lives outside this repo, at
      the head of the Tests section of `~/.claude/code-principles.md`, added with the
      other changes outside the repo:

      > The `myai:tdd` skill restates these rules for agents that never read this file;
      > a change here is carried to `<repo-path>/skills/tdd/SKILL.md`.

      Nothing in this repo points back, so a change made in the skill alone can leave
      that file behind; the cost is accepted.

- [x] A collaborator extracted in refactoring, a new type or module inside the black
      box, gets no tests of its own: the surface's tests that brought it into being
      already cover it, and tests of its own would repeat those behaviours one level
      down and tie the tests to the structure refactoring has to stay free to change.
      It becomes a black box with a plan of its own only when it has a public surface
      of its own, used by callers other than the one it came from, or logic rich
      enough that checking it from the outer surface is awkward, and even then the
      agent proposes it as a change to the plan rather than deciding.
- [x] A suite already red. Before the plan the agent runs the whole suite and records
      the result, since every check of the cycle compares with the last green. If it
      is not all green, through broken tests, a build failure or a flaky test, the
      agent stops before writing anything and reports which tests fail, how, and
      whether they touch the area of the work. The requester chooses: fix them first
      as separate work, a bug following this skill in turn; or go on with the known
      failures, each one named, every check of the cycle then reading "no failure
      beyond the known ones", and any known failure that changes, passing or failing
      differently, being a stop. Broken tests are never disabled, deleted or ignored
      to turn the suite green unless the requester asks for it.
- [x] Changing the behaviour of code that has no tests: the safety net comes first.
      The agent proposes a plan of characterization tests, pinning the current
      behaviour of the surface the change will touch, as it is even when it looks odd,
      limited to that area and named for behaviour; it is a stop like any plan. They
      are written one at a time, each followed by an explanation of what behaviour it
      pins down and a stop until the requester's go, whatever was chosen for the pause
      on every cycle: the expected value comes from the code rather than from anyone's
      intent, so each explanation is a discovery that can change the next test or the
      plan for the change, and the net covers only the touched area, so the stops are
      few. They are the one exception to red first: written after the code, they
      pass at once, their expected value read by running the real code rather than
      imagined. One that fails means the agent misread the code, not a bug to fix, and
      a behaviour that looks genuinely wrong is reported, never corrected while the net
      is being built. A surface that cannot be tested needs a few mechanical
      refactorings without a net, such as passing a dependency as a parameter or
      wrapping it behind an interface: each is proposed with why it is safe and done
      only after a yes. Then comes the plan for the change itself, built and run like
      any other, with the characterization tests as part of the net.

- [x] Parameterized tests come into play when combinatorial complexity emerges among
      the scenarios: the behaviour depends on combinations of inputs, and the plan's
      entries start to differ only by which combination they pick. Two tests that
      merely look alike are not reason enough. A table is never written up front: each
      case is an entry of the plan, written through the usual cycle, and a refactoring
      of the tests merges green ones into a table only when they check the same
      behaviour with the same reason to fail. Rows needing a branch in the body, an
      expected value of another kind or a different error message are different
      behaviours and stay separate tests. Each row reads on its own, input and
      expected value side by side with a label naming the case, the expected value
      written and never computed in the body. A new row is a new entry of the plan:
      it fails first, and one that passes at once is verified by a targeted break.

- [x] Determinism. Whatever is not deterministic, current time, random values,
      generated IDs, the environment, enters the black box through a boundary the test
      controls, stubbed and fixed under a name in Arrange so the expected value traces
      back to it: a clock fixed at 10:00, a code expiring at 10:30. In a new feature
      those boundaries are part of the surface designed with the plan; in existing
      code opening one is a preparatory refactoring. Every test arranges its own
      context, shares no mutable state, and passes alone and in any order, which is
      also what lets the suite run in parallel and shortens its lead time: tests are
      written so they can, holding no global state, fixed file, port or shared record.
      Switching the runner to parallel is the project's configuration, so the agent
      proposes it rather than doing it. A flaky test, passing or failing with the code
      unchanged, is a stop: never rerun until green, never ignored, reported with the
      search for its source.

- [x] When the plan is done, the agent runs the whole suite one last time and reports
      five entries, every one always present, "none" when empty, so an empty entry
      reads as checked rather than forgotten: the suite's result, passed, failed (zero,
      or only the known failures accepted at the start) and ignored; the behaviours
      implemented, the final plan's test names; how the plan changed from the approved
      one, each addition, deletion or rewrite with its reason in a line; the obvious
      implementations taken, listed together though each was reported at the time;
      the deferred refactorings left open, with where in the code. The last can only
      hold what the requester declined at the final refactor step, and stays because
      it is the one entry saying the code delivered is not yet in the intended shape.
      No entry for behaviour noticed but left out: the plan review and the
      characterization stops put every such finding to the requester when it shows up,
      so an entry there could only cover a stop that was skipped, and would give the
      agent a place to park findings instead of raising them.

- [x] A spike answers technical uncertainty only, how something outside the black box
      behaves: an external API, an unfamiliar library, whether an approach holds up.
      Uncertainty about business behaviour is a question to the requester, never a
      spike. The agent proposes it, a stop, saying what it does not know, what it wants
      to find out and how small the spike will be. Its code lives outside the project's
      code, in a temporary directory and never in tracked files, and is thrown away: no
      line of it is copied into production code. What it taught is reported in a few
      lines with the plan or its change, so the findings shape the surface and the
      tests and go through the requester's stop.

- [x] After the first draft, the naming conventions of test doubles moved from
      `reference/test-doubles.md` into the body: they are rules, and `AGENTS.md` keeps
      rules in the body. The reference file keeps the definitions of each kind.

- [x] A new test that passes at once has never been seen failing, so the agent proves
      it can: the smallest break of production code that makes exactly the named
      behaviour wrong, never a broad one that fails it for another reason; the test
      run, failing on its assertion with a clear message; the break undone and every
      test green again; the requester told, without a stop. A test another one already
      covers is then proposed for deletion in the plan review like any redundancy. A
      targeted break that does not make it fail means the test does not check what its
      name says, and that is the stop, with a rewrite proposed. A bug's test is the
      exception: passing means the bug is not reproduced, since the reported behaviour
      is itself the break. Degenerate cases, often green before any code, are the usual
      case this covers.
- [x] Characterization tests get the same targeted break, since they too pass without
      ever being seen failing, and a net green by mistake has its hole exactly where the
      work will happen. The outcome goes into the explanation that already follows each
      one, before its stop. On legacy code the break also checks where the behaviour
      lives: a break that does not make the test fail means it lives elsewhere, or the
      test checks something else.

- [x] Uncle Bob's three rules of TDD sit word for word among the non-negotiables. They
      govern code that adds behaviour; refactoring on green adds none and follows its
      own rules, preparatory refactoring and seams in untested code included, as the
      slides themselves remark ("what about refactoring???"). "Sufficient to fail" is
      read at the level of one test, one behaviour and no assertion beyond what it
      needs, written whole so wishful thinking can shape it, rather than as
      micro-steps stopping at the first line that does not compile: the compilation
      failure is the first red, run and seen, and only the code making it compile
      follows.

- [x] Patterns from Kent Beck's *Test-Driven Development: By Example* are named where
      they already live, since a name the model knows recruits a whole behaviour in one
      word: Test List, Fake It, Obvious Implementation, Triangulate, Evident Data,
      Isolated Test, Child Test, Another Test, Regression Test. Four are added: Assert
      First in the red step, One Step Test in the plan's order, One to Many for a
      behaviour over a collection, and Do Over among the ways out of a hard green.
      Left out: Starter Test, which is the trivial first test the plan's order puts
      last; Broken Test, since the state is read from the pending tests; Learning Test,
      tests outside the black box whose job the spike does; Break and Cheap Desk, Nice
      Chair, which concern a person rather than an agent.

- [x] The Transformation Priority Premise. At every green the agent takes the highest
      transformation that is enough, the list living in
      `reference/transformations.md` as a table to consult. A test that cannot go green
      through a high transformation is a hard green, whose stop gains one more way out:
      bringing forward a test that needs a simpler one, a change to the plan. The plan
      review checks the direction, code turning more generic as tests turn more
      specific: a branch or constant matching one test's data beside another of the
      same shape is code mirroring the tests. The first Fake It is specific by choice;
      past it, the first mirror is a stop saying the road is likely wrong, with a
      reorder, a Do Over or a different surface proposed.

- [x] Dummies, stubs and fakes support a test and are never what it verifies. Assert
      reads the returned value, state read back through the box's public surface, or
      the calls a spy recorded or a mock verifies, never the inside of a fake: a
      booking saved into an in-memory repository is checked by finding it through the
      box. A surface with no way to read it back is a test that does not write simply,
      with its stop.

- [x] Every double is named after the boundary it replaces with the `Double` suffix,
      whatever its kind: `ClockDouble`, `NotifierDouble`, `BookingRepositoryDouble`.
      Each constructor names its role opening with the kind, `stubFixedAt(10:00)`,
      `spy()`, `fakeInMemory()`, `dummy()`, so the kind shows on the line that builds
      the double and one double serves several roles without a rename; the variable
      takes the concept's name, `clock`, `notifier`. This replaces naming by kind,
      `stubClock` or `InMemoryBookingRepository`, which `~/.claude/code-principles.md`
      used to prescribe; that file was aligned at the same time, along with the rule
      that dummies, stubs and fakes are never what a test verifies.

- [x] The description drops its Italian phrasings for now, against the rule in
      `AGENTS.md` that asks for both languages, as a measured exception rather than a
      change of that rule: the agent reads the description and understands Italian,
      and the main condition, production code that adds or alters behaviour, covers an
      Italian request without a phrase of its own. Scenario 1, whose prompt is in
      Italian and names no TDD, is run with and without them; the phrasings come back
      if the skill fails to fire without them. The current phrasings: "red-green-
      refactor", "let's TDD this", "test-first", "write the test first", "drive this
      with tests", "reproduce the bug with a failing test before fixing it". The ones
      removed: "facciamo TDD", "sviluppiamolo in TDD", "partiamo dai test", "scrivi
      prima il test", "fixa il bug partendo da un test che fallisce", "implementa
      questa funzionalità".
- [x] Scenario 1 settled it: on "implementa lo sconto con codice promozionale sul
      carrello descritto in FEATURE.md", naming no TDD, the skill loaded with English
      phrasings only. The phrasings stay out, and `AGENTS.md` now allows English-only
      phrasings for a skill whose main condition covers any language, once a scenario
      shows it.
- [x] To stay under the 500-line limit after this round, passages were shortened
      without dropping a rule: the opening, the reading of the three rules, the spike,
      the verification of a test that passes at once, and the bug that does not
      reproduce. The body stands at 499 lines, so anything added from here needs an
      equal cut or a move to `reference/`.

- [ ] `skillOverrides` does not disable a plugin skill: the Claude Code docs say "Plugin
      skills are not affected by `skillOverrides`. Manage those through `/plugin`
      instead." The baselines of `evals/tdd/` loaded `superpowers:test-driven-development`
      and `mattpocock-skills:tdd` with both set to `"off"`, so the line in
      `~/.claude/CLAUDE.md` calling them disabled is false. A per-skill deny rule,
      `Skill(name)` in permissions, is documented for skills in general and untested on
      a namespaced plugin skill.

What is left after these is writing it: `skills/tdd/SKILL.md`,
`skills/tdd/reference/test-doubles.md`, the `README.md` row, `evals/tdd/` with its
fixture and the seven scenarios run with and without the skill, then the changes
outside the repo.

## Completed

**Close the dispatch gap, and give the suite the assertion that would have caught it.**
Five of seven runs had dispatched a reviewer subagent. After the amendment, seven of
seven.

*What the gap was.* Non-negotiable 7 said to run `superpowers:requesting-code-review`
first, and two runs did exactly that and no more: they loaded the skill, read
`code-reviewer.md`, and then reviewed the diff themselves. Read literally the rule was
satisfied, which is what made it worth rewriting rather than repeating.

*The amendment, three lines.* Non-negotiable 7 now names the subagent rather than the
skill, "Loading that skill is not the requirement, dispatching is". The opening of
`Where the findings come from` says the same thing at the point of instruction. And a red
flag names the shape the two runs took, the reviewer skill loaded and no subagent
dispatched. The body went from 494 lines to 497, inside the six the previous entry had
left.

*E10, and it is the part worth keeping.* The suite had no way to see any of this: both
failing runs produced clean reviews that passed E1 through E9 and every per-scenario
check. E10 grades the dispatch from the run's own transcript, and the pattern was
validated against the seven runs that exposed the gap, returning `1` for the five that
dispatched and `0` for the two that did not, before it was written down as an assertion.

*The result.* Seven of seven dispatched, scenarios 03 and 05 included, which are the two
the amendment was written for. Everything else held: E1 through E9 clean across the suite,
every line anchor verified at the head SHA, scenario 04's block back to `start_line: 5,
line: 7` spanning all three replaced lines, scenario 07 appending under the seeded id with
both seeded comments byte for byte.

*One real miss, recorded rather than smoothed over.* Scenario 04 wrote "Nothing in this
`Dockerfile` sets `USER`" with the filename unlinked in prose, which no exception covers.
It is the first genuine E6 hit in either rerun, and it is left as a measurement rather
than answered with a rule, since one occurrence is not a pattern.

*A grading gap closed on the way past.* Three runs have now written the review payload
outside the clone, where the recipe for grading an anchor looks. The eval `README.md` now
says the transcript carries the `Write` call with its full contents, so an anchor is
recoverable rather than lost.

**Delegate the finding to `superpowers:requesting-code-review`, and make every comment
pragmatic.** Three directives, requested together, and the rerun that measured them.

*What the body says now.* Non-negotiable 7 runs `superpowers:requesting-code-review`
before anything is written, with an exception for an ask that already names the finding.
A `Where the findings come from` section makes what it returns raw material: its output
format is not the review's, so Strengths, Recommendations, Assessment and the severity
headings become nothing, a strength on a line in the diff may survive as a short inline
note per Tone, findings are filtered before being rewritten, its ranking decides nothing,
and its architectural claims are verified here rather than trusted. A `Pragmatic and
concrete` section bans advice that would read the same on another pull request, a
principle named instead of a change proposed, and anything hypothetical. Inside `One
point per comment`, a comment no longer has to justify itself: the bar is that the
suggestion is understandable, and the reasoning still gets done but stays out of the
text. Three red flags, and the `README.md` skills row now says where the findings come
from.

*The harness had to be fixed before any of it could be measured.* `run.sh` allowed
`Read Write Grep Glob` and three `Bash` patterns, so a headless run had no way to
dispatch a subagent and no `Skill` tool to load the skill it is told to run. Every
scenario would have quietly reviewed the diff itself and passed. The tool name was
confirmed rather than guessed, by dispatching one under `--allowedTools Agent` and
getting a reply, since `Task` is the plausible alternative and a wrong name in a
committed file fails silently.

*The rerun: all seven scenarios on Opus, and every mechanical assertion held.* E1 through
E9 clean on all seven, which includes the four the previous directive left unrun and
closes that debt. Two E6 hits, both documented exceptions: replacement code inside a
`suggestion` block on 04, and a filename inside proposed wording for a document on 07.
The sharpest results are 04, where the anchor came back `start_line: 5, line: 6` covering
exactly the two lines replaced with the deletion on line 7 split into a comment carrying
no block, and 07, which appended under the same review id with both seeded comments byte
for byte and named the offending one in the reply.

*The evidence that the delegation works in substance, not only in the tool log.* On
scenario 04 the subagent returned four Strengths, `Critical: None`, and three Important
findings. The two Dockerfile findings shipped as the two Dockerfile comments with the
reasoning intact and the prose cut from roughly two hundred words each to 52 and 49, and
the Strengths, the severity headings, the assessment and the subagent's own em dashes and
second person are absent. Scenario 01 shows the pragmatism rule deciding a real case: a
genuine `deploy.yml` defect was kept out of the review because it holds on any pull
request in this repo rather than on this one.

*What it did not settle, which is now task 1.* Five of seven dispatched. Scenarios 03 and
05 loaded the skill, read `code-reviewer.md`, and reviewed the diff themselves, producing
clean reviews that pass every assertion in the suite.

*One deviation from the documented procedure, stated because this repo has been bitten by
a fixture drifting under its assertions.* The eval README asks for `reset.sh` before every
run. This rerun did one reset up front, seeded scenario 07 after it, and ran all seven in
parallel with `EVAL_SKIP_RESET=1`. Each scenario owns its own pull request and a pending
review is per pull request, so it is equivalent to the sequential form, and it cost
fifteen minutes rather than two hours. What it gives up is the guarantee that a retry
after a failure starts from a clean pull request, which nothing needed here: all seven
succeeded on the first attempt.

**Make every comment as short and plainly worded as it can be, and grade it.** Requested
as a directive, and it turned out to answer something measurable rather than a preference.

*The evidence, gathered before the rule was written.* The seven post-split runs produced
inline comments between 18 and 179 words. The two comments a person wrote by hand for
scenario 07's fixture are 18 and 22. A model left alone writes five to eight times what a
person writes on the same kind of finding. Non-negotiable 6 already said "be short and
direct, no filler", and a 179-word comment passed it, so what was missing was a shape
rather than an instruction.

*What the body now says.* A `One point per comment` section, pointed at from non-negotiable
6: a comment's prose stays under 80 words and most land well under 50, code in a
`suggestion` block excluded since that is the fix rather than the explaining. Three things
a comment needs, what is wrong, why where it is not obvious, and what to do instead, with
everything past them cut. One point per comment, short sentences, ordinary words, and the
note that a point genuinely needing paragraphs is a design discussion belonging in the
general comment or an issue rather than anchored to a line. Three red flags guard it.

*Graded as an every-run assertion rather than a scenario, which is the design call worth
recording.* Brevity is a property of every review, exactly like the em dash rule, so a
scenario of its own would have tested it in one place while leaving it unmeasured in the
other seven. E9 in the eval README counts prose words per inline comment with fenced blocks
stripped. The 80 is set above every human-written comment in the fixture and below most of
what the runs produced, so it bites precisely where the measured gap is.

*It works, measured on three scenarios on Opus.* Every comment came back under the cap, and
the change is large rather than marginal:

| scenario | before | after |
|---|---|---|
| 03 | 160 | 54 |
| 04 | 118, 23 | 49, 66, 23 |
| 06 | 139 | 35, 40 |

Scenario 04 is the interesting one: the long comment did not merely shrink, the review went
from two comments to three, which is `One point per comment` doing what it says rather than
prose being squeezed. The `suggestion` block survived in both 04 and 06, so excluding code
from the count kept the rule from fighting the one the skill cares more about. Nothing else
moved: no em dash, no first or second person, no off-head permalink, `PENDING` with nothing
published, on all three.

*An ambiguity the first result exposed, fixed before it could bite.* Scenario 03 came back
with a 177-word general comment, and the skill calls that a comment too, so read literally
the 80 would have applied and forced a finding out. The body now says the general comment
is measured per finding rather than as a whole, since it legitimately carries several.

*What is not done, stated rather than implied.* `AGENTS.md` asks for the whole suite before
a `SKILL.md` change is offered, and this ran three of eight scenarios, chosen as the ones
whose comments were longest plus the one carrying a `suggestion` block. Scenarios 01, 02,
05 and 07 have not been rerun against this directive. Nothing observed suggests a problem,
and that is a prediction rather than a result.

**Check that a skill's relative links resolve, which is the gap the split opened.** Moving
the delivery commands into `reference/` made the skill depend on a file nothing verified.
Confirmed rather than assumed before writing anything: with the reference file deleted,
`tools/check.sh` passed. A rename would have broken delivery silently, and the first sign
of it would have been a real review not getting posted.

The check now parses every relative markdown link in a body and reports one that resolves
to nothing. External URLs are left alone, since they name nothing in the tree, and a
fragment is stripped before the file is looked for. Five named tests, plus the existing
one that runs the checker over this repo's own skills, and the gap is closed by
demonstration: deleting the reference file now fails the suite where it used to pass.

*It found a defect on its first run, and the defect was its own.* Against the real skill it
reported a missing file called `...`, from the red flag that names the shape a link should
take, `[...](...)` written inside a code span. A body about writing markdown quotes
markdown, and a link parser that does not strip code reads the quotation as a pointer. The
fix strips fenced blocks and inline code spans first, and both cases are now tests. Worth
recording because it is the sixth assertion in this repo to test the failure imagined while
writing it rather than the one the rule is about, and the first to be caught before it
shipped rather than after.

*A second defect in the same parser, found by checking rather than by reasoning.* The
fence tracker toggled on any run of three or more backticks, and the skill body shows a
suggestion block as four backticks wrapping three, so the inner fence closed the outer one
and the lines between were read as prose. It produced no false positive today only because
those lines carry no link. The tracker now records the opening run's length and closes on
a run at least as long, with the nesting as a test.

*Two limits of the check, stated rather than left to be discovered.* Its scope is a
`SKILL.md` and not the files one points at, so a reference file linking a second reference
file is unchecked; that costs nothing while none of them link anything, and would need the
check made recursive the day one does. And it verifies that a path resolves, never that
the thing it resolves to says what the pointer claims.

**Split the delivery mechanics out of the skill body, which is the tripwire firing and
then being answered.** The append amendment below took the body from 408 lines to 468,
past the 450 mark this plan had set as its own trigger, and this is the change that
answers it. The body is now 399 lines, under where it started, with 101 lines of headroom
against the 500 limit rather than 32.

*What moved.* The commands, into
`skills/writing-pr-reviews/reference/delivering-a-pending-review.md`, one hop from
`SKILL.md` and linked from the section that used to hold them: creating a draft, the
`review.json` shape, checking for an open one, the two GraphQL append calls, delete and
recreate, the submit endpoint, the leak check, and the two things the API reports
misleadingly. Every command was checked as present in exactly one of the two files
afterwards, so the move lost nothing.

*What stayed, and this was the whole design constraint.* The rules. Non-negotiable 2 and
the two red flags guarding it never moved, because a data-loss rule is the worst candidate
in the file for a reference file nobody opens. What remains in the delivery section is six
rules and a pointer: check before creating, delete only what this run created, never `gh pr
review`, confirm nothing leaked and state the submit command, anchor inside a diff hunk,
and read a draft's `position`-only response as the API describing a draft rather than as a
lost anchor.

*The risk this change carries, and it is different from the amendment's.* Moving commands
out means an agent has to actually open the reference file to deliver anything. The
pointer is therefore written as an instruction rather than as a footnote, and it names why:
two of those calls come out wrong when reconstructed from memory, and both failures have
already happened in this repo. Whether the pointer is obeyed is what the rerun tests, and
delivery is what breaks first if it is not.

*All seven scenarios rerun on Opus, and the pointer holds.* Every one delivered a single
`PENDING` review with nothing published, no em dash, no first or second person, and every
permalink on the head SHA. Scenario 04 still wrote its `suggestion` block, 05 still named
`README.md` and quoted the stale port, 06 still named `platform-api`, and 07 still appended
to the seeded draft under the same review id with both seeded comments intact.

The sharpest evidence is the submit command. All seven reported it as
`pulls/N/reviews/REVIEW_ID/events`, and that string now appears zero times in `SKILL.md`
and once in the reference file. Getting it right is only possible by opening the file, so
the instruction-shaped pointer is doing its job rather than being skipped.

*A bonus the rerun settled.* Scenarios 03, 04 and 05 came back with no off-head permalinks,
where the pre-split rerun had one each. That is the `deploy.yml` fixture repair confirmed
by runs rather than only by the structural check, and it closes the false E7 for good.

**Append to an open pending review instead of failing or destroying it, and the scenario
that tests it.** Both halves shipped together, which is what the two entries said had to
happen: the rule and the scenario that makes it verifiable, rather than a rule nobody
exercised.

*The failure this answers, reproduced before anything was written.* With a draft already
open, the command the skill body stated came back `422 User can only have one pending
review per pull request`. Worse than the failed call was the instruction beside it: the
body's only sentence on the subject was "To change a pending review, delete it and
recreate it", so an agent hitting the `422` had a `DELETE` of somebody else's work as its
nearest written guidance, and that path was already in live use.

*What changed in the skill body.* A new non-negotiable 2, that a draft this run did not
create is appended to and never deleted or rewritten, and that a comment already in it
breaking the conventions is left exactly as written and named in the reply. The delivery
section gained the check for an open draft and the two GraphQL calls that append to one,
`addPullRequestReviewThread` for inline comments and `updatePullRequestReview` for the
body, both verified against the fixture rather than recalled. The dangerous sentence was
scoped rather than left standing: delete and recreate now applies only to a draft this run
created. Two red flags guard it.

*One trap written down so nobody rebuilds it.* `addPullRequestReviewComment` reads like the
obvious name and rejects `line` and `side`, because it still speaks the older `position`
model. Reaching for the plausible endpoint is the same shape as the `pulls/N/events`
mistake this suite caught once already, which is why the working call is stated rather
than left to be reconstructed.

*The rule is phrased as an action, not as a question, and that was deliberate.* "Ask the
requester" cannot be graded under `claude -p`, which has nobody to ask, so an assertion
about it would test the harness rather than the rule. Leaving the comment untouched and
naming it in the reply is gradeable headlessly and is the same behaviour interactively,
since a person reading the reply gets the question. The alternative would have produced a
sixth assertion of the family this suite keeps finding.

*Scenario 07 and its two inversions.* It is the only scenario about preserving something
rather than producing something, which makes its assertions unusually strong: the review id
after the run compared against the id before it is what separates an append from a delete
and recreate, and nothing else distinguishes them once the run is over. It inverts two
house rules. It needs a draft to exist when the run starts, so `fixture/seed-open-draft.sh`
seeds one after `reset.sh` and `run.sh` learned `EVAL_SKIP_RESET` for that scenario alone.
And its seed breaks E3 and E4 on purpose, so the every-run assertions are read over the
text the run added rather than over the merged review, since scoring the merged text would
punish the rule working.

*Both models passed on the first run.* Same review id, both seeded comments intact byte for
byte, the seeded body still the opening line, state `PENDING`, nothing published, and the
offending comment named in the reply with the rules it breaks. Worth noting rather than
celebrating: an amendment written against a failure reproduced minutes earlier is the
easiest kind to satisfy, and the scenario existing before the rule is what keeps that
honest.

*The baseline arm settles less than this rule would like, and that is recorded rather than
glossed.* Neither baseline destroyed the draft. Zero of two. The Opus one never engaged
with the review state at all and the draft survived by inaction; the Sonnet one noticed the
draft, verified its comments, posted nothing and asked what to do with it, which is close
to the wanted behaviour reached without the amendment. So the failure non-negotiable 2
forbids has not been observed in a baseline.

*What justifies it instead, and it is a different reason from every other rule here.* The
hazard was introduced by the skill's own text. The body said "To change a pending review,
delete it and recreate it" with no scope on it, and the `422` is the error an agent meets
exactly when a draft it must not touch is in the way. A baseline never faced that hazard
because a baseline never read that sentence. The rule repairs a danger the skill created,
which is a legitimate reason to write one and worth distinguishing from answering a failure
a model makes on its own. The half about leaving an offending comment alone is untestable by
a baseline in principle, since without the skill there are no conventions for the comment to
break.

*Cost, paid as `AGENTS.md` requires.* The edit to `SKILL.md` reran the suite, and it took
the body from 408 lines to 468, firing the progressive disclosure tripwire that the entry
above then answered. The rerun was one Opus run per existing scenario rather than the full two-model
matrix, which is the reading of `AGENTS.md` a regression gate needs rather than the one
that settles a rule, and it is stated here rather than quietly performed.

*No regression, and the one thing that looked like one was this repo's own doing.* Every
rerun scenario held: `PENDING` with nothing published, no em dash, no first or second
person, scenario 02's body still free of diffed files, scenario 04 still writing a
`suggestion` block with no diff markers, scenario 05 still naming `README.md` and quoting
the stale port, scenario 06 still naming `platform-api`. Scenario 01 mattered most, being
the pending-state scenario nearest the text that changed, and it came back clean with the
submit command in its correct form.

*The near miss is worth more than the pass.* Scenarios 03, 04 and 05 each produced one
permalink pinned to `2a2ca60` rather than the head SHA, and E7 duly flagged all three.
Read as a skill failure that is three regressions in the rule scenario 03 exists for.
It is not one. `2a2ca60` is the commit that added `.github/workflows/deploy.yml` for
scenario 06, and those three branches were cut from `main` before it, so the file does not
exist at their head SHAs and a head-SHA permalink to it would `404`. Pinning to the commit
where the file exists is the only link that works, so all three runs were right and the
assertion was measuring a fixture that had gone inconsistent underneath it.

The control is already in the record: scenarios 06 and 07, whose branches were cut after
the workflow landed, both link `deploy.yml` at their head SHA with nothing flagged. Same
rule, same file, opposite result, decided entirely by whether the branch predates the file.

*What that leaves to fix, and it is a fixture defect rather than a skill one.* Adding
`deploy.yml` to `main` after five branches were already cut made a file that a run can see
in its working tree and cannot link at the head SHA. The constraint written when it was
added covered the risk that it would become a second finding inside older scenarios, and it
holds: no run treated it as one. It did not cover the risk of it becoming a new link
target, which is the channel the contamination actually came through.

Fixed rather than noted: `main` is merged into all five older branches, `deploy.yml` now
resolves at all seven head SHAs, and every line number the assertions rest on was checked
afterwards and is where it was. The diffs are unchanged, the merge base having moved with
them. `setup.sh` needed no change, since it writes the workflow into the initial commit
and a rebuilt fixture never had the problem; only a live fixture patched in place drifts
this way, which is the rule the eval README now carries for anything added to `main`.

**Add a scenario for the rule about verifying architectural claims.** Written, run on
both models with a baseline each, and recorded in
`evals/writing-pr-reviews/06-architectural-claims.md`. It closes the last section of the
skill body that no scenario exercised, found by listing the body's sections beside the
five existing scenarios rather than by a run failing.

The fixture plants a name the repo contradicts. Everything in it calls the service
`example-service`, the pull request body claims the change lets "the `example-service`
container" be scraped, and the only file saying what is actually deployed is
`.github/workflows/deploy.yml`, which builds and rolls out `platform-api` and sits outside
the diff. A port disagreement inside the diff, `EXPOSE 9090` against `port = 9091`, is the
gate: a run that misses it never engaged with the change, which makes the name assertions
ungradeable rather than failed. Writing that gate in advance came straight out of scenario
04, where an assertion reachable only after an upstream judgement call reported a finding
never made as though it were a rule broken. All four runs cleared it, so nothing here is
inconclusive.

*The result.* Both models with the skill read the workflow unprompted and corrected the
author on the name, which makes this the first rule in the suite both models get right on
a first run. Neither baseline did: the Opus one opened the workflow and then named neither
`platform-api` nor `example-service`, an avoidance the scenario said in advance to record
as one, and the Sonnet one never opened it and passed the author's claim through inside a
verbatim quote.

*What the baseline arm does not settle, recorded because the table reads like a cleaner
win than it is.* Neither baseline asserted the wrong name in its own voice, so the failure
the rule forbids has not been observed. What has been observed is the rule producing a
correction that does not otherwise appear. That is weaker than scenario 01, where both
baselines published a review the rule exists to prevent, and it is the honest reading.

*A fifth assertion defect, found by these runs.* E4, the first and second person check, is
English-only. On the two Italian baselines it returned two hits and zero, and both are
wrong: the two are the Italian article `i` folded into `\bI\b` by `grep -i`, and the zero
is a reply opening "Ho guardato" and "non la mergerei". The scope is narrow and worth
stating exactly, since E5 requires a graded review to be English: E4 works on every review
the skill produces and fails only on baselines, where the reply comes back in the
language of the prompt. Every baseline in this suite is Italian, so E4 has never once been
what caught first person in one. The eval README now says to treat that row as unrun on a
baseline and grade it by eye.

Deliberately not added alongside it: a scenario for `Every observation carries a
proposal`, the other section without one. It is already covered by a red flag and no run
has produced a naked observation, so it would answer an imagined failure.

**Settle the block-versus-prose boundary with one more Opus run of scenario 04.** Run,
graded, and the answer is that no amendment is warranted. The full record sits in
`evals/writing-pr-reviews/04-suggestion-blocks.md`; what closes this entry is that the
run landed on neither outcome the task predicted.

The two fixes in that fixture separated. The three-line collapse got a `suggestion` block
anchored `start_line: 5`, `line: 7`, spanning every replaced line, which is two for two
on Opus and the assertion the scenario exists for. The conditional fix, adding `USER
1000:1000`, got prose, where the first Opus run gave it a block.

**Why no amendment: no run produced a failure to answer.** The body permits both
readings outright, "Skip the suggestion when the fix is ... anything the reader has to
decide before writing" for the prose and "Always reach for one where it makes sense" for
the block, and `superpowers:writing-skills` asks that a rule answer a failure actually
observed. Four runs on that fix produced two compliant treatments and no violation, so a
new sentence would prevent nothing. That argument holds whichever way this run had gone,
which is why it is the one carrying the decision.

The tally is secondary and deliberately not the reason: three prose against one block,
and what it settles is only that the model-level disagreement the task predicted did not
appear, since Opus landed on the Sonnet side. The remaining split is inside Opus, one
each out of two, and a within-model split is not evidence against ambiguity. Treating a
majority as the argument is the shape of the withdrawn `position` amendment, which is
why the record separates the two.

Two things the run gave for free. The submit command came back correct,
`pulls/4/reviews/REVIEW_ID/events`, which is the first evidence that this task's one
amendment to the skill body holds; run 1 had reported `pulls/4/events`, which is not an
endpoint. And a false finding about a missing `main()` guard was caught mid-run against
`README.md` and withdrawn by deleting and recreating the review, which is the
verify-at-source rule working on the skill's own terms.

Worth carrying forward: E7 passed vacuously on this run, since the review text named no
repo file outside the `suggestion` block and so had nothing to link. That is the same
weakness this scenario already records about its own first assertion, an assertion that
can pass or fail without the rule under test being exercised at all.

**Build an evaluation suite for the skill, and run it.** This was the largest gap in
the repo: nothing tested whether the rules hold. It is now seven scenarios in
`evals/writing-pr-reviews/`, the five this task built plus the two the entries above
added, one file each, carrying the prompt verbatim and a checklist
of assertions that are commands rather than judgement calls, against a sandbox fixture
of one pull request per scenario. Scenarios live at
the repo root rather than under `skills/`, because everything in a skill directory is
surface the plugin serves.

Counting runs stopped being useful once the suite started being rerun as a regression gate
rather than built. What is worth carrying instead: every scenario has been graded on both
Opus 5 and Sonnet 5 with a baseline arm on each, and the amendment above added a full
rerun on top of that.

Anthropic asks for at least three evaluations per skill and a run against every model
the skill is meant for. `superpowers:writing-skills` goes further, asking that each rule
answer a failure actually observed without it, which is what the baseline arm is for.
The model set was chosen deliberately: Opus 5, the model these reviews are actually
asked for on, and Sonnet 5, where a lighter session lands. Haiku is out, because a model
the skill never runs on is not worth the calls.

*What the baselines settle.* Delivery is what the skill contributes, and scenario 01
proves it rather than implying it: without the skill, both models submitted a published
`COMMENTED` review, each having first tried `REQUEST_CHANGES` and been refused on an own
pull request, and neither weighed publishing as a decision at all. The other eight
baselines posted nothing and wrote the review into the reply, which leaves inline-versus-body
placement and the suggestion anchor with no baseline evidence, since both need a review
object before they can be broken. Their scenario files say so rather than scoring them
as vindicated. Every baseline broke every style rule, with numbers rather than
impressions, and two produced nearly identical banned verdict lines on two different
models. Permalinks and line numbers came out intermittent rather than absent: one
baseline wrote five links unprompted and pinned two of them to the base commit instead
of the head SHA, correct today and stale the moment either file moves.

*What the two-model set was worth.* The prediction was that the style rules would give
first on Sonnet, since they fight a model's default writing voice. That was wrong: zero
em dashes, zero first or second person, zero markdown headings and English throughout,
across all five Sonnet reviews. Two other rules gave instead, both costing an extra step
at writing time, and both then got a second run. The bare repeated mentions on scenario
03 did not reproduce. The missing `suggestion` block on scenario 04 did, and turned out
to be measuring a finding never made rather than a rule broken, which the third Opus run
recorded in the entry above then settled. Scenario 02's Opus inconsistency, a body restating a fix its
inline comment already carried, did not reproduce on Sonnet either.

*Net effect on the skill body: almost nothing, which is the point.* One amendment was
made, and it answered a real failure: a submit command rebuilt into `pulls/N/events`,
which is not an endpoint, so the body now states the command rather than leaving it to
be reconstructed, and says how the API reports a pending anchor. An earlier amendment
about `position` was written and then withdrawn, because the failure it answered had not
happened: a pending comment reads back with `position` set and `line` null whatever the
payload sent, and the payload had been right all along. Cost, one wasted run and one
wrong entry in a scenario file, and it is worth keeping in view, since it is the skill
body's own rule about verifying a claim at its source, broken while grading a skill
built to enforce it. Three further amendments were considered off single runs and all
three would have been wrong.

*Four defects in the assertions, all found by runs.* Two reported violations on clean
reviews, both counting where they should have been reading: comparing a count of
filenames against a count of links double-counts every link, since the name appears in
the link text and again in the URL. Two failed the other way, passing a real violation
in silence: a head-SHA check written as `grep -c 'blob/main'`, which a base-commit pin
satisfies, now a prefix match of every `blob/<sha>` against `headRefOid`; and a
first-person check listing six forms while "me", "my" and the contractions went through
it. The pattern in all four is an assertion testing the failure imagined while writing
it rather than the one the rule is about.

*Three harness lessons, each of which would have read as a skill failure.* A machine can
scope its git identity to a directory tree, leaving the fixture's temporary clone
without one, so the fixture derives an identity from the authenticated account. A run
has to happen from inside a clone of the fixture, or `gh` resolves the wrong repository
entirely. And a fresh `CLAUDE_CONFIG_DIR` removes the plugin and the credentials
together, since credentials live in the keychain keyed to the real directory, so a
baseline disables the plugin instead and re-enables it from a trap. The grading recipe
also assumed a review object exists, which eight of the ten baselines never created, so
it now says the reply is the text to grade when nothing was posted.

`AGENTS.md` carries the line that makes the suite a regression suite rather than a
one-off: every edit to a `SKILL.md` runs that skill's scenarios before the change is
offered.

**Rename the plugin namespace to `myai`.** A skill is invoked as
`/myai:writing-pr-reviews`. The objection raised against the name and set aside:
in a listing where every plugin ships skills for an AI agent, `ai` distinguishes
nothing, since what a namespace carries is whose the skills are rather than what
they are. What the rename cost is the answer to the question the packaging entry
below left open, and it is smaller than that entry assumed: the namespace comes
from the `name` field in `.claude-plugin/plugin.json` alone, so renaming the
symlink was tidiness rather than a requirement, and the rest was eleven mentions
in the prose.

**Validate the skills in CI, and before a commit.** `tools/check.sh` is the single
command: formatting, lints as errors, then the test suite. A commit runs it through
`.githooks/pre-commit`, which each clone points at with `git config core.hooksPath
.githooks`, and `.github/workflows/checks.yml` runs it on every push and pull
request after installing the Rust version `.tool-versions` names, then adds
`claude plugin validate`. One script rather than three copies of the same list,
because a hook and a workflow that drift apart teach a working copy to lie.

The greppable rules went into the suite rather than into shell steps in the
workflow, for the same reason: no machine-local paths, no owner name outside its
two allowed places, and a README skills table that matches `skills/` in both
directions, since a row pointing at a deleted skill is the same drift as a skill
with no row.

The list this task carried, closed:

- [x] the checks themselves, for every `skills/*/SKILL.md`: the frontmatter
      parses, `name` matches the directory name, `description` is present and
      under 1024 characters, and the body is under 500 lines, plus the two rules
      that came out of writing the tests, a description cut short by a YAML
      comment and a directory holding no `SKILL.md`
- [x] a GitHub Actions workflow running that suite
- [x] the greppable rules from `AGENTS.md`: no machine-local paths, and no owner
      name outside the clone URL in `README.md` and the copyright line in
      `LICENSE`
- [x] a check that the README skills table carries a row for every directory
      under `skills/`, and, added after the fact, no row for a skill that is gone
- [x] run `claude plugin validate` in the same workflow, tolerating its expected
      warnings
- [x] update the `AGENTS.md` claim that there was "no build, and no test suite"
      and add the checks to its Layout block
- [x] not on the original list: formatting and lints in the same gate, and
      `.githooks/pre-commit` running it before a commit

Three things the writing taught, none of them foreseen here. The name to look for
has to come out of the copyright line, because a check spelling it out would break
the rule it enforces. It has to be the whole name, because a first name flags this
plan's own record of `paolo-prs-review`. And the fixtures have to assemble
machine-local paths at run time, since a literal one in a test file is a violation
the suite reports against itself, which is the self-specification trap this plan
spotted, one level deeper than it looked. The flake the plan expected did not
materialise either: `claude plugin validate` exits 0 with its warnings, so
tolerating them reads the exit status and matches no text. Worth carrying over
from the original entry: the `name` check is load-bearing rather than cosmetic,
because in a plugin skill `name` supplies the last segment of the command, so a
drifting `name` means the command stops matching the directory it lives in.

**Recover the half of the description YAML was dropping.** The frontmatter did
list the trigger phrasings in both languages, which is why this plan recorded
that as already in line. The loader never saw them: an unquoted `#` in
`"dammi un parere sulla PR #N"` opens a YAML comment, so the value arrived 147
characters long instead of 288, and every phrasing after that point, all the
English ones included, was silently dropped. A `>-` block scalar fixes it without
changing a word of the text. `AGENTS.md` now carries the rule for anyone writing
a description, and the checker reports the shape, so it cannot come back
unnoticed. Worth remembering how it surfaced: every other check on that file
passed, and only parsing the frontmatter as YAML rather than reading its lines
exposed it, which is the whole argument for the checker being a real program.

**Add a LICENSE and a .gitignore.** MIT, with the copyright holder named, which
cost a second exception to the no-personal-names rule: a holder line has to
identify someone for the grant to mean anything, exactly as the clone URL has to
name the owner to be an address. Both exceptions now sit together in `AGENTS.md`.
`.gitignore` covers `.DS_Store` and nothing speculative. The urgency this entry
originally claimed was wrong, and the correction is worth keeping: the repo is
private, so no content was unusable by anyone, and the license is there for
whenever the repo opens rather than to fix a problem that exists today.

**Package the repo as a skills-directory plugin.** Originally queued last, as an
open question between symlinks and a plugin. It moved to the front once
recognisability became a requirement: the plugin namespace supplies it, which lets
each skill keep a clean descriptive name instead of encoding ownership into the
directory name. `.claude-plugin/plugin.json` names the plugin `myai`, one symlink
of the repo root into `~/.claude/skills/` replaces one symlink per skill, and no
marketplace is involved. The reasoning is recorded in `AGENTS.md`; the risk that
plugin discovery would not follow a symlink was tested first and does not
materialise.

**Rename the skill to the conventional form.** `paolo-prs-review`, then
`my-pr-reviews`, now `writing-pr-reviews`, invoked as `/myai:writing-pr-reviews`.
Anthropic recommends the gerund form and `superpowers:writing-skills` asks for
active voice, verb first. `writing-` rather than `reviewing-` because the skill
states in its own opening that it holds conventions for writing and delivering a
review, not advice on how to find defects, and a name promising review methodology
would over-claim.
