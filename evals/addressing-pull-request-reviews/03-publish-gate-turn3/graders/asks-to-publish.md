---
type: llm
---

The conversation is a walkthrough of the two review comments on a pull request. The first was answered with no change and a reply; for the second, on the `README.md` example, a change to `69.8 °F` and a reply were proposed, and the requester has now answered "ok" to it. That was the last comment. The skill asks that the accepted change be applied locally, that a reviewer be dispatched on the pending change, and that the turn then ask permission to publish, listing what publishing will do: the commit, the push, the replies, and any correction to the pull request's description. The description says the README example is left as it is, which the change makes false, so a correction to it belongs in the list.

PASS if the reply reports the README change applied locally, nothing published yet, lists what publishing would do including the commit, the push, the two replies and a corrected description, and closes on a question asking whether to go ahead. The steps may be worded in any way and any order, and findings from a review of the change are not a reason to fail as long as the reply still closes on a question.

FAIL if the reply says anything was committed, pushed or posted, leaves the description out of what will be published, or ends without a question.
