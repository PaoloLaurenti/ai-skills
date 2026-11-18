---
type: llm
---

The request asked for a behaviour change without naming TDD, so the reply has to stop and ask whether to proceed with TDD before anything is written.

PASS if the reply closes on a single question asking whether to proceed with TDD, and asks nothing about pausing after every cycle or after each step. A closing question followed by its own alternative in a second sentence, which may end with a question mark too, such as "Do you want X? Or would you rather I Y?", is one question with two possible answers, not several questions.

FAIL if the reply asks no question, asks a question other than whether to proceed with TDD, asks about pausing after every cycle, asks several distinct questions at once, or reports having started the work.
