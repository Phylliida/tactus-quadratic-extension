---
title: "M1 tower core: Level/Tower/TowerElem types, quotient-ring ops, Ring instance"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

The number type itself, in THIS crate (tactus-quadratic-extension). See
DESIGN.md §4 for the representation sketch:

- `Level` = defining polynomial (coefficients one level down) + rational
  isolating interval (lo, hi). `Tower` = Seq of levels. `TowerElem` =
  recursive enum `Base(Rational) | Poly(Seq<TowerElem>)` — the DynTowerSpec
  precedent in verus-quadratic-extension used exactly this recursive-enum
  shape and it worked.
- Well-formedness predicates: `elem_wf(tower, k, e)` — element e lives at
  depth ≤ k, each Poly node's coefficient count < deg(defining at its
  level), coefficients wf one level down. Level-wf (monic, squarefree,
  interval isolating) is NOT this card — the sign machinery it needs is
  cad-08; for now levels carry the defining poly and the interval as data
  with only structural checks.
- Quotient-ring ops at each level: add/neg/sub are coefficient-wise
  (recursing into the level below); mul is poly-mul followed by reduction
  mod the defining polynomial — reduction = divmod's remainder, which is
  where cad-00's divmod gets consumed. The subtlety: tactus-algebra's poly
  layer is generic over `T: Ring/Field`, and level-k elements form a ring
  where the "scalars" are level-(k−1) elements — so either (a) implement
  the trait ladder for TowerElem-at-depth-k via a wrapper type and
  instantiate the generic poly layer, or (b) re-specialize the needed poly
  ops for TowerElem inline. (a) is cleaner if Verus's trait system
  cooperates with the depth indexing (a wf-carrying wrapper struct à la the
  `WellFormedDTS<W>` idea from the old crate); expect this to be the card's
  main design decision. Division at level k needs the level-(k−1)
  coefficient field to be a *decidable* field (inverses via cad-02 Bézout +
  zero-test via cad-05) — so full Field is M2; this card delivers Ring.
- `Ring` instance (or the equivalent lemma kit) for wf tower elements, by
  induction on tower height: eqv (coefficient-wise, mod nothing — two
  representatives are eqv iff their difference reduces to zero — for the
  Ring layer, use representative-wise peqv-style eqv; the *semantic*
  quotient eqv needs the zero-test and lands with cad-05).

**Done when:** types + wf + add/neg/sub/mul-with-reduction verified with Ring
axioms (commutativity, associativity, distributivity, congruence) at every
tower depth by induction; `./check.sh` 0 errors; committed. A concrete smoke
instance (ℚ(√2) as a depth-1 tower, i.e. defining poly x²−2 over Rational)
with a few computed identities ((1+√2)² ≡ 3+2√2) as executable-ghost sanity.

**Blocked by:** cad-01 (ring laws feed the induction). cad-02 not needed yet.

## Progress

## Writeup
