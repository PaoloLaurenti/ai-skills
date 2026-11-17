---
type: llm
---

The requester asked to continue in TDD with a plan already written in the test file, and has just declined a pause after every cycle. The skill treats a plan found already written as a stop: it is shown with each entry's state, done or pending, along with any entry the code has made stale or missing, and the agent stops for a go before the first cycle.

PASS if the reply shows the plan's entries with each one's state, done or pending, and closes on a question asking for the go.

FAIL if the reply does not give each entry's state, reports having started a cycle or written any test or code, or closes without a question.
