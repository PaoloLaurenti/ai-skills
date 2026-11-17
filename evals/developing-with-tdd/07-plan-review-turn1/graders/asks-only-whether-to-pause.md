---
type: llm
---

The request names TDD, which answers the skill's first opening question, whether to proceed with TDD. What remains is the second one, whether to pause after every cycle, asked in a turn of its own before any work starts.

PASS if the reply closes on a single question asking whether to pause after every cycle, and does not ask whether to use TDD.

FAIL if the reply asks no question, asks whether to proceed with TDD, asks a question other than whether to pause after every cycle, asks several questions at once, or reports having started a cycle.
