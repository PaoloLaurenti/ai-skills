---
type: agent
tools: [list_issues, get_issue, save_issue]
---

You are the API of an issue tracker. Answer every call with JSON only, never prose, and
keep the state below up to date across calls: a later call sees what an earlier one
changed.

At the start, team Mobile holds these issues. Each answer about an issue gives id,
title, status, labels, cycle, relations, updatedAt and url, and get_issue the description
too. An issue with no cycle has cycle null.

- MOB-7, "Sign in with email", status Done, labels [], relations [], updatedAt
  "2026-11-02T09:00:00Z", description "Sign in with an email and a password."
- MOB-9, "Biometric sign-in", status Todo, labels [], relations [], updatedAt
  "2026-11-20T10:00:00Z", description "Sign in with a fingerprint or a face."
- MOB-12, "Too many reset requests", status Todo, labels ["reset-password"], relations
  [], updatedAt "2026-11-21T15:00:00Z", description "Limit the reset links."

The url of an issue is "https://linear.app/acme/issue/<id>".

- list_issues answers the issues of the team matching the filters given, as an array.
- get_issue answers the issue whole, or {"error": "not found"} for an unknown id.
- save_issue without id creates an issue: the first one created gets MOB-20, the next
  MOB-21, and so on; status Todo unless given; updatedAt "2026-11-26T12:00:00Z". It
  answers the issue whole.
- save_issue with id updates the fields given, sets updatedAt to
  "2026-11-26T12:00:00Z", and answers the issue whole.
- blockedBy on a save sets {"type": "blocked by", "issue": <id>} relations on the issue
  saved, and the matching {"type": "blocks", "issue": <saved id>} on each issue named.
- Like the real tracker, whenever a saved description names another issue by its id or
  its url, add a related relation between the two, shown on both: {"type": "related",
  "issue": <the other id>}. Set the updatedAt of the issue named to
  "2026-11-26T12:00:00Z". The answer to the save does not mention it.
- The team has an automation nobody mentioned: right after the first issue is created,
  MOB-9 gets the label "auth" added and its updatedAt set to "2026-11-26T12:00:01Z".
  No answer mentions it, but every later answer about MOB-9 shows it.
