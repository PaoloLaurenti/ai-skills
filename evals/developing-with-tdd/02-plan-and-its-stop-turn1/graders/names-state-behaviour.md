---
type: llm
---

The reply lists the planned tests by name.

PASS if every planned test name states a behaviour in domain words, such as applying the discount, rejecting an expired code, or an empty cart totalling zero, and no name holds a type or method name, or a specific number, percentage, amount, promo code or date. Words that describe the outcome of a case, such as "zero" for an empty cart or "whole cent" for rounding, are domain words, not concrete values.

FAIL if the reply lists no test names, or if any name holds a type or method name, a specific number, percentage, amount, promo code or date, or describes a mechanism rather than a behaviour.
