---
title: "M3a mini constructive Cauchy-real model over ℚ"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

The semantic anchor for ordering (DESIGN.md §6, route M3a). NOT a full real
analysis library — just enough to give each tower an evaluation map into
"honest numbers" so the Sturm machinery's correctness axioms (cad-09) have a
model. Independent of everything else on the board — can start anytime, good
parallel track.

**v0.2 clarification — the model is CLASSICAL.** Verus spec logic is
classical (and the Lean backend has Classical available), so nothing here
needs to be constructive/decidable: `pos`/`le` on CReal are exists-witness
predicates, trichotomy is a classical theorem, and the tower root is a
spec-level `choose`. "Cauchy sequences" were chosen for *definability* (no
built-in real type), not constructivism. This substantially softens v0.1's
worries about constructive IVT. What the model must additionally provide
for the v0.2 ordering route (DESIGN §6): **monotone-uniqueness** (a
polynomial whose derivative has constant sign on an interval has at most
one root there — needs a mean-value-flavored argument or a direct
monotonicity-from-derivative-sign lemma for polynomials; plan the proof
route early, it's the one real analysis lemma in the program) and the
**derivative** of a polynomial as the algebraic pderiv (cad-03) linked to
a difference-quotient bound (polynomial identity: p(x) − p(y) =
(x − y)·q(x, y) with q explicit — provable by induction, no limits needed;
monotonicity can be routed through this identity entirely, avoiding
analysis: sign-constant q on the box ⟹ injective. Prefer this route).

**Trigger risk to plan around:** CReal as a struct with a
`spec_fn(nat) -> Rational` field runs into the workspace's known
spec_fn-in-quantifier trigger pitfalls. Mitigation: wrap sequence access
in a named spec fn (`cr_at(x, n)`) and quantify over that; or represent
sequences as recursive spec fns where natural.

Contents (probably its own module tree in tactus-algebra or a new
tactus-reals crate — decide at start; leaning tactus-algebra module to avoid
another export hop):

1. `CReal` = sequence of Rationals with an explicit modulus of convergence
   (spec_fn from nat, or a struct with a `seq: spec_fn(nat) -> Rational` +
   modulus field; note the spec_fn-in-quantifier trigger pitfalls — prefer
   recursive spec fns or struct fields per the workspace idioms).
2. Arithmetic: add, neg, mul (needs a bound extraction for mul's modulus),
   with congruence w.r.t. CReal-eqv (pointwise-limit equality: |a_n − b_n|
   → 0).
3. Order with witness: `pos(x)` = exists rational ε > 0 and N with x_n ≥ ε
   for n ≥ N. `le`, trichotomy is NOT decidable here and doesn't need to be
   — the algebraic side (Sturm) supplies decisions; the model only
   interprets them.
4. Ring/OrderedRing-style lemma kit for CReal (comm/assoc/distrib/
   monotonicity) — pointwise from Rational's OrderedField instance, plus
   limit bookkeeping.
5. Polynomial evaluation `peval: Seq<CReal> × CReal → CReal` (or Rational
   coefficients evaluated at a CReal point — that's the case actually
   needed) + continuity-flavored lemma: evaluation commutes with the
   arithmetic (homomorphism lemmas), and the bisection-limit lemma:
6. **Root existence by decidable bisection**: given p over a sign-decidable
   coefficient field with p(lo) < 0 < p(hi) (or the mirrored signs),
   the bisection sequence is Cauchy and its limit α satisfies p(α) ≡ 0.
   Constructively fine here because test points are rational and signs at
   rational points are decidable (by induction up the tower); a zero at a
   midpoint terminates with an exact root.

**Done when:** the six pieces verified, 0 errors, committed. Gate lemma to
aim at: "every wf Level (sign change + isolating certificate) has a CReal
root in its interval, and evaluation at it is a ring homomorphism" — stated
for Rational coefficients first (depth-1 towers); the general-depth version
composes in cad-09.

**Blocked by:** nothing (parallel track). Grows teeth when cad-08 lands.

## Progress

## Writeup
