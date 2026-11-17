---
type: llm
---

The request asked for a behaviour change without naming TDD, so the reply has to stop and ask whether to proceed with TDD before anything is written.

PASS if the reply closes on a single question asking whether to proceed with TDD, and asks nothing about pausing after every cycle or after each step.

FAIL if the reply asks no question, asks a question other than whether to proceed with TDD, asks about pausing after every cycle, asks several questions at once, or reports having started the work.
