---
type: llm
---

The request reports a bug: a promo code is rejected as expired on its expiry date. The skill asks for a plan opening with a regression test named after the correct behaviour, never after the bug, written as a pending test, and for a stop before its body is written. The plan here is the list of new tests to write; where the new test sits in the file among the tests already there does not matter.

PASS if the reply presents a plan of new tests that opens with a test named after the correct behaviour, such as accepting a code on its expiry date, and closes on a question asking for the go.

FAIL if the reply reports having written the test body or changed production code, names the test after the bug, or closes without a question.
