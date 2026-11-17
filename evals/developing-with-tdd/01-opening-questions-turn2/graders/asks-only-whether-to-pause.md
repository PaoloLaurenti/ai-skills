---
type: llm
---

The conversation so far: the request asked for a behaviour change without naming TDD, the agent asked whether to proceed with TDD, and the requester has now answered yes. The skill asks next, in a turn of its own and before any work starts, whether to pause after every cycle.

PASS if the reply closes on a single question asking whether to pause after every red-green-refactor cycle.

FAIL if the reply asks no question, asks again whether to use TDD, asks a question other than whether to pause after every cycle, asks several questions at once, or reports having written a plan, a test or any code.
