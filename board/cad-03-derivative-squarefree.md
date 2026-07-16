---
title: "derivative + squarefree-part operations"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

Small card closing out M0. In tactus-algebra:

1. **nat-scalar multiple**: `nat_mul(n, x)` = x + x + ... (n times), recursive
   spec fn + congruence lemma. (Needed because derivative coefficients are
   `(i+1) · p[i+1]` and T has no ℕ-action built in.)
2. **derivative**: `pderiv(p) = Seq::new(p.len() - 1, |i| nat_mul(i + 1,
   p[i + 1]))` (empty for constant/empty p). Congruence lemma
   (peqv(p, q) → peqv(pderiv(p), pderiv(q)) — careful: this is FALSE for
   length-agnostic peqv only in appearance; trailing eqv-zero coefficients
   contribute nat_mul(k, ε) ≡ 0 coefficients, so it holds up to peqv — the
   proof needs nat_mul congruence + nat_mul(k, 0) ≡ 0).
3. **squarefree-part operation**: `sqfree_part(p)` := first component of
   `divmod(p, gcd(p, pderiv(p)))` (suitably trimmed). Plus the *checkable
   predicate* the D5 layer will actually use:
   `is_squarefree_witness(p) := trim(gcd(p, pderiv(p))).len() <= 1` — gcd
   with the derivative is a (nonzero) constant.

Explicitly NOT in scope (cad-14): the theorem that sqfree_part(p) is
squarefree, or that exact division holds (remainder of p by the gcd is
eqv-zero). The certificate checker never needs them: it checks concrete
identities (r ≡ 0, products re-multiply) on the actual certificate polys.

**Done when:** definitions + congruence lemmas verified, 0 errors, committed;
DESIGN.md M0 status note updated to "M0 complete".

**Blocked by:** cad-02.

## Progress

## Writeup
