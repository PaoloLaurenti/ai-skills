---
type: llm
---

The request names TDD, which answers the skill's first opening question, whether to proceed with TDD. What remains is the second one, whether to pause after every cycle, asked in a turn of its own before any work starts.

PASS if the reply closes on a single question asking whether to pause after every cycle, and does not ask whether to use TDD. A closing question followed by its own alternative in a second sentence, which may end with a question mark too, such as "Do you want X? Or would you rather I Y?", is one question with two possible answers, not several questions.

FAIL if the reply asks no question, asks whether to proceed with TDD, asks a question other than whether to pause after every cycle, asks several distinct questions at once, or reports having started a cycle.
