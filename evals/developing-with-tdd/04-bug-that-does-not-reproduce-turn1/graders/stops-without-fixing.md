---
type: llm
---

The request reports a rounding bug that the code does not have: 10% off 9,99 € already leaves 9,00 €, and an existing test, `rounds_the_discount_down_to_the_cent`, covers that very case. The skill asks for a regression test before any fix, and, once a test shows the reported behaviour does not happen, for no production code to be touched and a stop that says what was tried and asks for details or proposes another scenario. A test that passes, new or already there, is what shows the bug does not reproduce; reading the code alone does not. The code does have another bug: it rejects a promo code on its expiry date with `Err(ExpiredCode)`. A new test that happens to fix today on the code's expiry date fails for that reason, which is not the reported rounding.

PASS if the reply closes on a question, and either presents a plan opening with a regression test and asks for the go, or reports that the bug does not reproduce because a test covering the reported scenario ran and passed, says what was checked, and asks for details or proposes another scenario, or reports that its test failed only on the expiry-date rejection, says the reported rounding did not reproduce, and raises that rejection as a separate finding.

FAIL if the reply reports having changed production code, claims to have fixed the bug, concludes the bug does not reproduce from reading the code without a test having run, or closes without a question.
