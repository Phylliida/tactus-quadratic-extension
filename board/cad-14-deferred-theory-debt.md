---
title: "deferred theory debt: divmod uniqueness, exact-division, squarefree-part theorem, gcd divisibility"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

Parking card for theorems the certificate-checker path deliberately does NOT
need (the checker validates concrete identities instead), but which a
self-respecting algebra library eventually wants, and which M3's model work
may want anyway:

- **divmod uniqueness**: r₁ ≡ r₂ when both satisfy the division equation
  with deg < deg(b). Needs the degree-of-product analysis (leading
  coefficient of a product of nonzero-lc polys is the product of lcs — an
  integral-domain fact, fine over a field but requires a pmul top-coefficient
  formula our recursive definition doesn't directly expose; likely via the
  push-decomposition normal form).
- **exact division as a theorem**: g | p (with a wf gcd) ⟹ divmod(p, g).1 ≡
  zero-poly. Follows from uniqueness.
- **gcd divisibility**: Euclid's g divides both inputs, with witnesses
  threaded through the recursion (needs cad-01 laws; straightforward once
  uniqueness exists, or directly by witness-threading without uniqueness).
- **squarefree-part is squarefree**: gcd(sqf, sqf′) is constant, char-0
  argument. The deepest of the four; BPR ch. 10 / any algebra text. Needed
  only if/when M3's model work wants "squarefree ⟹ simple roots ⟹ sign
  changes isolate roots" — check cad-08's actual needs before investing;
  it may pull this card into the critical path, in which case split it out.

**Done when:** whichever pieces get built are verified and the rest are
explicitly re-parked with a reason; revisit after cad-08 clarifies what M3
consumes.

**Blocked by:** cad-01 (all items), cad-08 (to know what's actually needed).

## Progress

## Writeup
