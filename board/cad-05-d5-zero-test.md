---
title: "M2 D5 dynamic evaluation: zero-test with gcd splitting + tower rewriting"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

The algorithmic heart of the whole design (DESIGN.md §5) and its fiddliest
piece. Zero-test for f at level k with squarefree defining polynomial p:

1. g = gcd(f, p) in (level k−1)[x] with Bézout data (cad-02, instantiated at
   the tower coefficient field).
2. g constant → f invertible mod p → f(α) ≠ 0. The Bézout identity
   u·f + v·p ≡ 1 is the certificate.
3. g nonconstant → p splits as g·(p/g); decide which factor owns the root α
   by interval refinement (endpoint signs = level-(k−1) sign queries — but
   NOTE: sign queries are M3! see the layering note below), then REPLACE the
   level's defining polynomial with the owning factor. If α owns g, f(α) = 0
   exactly.

**Layering note (important design decision):** the split-decision in step 3
needs signs, which don't exist until M3. Two ways to stage it:
  (a) Deliver the zero-test CONDITIONALLY: the checker-facing API takes the
      certificate's claim of which factor owns α as INPUT (the untrusted
      solver says "α owns g"), and the verified statement is conditional on
      level-wf (which, once M3 lands, includes the interval isolating a root
      of exactly that factor). This keeps M2 unblocked and matches the
      certificate philosophy — the checker re-derives everything checkable
      now and the wf-condition carries the rest.
  (b) Block on M3. Not recommended; M4 (equality checker) wants this card.
  Go with (a); revisit the interface when cad-09 lands.

**The recursion pit:** gcd over level-(k−1) coefficients needs leading-
coefficient invertibility tests = level-(k−1) zero-tests = recursion down the
tower, and any of those may themselves split lower levels, which REWRITES the
tower under the computation above it. Design the tower-rewriting story
carefully: a split produces a new Tower (same levels except level j's
defining poly shrank) + a coherence lemma (every wf element of the old tower
is wf in the new one; eqv/ops commute with the reinterpretation). Consider
making the zero-test return `(verdict, new_tower, coherence-facts)`
explicitly. Termination: lexicographic (tower height, then sum/multiset of
defining-poly degrees — every split strictly shrinks one degree).

**Done when:** zero-test verified: verdict=nonzero comes with Bézout
invertibility data; verdict=zero comes with explicit divisibility (g | f via
the divmod remainder ≡ 0 on the concrete polys — checkable identity, no
uniqueness theorem needed); tower rewriting + coherence proven; 0 errors;
committed. Smoke test: detect that (√2)² − 2 ≡ 0 and that √2 − 1 ≠ 0 in the
ℚ(√2) tower.

**Blocked by:** cad-02, cad-04.

## Progress

## Writeup
