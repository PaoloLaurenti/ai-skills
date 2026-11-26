---
name: writing-cards
description: >-
  Use when splitting a feature into cards for a tracker (Linear, Jira, GitHub
  Issues, Trello or any other), writing, rewriting or reviewing a card, or deciding
  what happens to the existing cards of a project, including phrasings like "crea
  le card su Linear per questo flusso", "scrivi la card", "ridammi la card",
  "rivediamo le card del progetto", "split this feature into tickets", "write the
  issues for this epic", or "rewrite this story". Not for building what a card
  describes.
---

# Writing Cards

A card tells the engineers what to build and why, and gives them every fact they
need to decide how. Two words carry the rules: a card is a **slice**, a thin
piece of the product that a person can watch working in a demo, and every fact
in it is **sourced**.

## The team's settings

Four things are asked once per team, and kept in
`~/.local/state/writing-cards/<tracker>-<team>.md`, never in the project:

- where the team's Definition of Done lives, or that it has none;
- where its Definition of Ready lives, or that it has none;
- the language the cards are written in;
- the status a card goes in while it is not ready.

When the file exists, the first message summarises it, so the requester can
correct what changed. Whatever it lacks is asked, and the answer added to it.

## A set of cards

- **Each card is the smallest slice possible.** Once done, it adds one feature
  that a person can see working on the product, on top of the slices before it,
  and nothing that could ship as a slice of its own. A technical card is an
  exception, and the card says why it is needed.
- **A card holds all the work its slice needs**, in every part of the system:
  app, backend, configuration. Cut cards by behaviour, never by component or team.
- **Keep dependencies few**, so that several cards can be worked at the same time.
- **Order lives in the tracker's own tools**: "blocked by" relations, and the
  position in the list, most important at the top. Titles carry only the name of
  the slice.
- **Every scenario has a home** somewhere in the set: happy path, errors,
  exceptions, validations and edge cases.

## One card

**A card is short and plain**: it fits on one screen, and a person who did not
plan it understands it at first read. No wall of text: detail the engineers can
open lives in the linked sources, not in the card. Before showing a card, rewrite
it as short and as simple as it can be said to the engineers who will read it.
Where blader's `humanizer` is installed, pass the card and every message drafted for
the team through it too, keeping the card's sections, links and the form of its
acceptance criteria as they are.

### Its structure

- **The team's Definition of Ready decides it, when it sets one**: its sections,
  in its order. A section matching one of the defaults below is filled by the
  Definition of Ready's instructions, and by the default ones where it gives none.
  A section with no default and no instructions is a question to the requester.
  A default section the Definition of Ready leaves out stays out.
- **Otherwise, the default structure below**: when the team has no Definition of
  Ready, or one that says nothing on how a card is built.

The default sections, in this order:

1. **What**: what a person can do once the card is done, told as they experience it.
2. **Why**: the reason for the slice.
3. **How <mechanism> works**: only when the card rests on a mechanism the engineers
   need explained, such as how a session stays alive: a link to the document that
   explains it, rather than a copy, or plain words where none does. Placed before
   the criteria because they build on it.
4. **Acceptance criteria**.
5. **Open points**: only on a skeleton card, see below.
6. **Notes**: design references, architecture constraints, links to sources.
7. **Out of scope**: last. Each item names the card that takes it, when one exists.

### Acceptance criteria

- **Form**: "As a <actor>, when I <action>, I <outcome>." The actor is "a user",
  qualified when it matters: "a new user", "a returning user", "a signed-in user".
- **They are the demo.** Each one describes what a person does on the product and
  what they see, never the internals. A check that nobody can show in a demo, such
  as data staying out of a phone backup, moves to Notes as something the code
  review verifies.
- **Leave out what the team's Definition of Done already asks for**, such as
  automated tests and reviews. A non-functional requirement appears only when
  it is specific to this card. A card names a point of the Definition of Done
  only to skip it, saying which and why, where the Definition of Done allows it.

### Words

- **What and the acceptance criteria use the words of the screen**: "the sign-in
  page that opens over the app", rather than the component that shows it. The
  technical term goes in Notes.
- **Use the team's own names** for environments, systems and roles, as the
  requester uses them.
- **State the what and the why; the engineers own the how.** Name a constraint when
  it binds, such as a security rule or an existing endpoint, and leave out the
  implementation step that satisfies it.
- **Plain language at B2 level, in the cards' language from the team's settings**:
  short, grammatical sentences, one term per concept.
- **A reference to another card is a link whose text is that card's title.**
- **A reference to a card outside the scope is its plain title**, with no link
  and no id: some trackers turn a link or an id into a change on the card it
  names.

### Skeleton card

When decisions are missing, write a skeleton: the sections of the card's structure
that can be filled so far, and an Open points table with the columns `#`,
`Question`, `Who decides`. Its Notes say what moves it to ready. It goes in the
not-ready status from the team's settings.

## Sourced facts

- **Read each source whole**: a page together with its comments and
  discussions, a design frame together with its notes. Discussions often hold
  the latest decision, and some tools return them only through a separate call.
- **Every fact traces to a source**, in a card and in every message drafted for
  the team: a design file, a document, the code, or a decision the requester
  made in the conversation. A fact with no source becomes a question to the
  requester. An open gap is always better than a guess.
- **When two sources disagree, ask which one wins**, and note the answer with the
  decisions behind the card.

## Steps

1. **Read the sources and the current cards.** Find the team's Definition of Done
   and Definition of Ready through the team's settings, and read both whole, every
   time. Read the heavy sources, such as design files and long lists of cards,
   through a subagent that returns text only, and ask for an image at the smallest
   size that is still readable. Done when every source the requester named has been
   read and every card of the project that is not done is listed.
2. **Agree the scope, and where approved cards are kept.** Ask which cards the work
   may change, for example those with a given label. Changes reach only cards in
   scope that are not done. Ask too where the approved cards and the decisions
   behind them are saved until they reach the tracker, such as a page or a file the
   requester names. Done when the scope is a rule you can check card by card, and
   that place is named.
3. **Ask about the open decisions**, one question per message. Before the first one,
   when Matt Pocock's `grilling`, from `mattpocock-skills`, is in your skill list, load
   it and follow it to find the questions and their order, with one change: a single
   question per message, with its recommended answer, never a round of several. Done
   when every decision has an answer or is an open point of a card.
4. **Propose the slices, in order, then have them checked for thinner cuts.**
   Hand the slices and the sources to a subagent, asking of each one whether it
   splits into smaller slices that a person can still see working, and how.
   Bring each split it finds to the requester, one per message, and send the
   slices that result back to the same check. Then go through the existing cards
   one at a time: keep, rewrite, absorb into a slice, move to backlog, or cancel;
   a kept or rewritten card goes through the same check. A cancelled card gets a
   comment naming the card that absorbed it. Done when the check finds no
   further split the requester accepts, and every existing card has an approved
   fate.
5. **Draft one card per message**, and wait for approval. Once a card is approved,
   save its text and the decisions behind it in the durable place the requester
   chose, then move on. Done when every approved card is saved there.
6. **Write to the tracker** only after an explicit yes to the write plan; choosing
   an option is not that yes. Before the batch, record id, status, labels and last
   update of every card outside the scope. Create the new cards first, then add the
   links between them, since a card has an id only once it exists. After the first
   write, check whether the tracker changed anything by itself from the text: some
   trackers turn every card id or link in a description into a relation on the other
   card. Done when the record compared with the tracker shows every card outside the
   scope unchanged, or every difference has been reported to the requester.
7. **Hand over what the tracker's API cannot do**, such as a manual order, as a short
   list for the requester.
