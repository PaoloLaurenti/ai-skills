---
name: tdd
description: >-
  Use when about to write or change production code that adds or alters behaviour,
  such as a feature, a behaviour change or a bug fix, and when asked for TDD,
  including phrasings like "red-green-refactor", "let's TDD this", "test-first",
  "write the test first", "drive this with tests", or "reproduce the bug with a
  failing test before fixing it". Not for refactoring that keeps behaviour, documentation,
  configuration, or adding tests to code that already exists.
---

# TDD

Test-driven development here drives the design of production code; the tests are a
welcome side effect. The code being built is a **black box**: a test sees only its
surface, arranges the context the box lives in, sends it a stimulus and checks the
reaction. Every cycle goes **red, green, refactor**, then looks at the plan again.

**One test at a time. Red before any production code. Every refactoring step stays
green. The plan changes only with the requester's yes**: the requester steers the design
through the plan and the stops below.

## Non-negotiables

1. **The tests see the surface only**: what calling code can see and touch, the public
   API. A test reaches no private method, no internal type, no database behind the API.
   A unit is a unit of behaviour, not a class or a function.
2. **The three rules of TDD hold, word for word**, as Robert C. Martin states them:
   > 1. You are not allowed to write any production code unless it is to make a failing
   >    unit test pass.
   > 2. You are not allowed to write any more of a unit test than is sufficient to fail;
   >    and compilation failures are failures.
   > 3. You are not allowed to write any more production code than is sufficient to pass
   >    the one failing unit test.

   They govern code that adds behaviour; refactoring on green adds none and follows
   [Refactor](#refactor), preparatory refactoring and seams in untested code included.
   "Sufficient to fail" means one whole test of one behaviour, no assertion beyond what
   it needs, written whole so wishful thinking can shape it; its compilation failure is
   the first red, and only the code making it compile comes next.
3. **The plan lives in the test file as pending tests**, and changes only after the
   requester's yes: adding, deleting, reordering or rewriting an entry is a stop.
4. **Refactoring is the third step of every cycle**, never a later phase, and it happens
   on green: every test is run after every refactoring step, and a step that turns
   anything red is undone.
5. **Tests are dumb**: no branch, loop or computation, no business logic written a
   second time, and the expected value traces back to Arrange inside the test itself.
6. **No commit on the agent's own initiative.** When the requester or the project asks
   for commits, there is one per completed cycle, after refactoring with every test
   green, in the project's message format.
7. **Every stop in [Stops](#stops) is a real stop**: the turn ends on the question, and
   the work resumes only on the answer.

## Before the first test

Two questions, each in a turn of its own, each skipped when the request or the project's
rules already answer it:

1. **Whether to proceed with TDD at all.** A no ends this skill: the work goes on without
   it. A request that already names TDD, test-first or red-green-refactor has answered.
2. **Only after a yes: whether to pause after every cycle.** Without the pause the agent
   stops only where [Stops](#stops) says; with it, it also stops at the end of every
   red-green-refactor-review cycle, showing what the cycle did.

Then **run the whole suite** and record the result: every check of the cycle compares
with this last green. A suite that is not all green is a start of its own, see
[A suite already red](#a-suite-already-red).

## The plan

### The surface first

A bit of upfront design opens the plan. In a few lines, state:

- **the entry point** the tests call in Act: the function, command or public method;
- **the external dependencies** the box uses, which become test doubles: repository,
  gateway, clock, notifier, anything that reaches outside the process;
- **the observable reactions**: the returned value, the state read back through the
  public API, the calls made to dependencies.

Whatever is not deterministic is one of those dependencies, as
[Determinism](#determinism) says.

### The entries

Each entry is a test name stating one business behaviour; together they are the
**Test List**. Each next entry is a **One Step Test**: it teaches something new, and it
can be taken to green in one confident step. Write them in this order:

1. **Happy paths, simplest first.** Simpler means fewer moving parts: fewer inputs that
   vary, fewer dependencies involved, fewer business rules applied together. On a tie,
   the one whose green needs less production code comes first. The first entry is the
   smallest success that makes real behaviour move. A behaviour over a collection is
   born on a single element first and generalised by a later entry: **One to Many**.
2. **Degenerate and empty cases close the happy paths**, such as an empty cart totalling
   zero. They have the fewest moving parts, yet with nothing moving they drive no design,
   and they are often green before any code exists.
3. **Domain errors**, the business rules a request can break.
4. **Input validation.**
5. **Edge cases.**

The plan is a starting point, not a contract: the cycles will show what it missed and
what it no longer needs.

### Naming a test

The name is the plan's entry, so it states the behaviour in domain words, as short as
still tells it apart from the other entries: `apply promo code discount`. The condition
goes in only when it is what distinguishes the scenario: `rejects a booking outside
opening hours` beside `accepts a booking within opening hours`. A name carries no class,
method or technical detail, and no concrete value, which belongs in Arrange.
`Should_cart_aggregate_apply_20_percent_discount_based_on_promo_when_updating_with_its_code`
fails on all three: the technical name, the value, the mechanism. Casing and phrasing
follow the project and its test framework.

### Writing it down

Write each entry into the test file as a **pending test**: its final name, an empty body,
marked with the framework's own mechanism (`it.todo`, `@Disabled`, `#[ignore]`,
`t.Skip`, `pytest.mark.skip`). The runner then reports the plan's state, done and pending,
and a new session reads it from there. A pending test is a title, not a test written
ahead: each body is written only when its turn comes.

Present the surface and the entries together, then **stop for the requester's go**.
No body is written before it.

### A plan already written

A plan found in the test file, from an earlier session or someone else, is shown with
each entry's state, done or pending, along with any entry the code has since made stale
or missing, and the agent **stops for a go** before the first cycle. The file cannot tell
whether the plan was ever approved, and the code may have moved since.

### When something technical is unknown

When the first test cannot be written because something outside the box is unknown, how
an external API answers, what a library returns, whether an approach holds up, **propose
a spike**, a stop: what is unknown, what it will find out, how small it will be. Its
code lives in a temporary directory outside the project's tracked files and is thrown
away, not one line copied into production code. What it taught is reported with the
plan, so it shapes the surface and the entries. Uncertainty about business behaviour,
what should happen when a code has expired, is a question to the requester instead.

## The cycle

### Red

Goal: a test in its final form that compiles and fails on its assertion, for the
expected reason.

1. **Take the first pending test of the plan**, and only that one.
2. **Write its body by wishful thinking, Assert First**: start from the assertion, what
   must hold, comparing the whole expected value; then the Act that makes it hold, calling
   the API the way it would read best, whether or not that API exists yet; then the
   Arrange it needs, holding only the data that matters to the behaviour. The test is
   written in its final form, following [Writing a test](#writing-a-test), not as a draft
   to clean up later.
3. **Run it and see the compilation failure**, the first red, where the language has one.
   Then **make it compile with the least production code**: signatures, types, constructors,
   with a body returning a trivial wrong value, empty, zero, null, so the test fails on
   its assertion. A thrown "not implemented" fails the test without proving the
   assertion can catch anything. In a dynamic language the same holds: the first failure
   is the assertion, not a missing function.
4. **Run the tests and read the output.** The new test fails on its assertion, with a
   message that says what is missing, and every other test stays green. An unclear
   message is fixed, in the test or its assertion, before going on.

A new test that **passes at once** has never been seen failing. Prove it can:

1. **Break production code on purpose**, with the smallest change that makes exactly the
   behaviour the test names wrong, such as the value returned in that branch; a broad
   break, a whole function commented out, would fail it for the wrong reason.
2. **Run it**: it fails on its assertion, for the expected reason, like any red.
3. **Undo the break** and run every test: all green again.
4. **Tell the requester**, without stopping, and go on. The plan review proposes deleting
   it if another test already covers it.

A targeted break that **does not make it fail** means the test does not check what its
name says: stop and propose rewriting it.

A test that **does not write simply** from the surface is never forced. Stop, name what
makes it hard, an oversized Arrange, too many doubles, a reaction that cannot be
observed, an Act in several steps, and propose one of three ways:

- **a different surface**: the API the test wishes it had;
- **a split**, a **Child Test**: the entry broken into smaller behaviours;
- **a preparatory refactoring** that makes the test easy to write. It happens on green:
  the test goes back to pending first, every rule of [Refactor](#refactor) holds, and the
  red resumes afterwards.

### Green

Goal: the assertion satisfied, as soon as possible.

- **Write the least production code that turns this test green**, **Fake It** if need
  be. Sins are allowed: hardcoded values, duplication, specific branches, provisional
  names. Cleaning up is the next step's job.
- **Write nothing for a behaviour that belongs to a later pending test**, even when it
  is plain where the code is heading. Code turns generic only when a test forces it, as
  tests get more specific the code gets more generic: **Triangulate**, a hardcoded
  answer that stops satisfying two tests at once, or the refactor step removing the
  duplication between the test's data and the code.
- **Obvious Implementation** is allowed when it is as short as the hardcoded one and
  covers no behaviour named by a pending test: `return a + b` rather than `return 5`.
  **Tell the requester each time** it is taken, naming the pending tests it was checked
  against, since a pending test has an empty body and cannot be run to prove the code
  stayed narrow.
- **Take the highest transformation that is enough**, per the Transformation Priority
  Premise. **Open [reference/transformations.md](reference/transformations.md) at every
  green**: it ranks the transformations, and a green that reaches past one high enough
  writes code no test asked for yet.
- **Run every test**: the new one green, every other one still green. An old test that
  turns red is fixed in production code; changing a test already green is a change to
  the plan, so a stop.

Green that does not come from a **small, direct change** is a stop. Say what blocks it
and propose how to change the test or the plan: an earlier test needing a simpler
transformation, or a **Do Over**, the work since the last green thrown away and the test
started again, rather than building something elaborate to get there.

### Refactor

Goal: clean, readable code, production and tests alike, behaviour unchanged.

Look at, after every green:

- **in production code**: duplication, including between the test's data and the code,
  which is where a hardcoded value turns generic; names; the rules of simple design,
  in priority order: passes the tests, reveals intention, no duplication of knowledge,
  fewest elements;
- **in the tests**: names, the three blocks, the link from Assert back to Arrange, any
  logic that crept in.

How:

- **Behaviour stays as it is.** Nothing new enters production code. A test changes shape,
  never what it checks.
- **Small steps, every test run after each one**, never once at the end.
- **A step that turns anything red is undone back to the last green**, never fixed
  forward, and tried again smaller or by a longer path that stays green: add the new
  beside the old, move one caller at a time, remove the old. Red is the moment the step
  proved bigger than it looked, which is no moment to go on without the net.
- **Local refactorings are done**: names, local duplication, extracting a function.
  **Structural ones are proposed**, a stop: a new type, an abstraction, a responsibility
  moved. Propose waiting for the next tests when the right shape is not clear yet: a
  refactoring made too early sends the design down a path the next tests then fight.
- **A deferred refactoring is not written down anywhere.** The smell stays visible in the
  code, and every later refactor step considers it again. The last refactor step of the
  plan proposes each one still open.
- **A collaborator extracted here**, a new type or module inside the box, gets no tests
  of its own: the surface's tests that brought it into being cover it, and tests of its
  own would tie the suite to the structure refactoring has to stay free to change. Only
  when it gains a public surface of its own, used by other callers, or logic rich enough
  that the outer surface checks it awkwardly, propose making it a black box with a plan
  of its own.

### Review the plan

Every cycle closes by looking at the plan again:

- **A missing test**, **Another Test**: a behaviour the cycle brought out that no entry
  covers. Propose it with its name and its place in the plan's order.
- **A redundant test**: another test checks the same behaviour and would fail for the
  same reason, so deleting it loses nothing. The usual case is an early, simple test that
  later, more specific ones include. Propose deleting it, naming the test that covers it.
- **The direction**: as the tests get more specific, the code gets more generic. Check
  whether this green generalised the code or added a special case: a branch or constant
  matching one test's data, beside another of the same shape, is code mirroring the
  tests. The first Fake It is specific by choice; past it, the first mirror is a stop.
  Say the road is likely wrong, and propose reordering the tests, a Do Over, or a
  different surface.

Each proposal is a stop. With nothing to change, move to the next pending test, or pause
if the requester chose a pause after every cycle.

## Writing a test

### Anatomy

Three blocks, in this order, set apart by a blank line:

1. **Arrange**: the context the box needs, its dependencies and the data that matters to
   the behaviour. Setup irrelevant to the behaviour goes into builders or object mothers
   named for what they prepare.
2. **Act**: a single stimulus to the surface.
3. **Assert**: the reaction. By default **one whole expected value, compared by
   equality**, never a proxy such as a length, one contained item or not-null. Several
   assertions only when the behaviour has distinct observable reactions that do not fit
   one value, a returned value and a call recorded by a spy, each about the same
   behaviour. Failure messages say specifically what went wrong.

**The expected value traces back to Arrange, inside the test**: **Evident Data**. It is
built from names Arrange gives, or is a literal whose link to Arrange is plain at a
glance. It is never computed with the production formula, which passes by construction,
and never a literal repeating data hidden in a helper, which turns reading the test into
a scavenger hunt:

```text
# the expected value repeats the production formula
price = 100
discount = 20%
cart = cartWith(item(price))
total = cart.apply(promoCode(discount))
assert total == price * (1 - discount)

# the link shows by itself
cart = cartWith(item(price: 100))
total = cart.apply(promoCode(discount: 20%))
assert total == 80
```

**No comments by default**, unless the project's rules ask for them, so the blocks are
told apart by blank lines, not `// Arrange` markers. A comment that survives either
clarifies a detail the test's code cannot express, or describes the scenario very
briefly.

### Determinism

A test is a safety net only if, with the code unchanged, it always gives the same result.

- **Whatever is not deterministic enters the box through a boundary the test controls**:
  current time, random values, generated IDs, the environment. In the test it is a stub,
  fixed under a name in Arrange, so the expected value traces back to it: a clock fixed at
  10:00, a code expiring at 10:30. In a new feature those boundaries belong to the surface
  designed with the plan; in existing code, opening one is a preparatory refactoring.
- **Every test is independent**, an **Isolated Test**: it arranges its own context, shares
  no mutable state, and passes alone and in any order. That is also what lets the suite
  run in parallel and keeps its lead time short, so tests hold no global state, fixed
  file, port or shared record. Switching the runner to parallel is the project's
  configuration: propose it.
- **A flaky test**, passing or failing with the code unchanged, at any point, is a stop:
  report it and look for its source of non-determinism, rather than running it again
  until it passes, or ignoring it.

### Test doubles

Replace only the boundaries the project owns, repositories, gateways, clock, notifier,
and use real domain objects and value objects everywhere else. Pick the double that fits
the job:

- a **stub** to steer the code with canned answers;
- a **spy** for outgoing commands, compared by equality in Assert. It is the usual choice,
  since it keeps Act and Assert apart;
- a **mock** only when the exact protocol of calls, order, arguments, no other call, is
  the behaviour under test;
- a **fake** when a working lightweight implementation, an in-memory repository, reads
  better than a pile of stubbed answers;
- a **dummy** only to fill a parameter the path never uses. It proves nothing: a call
  that must not happen is checked with a spy that received none.

**Dummies, stubs and fakes support a test and are never what it verifies.** Assert reads
the returned value, state read back through the box's public surface, or the calls a
spy recorded or a mock verifies; never the inside of a fake. A booking saved into an
in-memory repository is checked by finding it through the box, and a surface with no
way to read it back is a test that does not write simply.

Every double is named after the boundary it replaces with the `Double` suffix, whatever
its kind: `ClockDouble`, `NotifierDouble`, `BookingRepositoryDouble`. Each constructor
names the role it sets up, opening with the kind, so the test shows which kind it uses
on the line that builds it, and one double can serve as a stub here and a spy there
without a rename. The variable takes the concept's name, since the kind already shows,
and casing follows the project:

```text
clock    = ClockDouble.stubFixedAt(10:00)
notifier = NotifierDouble.spy()
bookings = BookingRepositoryDouble.fakeInMemory()
logger   = LoggerDouble.dummy()
```

**Read [reference/test-doubles.md](reference/test-doubles.md) when unsure which kind a
double is.** It defines each kind, and a double built as one kind and used as another
misleads the next reader about what the test proves.

### Parameterized tests

A table of cases comes into play only when **combinatorial complexity** emerges among the
scenarios: the behaviour depends on combinations of inputs, and entries start to differ
only by which combination they pick. Two tests that merely look alike are not reason
enough.

- **A table is never written up front.** Each case is an entry of the plan, written
  through the cycle. A refactoring of the tests merges green ones into a table, and only
  when they check the same behaviour with the same reason to fail.
- **Rows needing a branch in the body**, an expected value of another kind or a different
  error message are different behaviours and stay separate tests.
- **Each row reads on its own**: input and expected value side by side, a label naming
  the case, the expected value written and never computed in the body.
- **A new row is a new entry of the plan**: it goes red first, and one that passes at
  once is verified by a targeted break like any new test.

## Other starts

### A bug

The plan opens with a **Regression Test** that **reproduces the bug through the
surface**, named after the correct behaviour, `applies the discount to discounted items
too`, never after the bug or an issue number, and placed in the test file of the
behaviour it concerns. It is a plan like any other and goes through the same stop, even
with one entry.

- **Red shows the bug**: the test fails on its assertion, the actual value being the
  wrong behaviour reported.
- **A test that passes means the bug is not reproduced**: the reported behaviour is
  itself the break, so there is none to make. Touch no production code. Stop, say what
  was tried, and ask for details or propose another scenario.
- **Needing the inside to reproduce it**, a private method, the database instead of the
  API, is a signal about the surface or the design. Stop and say so.
- Then the cycle as usual. The plan review also asks whether the same defect can show up
  in nearby cases.

### A suite already red

When the run before the plan is not all green, through broken tests, a build failure or
a flaky test, **stop before writing anything**. Report which tests fail, how, and whether
they touch the area of the work. The requester chooses:

- **fix them first**, as separate work; a bug follows this skill in turn;
- **go on with the known failures**, each one named. Every check of the cycle then reads
  "no failure beyond the known ones", and a known failure that changes, starting to pass
  or failing differently, is a stop.

Broken tests stay as they are: disabling, deleting or ignoring one to turn the suite
green happens only on the requester's explicit request. A flaky test is handled as
[Determinism](#determinism) says.

### Code that has no tests

Changing the behaviour of untested code needs a safety net first.

1. **Propose a plan of characterization tests**: tests pinning the current behaviour of
   the surface the change will touch, as it is even where it looks odd, limited to that
   area and named for behaviour. It is a stop.
2. **Write them one at a time.** They are the one exception to red first: written after
   the code, they pass at once, their expected value read by running the real code, not
   imagined. One that fails means the code was misread, not a bug to fix.
3. **Verify each one by a targeted break**, as for a new test that passes at once: break
   exactly the behaviour it pins, see it fail on its assertion, undo, every test green
   again. A break that does not make it fail means the behaviour lives elsewhere than
   where it was looked for, or the test checks something else: say so in the
   explanation that follows.
4. **After each one, explain what behaviour it pins down, and stop for a go**, whatever
   was chosen about pausing after every cycle. The expected value comes from the code
   rather than from anyone's intent, so each explanation is a discovery that can change
   the next test or the plan for the change. A behaviour that looks genuinely wrong is
   reported, and stays as it is while the net is built.
5. **A surface that cannot be tested** needs a few mechanical refactorings made without a
   net, such as passing a dependency as a parameter or wrapping it behind an interface.
   Propose each one with why it is safe, and make it only after a yes.
6. **Then the plan for the change itself**, built, approved and run like any other, with
   the characterization tests as part of the net.

## Stops

Every one of these ends the turn on a question:

- the plan, before the first body is written, and a plan found already written;
- any change to the plan: adding, deleting, reordering or rewriting an entry, changing a
  test already green;
- a new test that a targeted break of production code cannot make fail;
- a test that does not write simply from the surface;
- green that does not come from a small, direct change;
- code mirroring the tests instead of turning more generic;
- a structural refactoring, or the proposal to wait before one;
- a spike;
- a suite not all green before the plan, a known failure that changes, a flaky test;
- a bug that does not reproduce, or that needs the inside to reproduce;
- the plan of characterization tests, each characterization test, each refactoring made
  without a net;
- the end of every cycle, when the requester chose that pause.

## When the plan is done

When the last pending test is green, refactored, and the plan review adds nothing, run
the whole suite one last time and report these five entries, **every one always
present**, "none" when empty, so an empty entry reads as checked rather than forgotten:

1. **The suite**: passed, failed, zero or only the known failures accepted at the start,
   and ignored.
2. **The behaviours implemented**: the final plan's test names.
3. **How the plan changed** from the one approved: each addition, deletion or rewrite,
   with its reason in a line.
4. **The obvious implementations taken**, listed together though each was reported at
   the time.
5. **The deferred refactorings left open**, with where in the code: those the requester
   declined at the last refactor step.

## Red flags

Each of these means the practice has slipped. Stop and go back to the rule it breaks.

| Thought | Reality |
| --- | --- |
| "I can see the whole solution, I'll write it now" | Green is the least code for this one test. Behaviour of a pending test waits for that test. |
| "I'll write the tests first and the code after" | One body at a time, each red before its code. The plan holds titles, not bodies. |
| "This internal class deserves its own test" | Tests see the surface. An extracted collaborator is covered by the tests that brought it into being. |
| "I'll throw `not implemented` to get the red" | Red is a failing assertion. Return a trivial wrong value. |
| "The new test passed straight away, good" | It was never seen failing. Break the code where it points, watch it go red, undo. |
| "This test needs a long setup, but it works" | A test that does not write simply is design feedback. Stop and propose a surface, a split or a preparatory refactoring. |
| "Green is close, one more workaround" | Green that needs more than a small, direct change is a stop. |
| "I'll clean it up at the end" | Refactoring is the third step of every cycle. |
| "The refactoring broke a test, I'll just fix it" | Undo the step back to the last green, and try a smaller one. |
| "It is only moving code around, I'll just do it" | A new type, an abstraction or a moved responsibility is structural: propose it. |
| "This test is redundant now, I'll delete it" | Deleting, adding or rewriting an entry is the requester's call. Propose it. |
| "The expected value is easiest computed the way the code does it" | Then the test passes by construction. Write the value, traced back to Arrange. |
| "Those tests were failing before I started" | A suite not all green is a stop before the plan. |
| "It failed once, I'll run it again" | A flaky test is a stop. Find the non-determinism. |
| "I'll use `now()`, it is the quickest green" | Time, randomness and IDs enter through a boundary the test controls. |
| "The spike works, I'll keep its code" | Spike code is thrown away. What it taught goes into the plan. |
