---
title: "M3c OrderedField instance for towers (the mountain summit)"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

Assemble the ordering (DESIGN.md §6): sign of a tower element as a total
spec function by structural recursion — at level k, sign(f) = TaQ(f, p_k;
lo_k, hi_k) (cad-08), whose own computation does arithmetic and sign tests
at level k−1, bottoming out at Rational.

Then the correctness obligations, by induction on tower height, each level
interpreted through the model (cad-07):

- sign is well-defined on quotient-eqv classes (cad-06's eqv);
- trichotomy: sign(f) = 0 iff zero-test(f) = zero (D5, cad-05) — this is
  the lemma that welds the algebraic and order-theoretic halves together;
- sign(f·g) = sign(f)·sign(g); sign compatibility with addition;
- `le(f, g) := sign(g − f) >= 0` gives the `OrderedRing`/`OrderedField`
  axioms (le_total, le_antisymmetric via trichotomy, monotonicity).

Also close the loop left open by cad-05's staging decision: level-wf now
includes "the interval isolates exactly one root of the defining poly"
(checkable, cad-08 item 5), and the D5 split-decision (which factor owns α)
becomes verified-checkable instead of certificate-supplied — upgrade the
zero-test interface accordingly.

The dts precedent says this genre (ordering axioms for towers) was 23k lines
at degree 2 under Z3. The bet of this whole program is that
Sturm-as-total-function + Cauchy model + Lean-backend induction lands it in
a fraction of that. Track the actual cost honestly in the writeup — it's
data about the bet.

**Done when:** `OrderedField` (trait instance or equivalent lemma kit) for wf
tower elements, 0 errors, committed. Smoke: √2 > 1, √2 < 3/2 + 1/10, and
sign((√2)² − 2) = 0 all decided by the verified sign function on the ℚ(√2)
tower.

**Blocked by:** cad-06, cad-07, cad-08.

## Progress

## Writeup
