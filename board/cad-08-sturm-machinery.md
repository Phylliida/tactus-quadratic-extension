---
title: "M3b interval-enclosure machinery + wf-by-monotonicity (Sturm demoted, v0.2)"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T23:10:00Z
---

## Description

**v0.2 rewrite** (was: Sturm machinery — see DESIGN §6 and the fallback note
below). The checker-side computation layer for signs and level
well-formedness, with only soundness-direction proofs:

1. **peval**: polynomial evaluation at a point, in tactus-algebra's poly
   layer (~20 lines + congruence + homomorphism lemmas: peval of padd/pmul
   is add/mul of pevals — the mul case is the one real induction, reusing
   the push-decomposition toolkit).
2. **Rational interval type** (lo, hi as Rational, lo ≤ hi) + interval
   add/neg/mul/scale with the containment soundness lemmas (model value in
   → result in). Port shapes from `verus-interval-arithmetic` where useful;
   the soundness lemmas are small over the Rational OrderedField instance.
3. **Interval Horner**: enclosure of p(x) from an enclosure of x and
   enclosures of p's coefficients; soundness by induction via (2).
4. **Recursive enclosure of tower elements**: level k's α is enclosed by its
   interval; a level-k element (poly in α with level-(k−1) coefficients)
   is enclosed via (3) with coefficient enclosures by recursion. Refinement:
   bisect α's interval, decide the surviving half by the sign of p at the
   rational midpoint — that sign is a level-(k−1) element's sign, which the
   checker gets by *this same machinery one level down* (plus D5 for exact
   zeros: p(mid) ≡ 0 means the midpoint is the root exactly — handle that
   branch). **Fuel for all refinement comes from the certificate** — no
   termination proof anywhere, just "run d steps, check the enclosure
   excludes 0".
   Soundness invariant (proven against the cad-07 model): the model root
   stays inside the maintained interval through every bisection step.
5. **Level wf checking** (DESIGN §4.2 v0.2): monic; squarefree = gcd(p, p′)
   constant with Bézout data (cad-02/03); endpoint sign change (endpoint
   values are level-(k−1) elements — recurse); **monotonicity certificate**:
   enclosure of p′ over the whole interval (interval-in-interval Horner)
   excludes 0. Soundness target: wf ⟹ the model has exactly one root in the
   interval (cad-07's IVT + monotone-uniqueness).

**Done when:** items 1–5 verified (soundness direction only), 0 errors,
committed. Smoke: certify wf of the ℚ(√2) level (x²−2 over (1, 2), p′ = 2x
enclosed positive) and compute sign(√2 − 7/5) = + with explicit fuel.

**Sturm fallback (parked):** if this route hits a wall, v0.1's plan was
sign-as-Sturm–Tarski-query (signed remainder sequences over divmod+trim,
variation counts, Sturm's theorem proven against the model; Wenda Li's
Isabelle development and BPR ch. 2 as guides; the degree-2 query reproduces
the dts case analysis). Also a fine *later* addition: it gives a
solver-independent wf checker (no fuel needed). Tracked in cad-14.

**Blocked by:** cad-02 (gcd for squarefree checks), cad-04 (tower types).
cad-07 for the soundness statements (can build computation first, prove
against the model as cad-07 lands).

## Progress

## Writeup
