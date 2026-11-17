---
type: llm
---

Before writing any test body, the reply has to present the surface the tests will drive. Check for these four elements, in any wording and any language:

1. the call the tests make to apply a promo code to the cart;
2. the catalogue of promo codes, as a dependency that the tests replace with a double;
3. how the tests fix today's date, either with a clock double or by passing the date in;
4. what the call returns or reports for a valid code, an expired code and an unknown code.

PASS if all four are present.

FAIL if any of the four is missing, naming the missing one.
