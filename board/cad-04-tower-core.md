---
title: "M1 tower core: TowerElem free-ring instance, monic reduction, Tower types"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T23:10:00Z
---

## Description

**v0.2 rewrite** after working the relative-ring crux (DESIGN §9.1). The
crux: trait methods are absolute (`zero()` static, reduction needs the
tower), the tower is runtime data, so a *quotient* OrderedField instance
for towers is impossible — it's why dts's Ring impls were removed. But two
discoveries make this card cleaner than v0.1 feared:

**(A) The free-ring instance is tower-free.** Define TowerElem =
`Base(Rational) | Poly(Seq<TowerElem>)` with structural ops and NO
reduction: add/neg/sub with cross-depth broadcasting (dts_eqv-style:
Base(q) ≈ Poly(c) iff head ≈ q and tail eqv-zero — dts proved
Equivalence + AdditiveGroup in exactly this shape at degree 2), and mul as
*formal* nested-poly multiplication. Unreduced products have degree ≥ deg(p)
— unrepresentable in dts's (re, im) shape, but our Seq shape holds them
fine. The Ring axioms hold *formally* (it's a free polynomial construction
— no tower, no wf, no same_radicand!), so **TowerElem gets a genuine
`Ring` trait instance**, and tactus-algebra's whole generic poly layer
(padd/pmul + the cad-00/cad-01 lemma stock) instantiates at
T = TowerElem for free. The beefy proof is mul associativity (nested
structural induction); dts's Equivalence/AdditiveGroup impls are the
warm-up precedent.

**(B) Reduction mod monic divisors needs only Ring.** Defining polynomials
are monic by wf, and monic divmod needs no `recip` (the division factor is
just the leading coefficient). Add `divmod_monic<T: Ring>` to
tactus-algebra — a small variant of the existing divmod proof (factor
`f = a.last()`, kill lemma via `la·lc(b) ≡ la·1 ≡ la`). Then
**normalization** (reduce a TowerElem mod the tower's defining polys,
top-down) is Ring-only — no D5, no field ops, deliverable in this card.

So the card:

1. TowerElem + structural ops + cross-depth eqv; `Equivalence`,
   `AdditiveCommutativeMonoid`, `AdditiveGroup`, `Ring` instances (the
   free ring). Mul associativity is the mountain-let.
2. `divmod_monic<T: Ring>` in tactus-algebra.
3. `Level`/`Tower` types + structural wf (monic head coefficient,
   degree bounds, coefficient depths; interval-isolation wf is cad-08).
4. `normalize(tower, e)`: top-down monic reduction; ensures: degree bounds
   at every level, and `e ≈ normalize(e)` where ≈ is representative-wise
   "differs by an explicit multiple of the defining polys" (the witnesses
   come straight out of divmod_monic's quotients — certificate-shaped).
5. Quotient-ring lemma kit *relative to (tower, wf)*: the quotient ops are
   free-ops-then-normalize; congruence/assoc/comm/distrib w.r.t. ≈ follow
   from the free Ring instance + normalize's ensures. No quotient trait
   instance — the constraint layer meets towers via relativization
   (DESIGN §9.1 route η, at cad-11).

Note what this does NOT deliver: semantic zero-testing (normalize's
structural zero is sound but incomplete when a defining poly is reducible —
that gap is D5's whole job, cad-05) and any ordering (cad-08/09).

**Done when:** 1–5 verified, 0 errors, committed. Smoke: in the ℚ(√2) tower
(x²−2 over (1,2)), normalize((1+√2)²) is structurally 3 + 2√2.

**Blocked by:** cad-01 (the generic poly lemma stock it instantiates).

## Progress

- (2026-07-16T23:45Z) **Derisking probe PASSED** (`src/probe_tower.rs`,
  throwaway): (1) `ghost enum TowerElem { Base(Rational),
  Poly(Seq<TowerElem>) }` accepted; (2) recursive spec fn `te_eqv` with
  `decreases a` and the recursive call *under a forall* on `xs[i]` —
  container-decreases through Seq works; (3) nested-induction proof fn
  (`te_eqv_refl`, recursive call inside assert-forall) discharges under the
  Lean backend; (4) cross-crate generic instantiation of the poly layer at
  T = Rational works including `divmod` (with its requires). Only hiccup:
  concrete-literal eqv facts (3·1 ≠ 0·1) need explicit unfolding asserts.
  The card's core bets are confirmed; no Box-list fallback needed.

## Writeup
