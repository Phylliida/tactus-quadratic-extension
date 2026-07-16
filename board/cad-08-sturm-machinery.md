---
title: "M3b Sturm machinery: signed remainder sequences, variation counts, Sturm–Tarski"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

The algebraic half of the ordering mountain (DESIGN.md §6). Over a generic
ordered coefficient field (sign-decidable — instantiated at Rational first,
then at towers-with-signs as cad-09 climbs):

1. **Signed remainder sequence** SRemS(p, q): p, q, −rem(p, q), ... (divmod
   exists; needs trim between steps; termination by degree).
2. **Sign variation count** Var(S; x): signs of the sequence evaluated at a
   rational point, zeros dropped, count sign changes. (Polynomial evaluation
   at a point: add `peval` to the tactus-algebra poly layer if cad-07
   hasn't already — it's ~20 lines with a congruence lemma.)
3. **Sturm's theorem**: for squarefree p, Var(SRemS(p, p′); lo) −
   Var(SRemS(p, p′); hi) = number of roots of p in (lo, hi]. This is the
   biggest single proof in the program. Textbook route (BPR ch. 2):
   induction on the interval refinement / root-crossing analysis — in OUR
   setting, prove it AGAINST THE CAUCHY MODEL (cad-07): "number of roots"
   means the model's roots, sign behavior near a root comes from the model's
   continuity lemmas. Do not attempt a model-free syntactic proof.
4. **Sturm–Tarski / Tarski query**: TaQ(f, p; lo, hi) via SRemS(p, p′·f)
   computes Σ_{roots α of p in (lo,hi)} sign(f(α)). With the exactly-one-root
   wf condition, this IS sign(f(α)) — the total, computable spec-level sign
   function the whole design rests on.
5. **Level well-formedness becomes checkable**: monic + squarefree
   (gcd(p, p′) constant, cad-02/03) + Var-difference = 1 (exactly one root
   in the interval).

Sizing honestly: this is the card most likely to take multiple sessions and
to spawn sub-cards (root-counting lemmas, sign-of-polynomial-near-endpoint
lemmas, multiplicity handling). Prior art to consult for proof structure (not
code): Wenda Li's Isabelle Sturm development, Cyril Cohen's math-comp real
closure, BPR ch. 2. The degree-2 sanity check: the Sturm query at x²−d over
(0, d+1) should reproduce the dts closed-form case analysis.

**Done when:** items 1–5 verified over Rational coefficients, 0 errors,
committed. (Generic-over-tower instantiation is cad-09's job.)

**Blocked by:** cad-02 (SRemS needs divmod+trim — already available — and
benefits from cad-01/02 being settled). cad-07 strongly recommended first or
concurrent (item 3 proves against the model).

## Progress

## Writeup
