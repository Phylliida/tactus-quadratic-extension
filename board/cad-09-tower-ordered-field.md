---
title: "M3c model-welded ordering: OrderedField laws for towers (the summit, v0.2)"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T23:10:00Z
---

## Description

**v0.2 rewrite**: the order is *defined by the model*, not computed by the
spec (DESIGN §6). Assemble:

1. **Evaluation homomorphism**: `eval(tower, e) → CReal` by structural
   recursion — each level's α is the spec-level chosen root
   (`choose|x| p(x) = 0 && lo < x < hi`, existence from cad-07's IVT given
   cad-08's wf). Homomorphism lemmas: eval commutes with tadd/tmul/tneg
   (uses cad-07's CReal ring kit + peval homomorphism from cad-08).
2. **The spec order**: `tsign(tower, f) := sign of eval(tower, f)` (classical
   CReal sign — noncomputable spec, deliberately). `tle(f, g) :=
   tsign(g − f) ≥ 0`.
3. **Welding lemmas** (D5 ⟺ model):
   - nonzero side: Bézout `u·f + v·p ≡ 1` (peqv, from cad-05's verdict)
     evaluated at α — since p(α) = 0, get u(α)·f(α) = 1, so f(α) ≠ 0;
   - zero side: `f ≡ w·g` with α rooting g gives f(α) = 0.
   Corollary: D5-eqv (cad-06) coincides with model equality — trichotomy
   welds, and `tsign` is well-defined on quotient classes.
4. **Order laws**: the OrderedRing/OrderedField axiom set (le_total,
   antisymmetry-up-to-eqv, transitivity, add/mul monotonicity, congruence)
   — each inherited from the corresponding CReal lemma through the
   homomorphism. In the **relative form** per DESIGN §9.1 (η): these are
   lemmas parameterized by (tower, wf), not a trait instance.
5. **Checker-sign soundness**: cad-08's fueled enclosure sign, when it
   reports a sign, equals tsign (the enclosure contains eval(f); exclusion
   of 0 decides). This is the lemma cad-12 consumes.
6. Close cad-05's staged split-decision: the certificate's "α owns factor g"
   claim becomes checkable (endpoint signs of g via cad-08) and the D5
   zero-test's conditional wf assumptions discharge.

The dts precedent says this genre was 23k lines at degree 2 under Z3. The
program's bet is that model-definition + welding + Lean-backend induction
lands it in a small fraction. Track actual cost honestly in the writeup —
it's the data on the bet.

**Done when:** 1–6 verified, 0 errors, committed. Smoke: √2 > 1,
√2 < 3/2 + 1/10, sign((√2)² − 2) = 0, all through the verified pipeline on
the ℚ(√2) tower.

**Blocked by:** cad-06, cad-07, cad-08.

## Progress

## Writeup
