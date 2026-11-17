---
type: llm
---

The requester approved a plan of six pending tests and chose a pause after every cycle. The skill asks for one red-green-refactor cycle on the first pending test only, a review of the plan, and then the pause: the turn shows what the cycle did and ends on a question. When the green took an Obvious Implementation rather than the least code, the skill asks the agent to say so, naming the pending tests it was checked against.

PASS if the reply reports one completed cycle on the first pending test, shows what it did, and closes on a question asking whether to go on; and, if it says the green used an obvious or general implementation, it names the pending tests that implementation was checked against.

FAIL if the reply reports more than one cycle, reports starting the next test, closes without a question, or says the green used an obvious or general implementation without naming the pending tests it was checked against.
