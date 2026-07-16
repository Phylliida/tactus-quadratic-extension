---
title: "M2 tower field ops: inverses via Bézout, division, Field-modulo-ordering"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

With the D5 zero-test in hand, finish the field structure on tower elements:

- `recip` for elements testing nonzero: the Bézout data from the zero-test's
  gcd IS the inverse (u·f + v·p ≡ 1 means u·f ≡ 1 mod p, so u is f⁻¹). Wire
  it through and prove `f · recip(f) ≡ 1` at the quotient level.
- `div` = mul by recip; congruence lemmas.
- Quotient-level eqv done properly: f ≡ g iff zero-test(f − g) = zero. Show
  it's an equivalence relation compatible with the ops (this is where the
  representative-wise eqv from cad-04 gets upgraded to the true quotient
  equivalence).
- Field trait instance for the wf-carrying wrapper (or the equivalent lemma
  kit if the trait route fought back in cad-04) — everything EXCEPT the
  ordering (OrderedField waits for cad-09).

**Done when:** verified inverses + quotient eqv, 0 errors, committed. Smoke:
1/(1+√2) ≡ √2 − 1 in the ℚ(√2) tower.

**Blocked by:** cad-05.

## Progress

## Writeup
