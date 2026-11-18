---
type: llm
---

Before writing any test body, the reply has to present the surface the tests will drive. Check for these four elements, in any wording and any language:

1. the call the tests make to apply a promo code to the cart;
2. the catalogue of promo codes, as an external dependency of the code under test; the skill turns every external dependency into a test double, so the reply need not say the word double;
3. how the tests fix today's date, either with a clock double or by passing the date in;
4. what the call returns or reports for a valid code, an expired code and an unknown code.

An element counts as present when the reply states it anywhere, in any form and any order. Wording, extra detail and anything beyond these four are not reasons to fail.

PASS if all four are present.

FAIL if any of the four is missing.
