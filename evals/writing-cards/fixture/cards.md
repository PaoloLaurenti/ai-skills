# Password reset: approved cards

Linear, team Mobile. Every card below was approved one at a time, and the slices were
checked for thinner cuts.

## Scope

The cards with the label `reset-password`, and the new cards, which take that label too.
Cards without that label stay as they are.

## Decisions

- The two new cards go in a cycle the requester picks when the cards are written: cycle
  14 or cycle 15.
- MOB-12, "Too many reset requests", is in scope and is rewritten with the text below.
- MOB-9, "Biometric sign-in", is out of scope: signing in with a fingerprint after a
  reset stays its work.
- MOB-7, "Sign in with email", is done.
- "Set a new password" and "Too many reset requests" are both blocked by "Request a reset
  link".
- How long a link stays valid is still undecided, an open question for the security
  team, not for these cards.

## New card: Request a reset link

### Context

The first step of the password reset in FEATURE.md: a person who forgot the password
asks for a link by email.

### User story

As a user who forgot my password, I want to ask for a reset link, so that I can get back
into my account without contacting support.

### Acceptance criteria

- As a user, when I tap "Forgot password?" on the sign-in page, I see a page asking for
  my email address.
- As a user, when I send an address, I see "If an account exists for this address, we
  sent you a link", whether or not the account exists.
- As a user with an account, when I send my address, I get an email with a link that
  opens the app.

### Design

The "Password reset" page of the app's design file.

## New card: Set a new password

### Context

The second step of the password reset in FEATURE.md, after
[Request a reset link](the link to that card, once it exists). Signing in with a
fingerprint afterwards stays with Biometric sign-in.

### User story

As a user who forgot my password, I want to set a new one from the link I received, so
that I can sign in again.

### Acceptance criteria

- As a user, when I open the link from the email, I see the "New password" page in the
  app.
- As a user, when I enter a password shorter than 12 characters or equal to my current
  one, I see which rule it breaks, and it is not saved.
- As a user, when I save a valid new password, I am signed in on this device, and every
  other device signed in to my account is signed out.

### Design

The "Password reset" page of the app's design file.

## Rewrite of MOB-12: Too many reset requests

### Context

A limit on the password reset in FEATURE.md, after
[Request a reset link](the link to that card, once it exists), so that nobody floods an
address with emails.

### User story

As a user, I want reset links to be limited, so that nobody can flood my inbox with
them.

### Acceptance criteria

- As a user, when I ask for a fourth link for the same address within an hour, I see
  "Too many requests, try again later", and no email is sent.

### Design

No design.
