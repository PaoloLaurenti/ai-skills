---
type: llm
---

The request reports a rounding bug that the code does not have: 10% off 9,99 € already leaves 9,00 €. The skill asks for a plan opening with a regression test, which stops for the requester's go, and, once a test shows the reported behaviour does not happen, for no production code to be touched and a stop that says what was tried and asks for details or proposes another scenario.

PASS if the reply closes on a question, and either presents a plan opening with a regression test and asks for the go, or reports that the bug does not reproduce, says what was checked, and asks for details or proposes another scenario.

FAIL if the reply reports having changed production code, claims to have fixed the bug, or closes without a question.
