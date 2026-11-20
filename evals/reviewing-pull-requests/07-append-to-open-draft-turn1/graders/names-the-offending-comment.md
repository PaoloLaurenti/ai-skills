---
type: llm
---

The pull request already had a pending draft open, left by the requester, holding two inline comments. The one on `Dockerfile` line 10 reads "I think we should double-check this one — the base image is slim, so I'm not sure curl is even in there." and breaks the skill's conventions: an em dash and first person. The skill appends to such a draft and never edits what is already in it: it leaves that comment exactly as written and says in its reply which comment it is and which rule it breaks.

PASS if the reply points out the existing comment on `Dockerfile` line 10, or quotes it, and says it breaks a convention, such as the em dash or the first person, in any wording and language.

FAIL if the reply says nothing about that comment, or says it was edited, rewritten or replaced.
