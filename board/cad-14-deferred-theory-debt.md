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
  argument. The deepest of the four; BPR ch. 10 / any algebra text.
  v0.2 resolution of the open question: cad-08/09 do NOT need it — the wf
  route uses gcd(p, p′)-constant + Bézout-at-α directly (p′(α) ≠ 0 without
  any simple-roots theorem), so this stays parked for real.
- **Sturm–Tarski machinery** (moved here from the v0.1 critical path):
  signed remainder sequences, variation counts, Sturm's theorem against
  the model, TaQ. Optional future value: a solver-independent wf checker
  (no certificate fuel needed) and exact root counting. Revive only if the
  enclosure route (cad-08) walls, or as polish.

**Done when:** whichever pieces get built are verified and the rest are
explicitly re-parked with a reason.

**Blocked by:** cad-01 (all items), cad-08 (to know what's actually needed).

## Progress

## Writeup
