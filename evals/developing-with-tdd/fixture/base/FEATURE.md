# Promo codes

A customer can apply a promo code to the cart and pay a discounted total.

- A promo code takes a percentage off the cart total: `SPRING10` takes 10% off.
- The discount is rounded down to the whole cent.
- A code is valid up to and including its expiry date. A code past that date is
  rejected as expired.
- A code the shop does not know is rejected as unknown.

The codes come from a catalogue the shop already keeps elsewhere: the cart does not know
where it is stored, only that it can ask it for a code.
