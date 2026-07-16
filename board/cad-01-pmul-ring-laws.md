---
title: "pmul ring laws: commutativity, distributivity, associativity, full congruence"
status: done
claimed_by: fable
created: 2026-07-16T22:30:00Z
updated: 2026-07-17T00:05:00Z
---

## Description

Finish the polynomial multiplication law kit in `tactus-algebra/src/poly_mul.rs`
(or a new `poly_ring.rs`). divmod deliberately avoided these; **gcd/Bézout
(cad-02) cannot** — the Euclid invariant g ≡ v·a + (u − v·q)·b needs left-
distributivity over psub, pmul associativity, and second-argument congruence.

Proof strategy (all structural induction on the first argument, reusing
`lemma_pmul_push` and the shiftk helpers):

1. **Second-argument congruence**: peqv(q, q′) → peqv(pmul(p, q), pmul(p, q′)).
   Induction on p: scale-cong (new pointwise lemma) + shiftk-cong + padd-cong.
2. **Left-distributivity**: peqv(pmul(p, padd(q, r)), padd(pmul(p,q), pmul(p,r))).
   Induction on p: scale distributes over padd (pointwise via T
   distributivity), shiftk-padd, then a 4-term padd shuffle.
   Same statement for psub (or derive via pneg: pmul(p, pneg(q)) ≡
   pneg(pmul(p, q)), also by induction).
3. **Push-decomposition mirror** (worked out in detail 2026-07-16): prove
   `pmul(q, p.push(c)) ≡ padd(pmul(q, p), shiftk(scale(c, q), p.len()))`
   by induction on q, via two new helpers:
   - `lemma_push_as_padd`: peqv(x.push(v), padd(x, shiftk(seq![v],
     x.len()))) — pointwise, easy (i < len: x_i + 0; i = len: 0 + v).
     Turns every push into padd + monomial, so the whole card runs on
     padd/shiftk algebra.
   - `lemma_pmul_monomial_right`: pmul(q, shiftk(seq![v], k)) ≡
     shiftk(scale(v, q), k) — induction on q; NOTE the coefficient order
     flips (q_i·v vs scale's v·q_i), so **this is where T-commutativity
     enters the poly layer**. Also needs scale-over-push/scale-compose
     pointwise facts (scale(h, p.push(c)) == scale(h,p).push(h·c) by Seq
     ext).
   Then (3) = second-arg congruence + left-distrib (item 2) + the monomial
   lemma, assembled.
4. **Commutativity**: peqv(pmul(p, q), pmul(q, p)). Induction on p using (3).
5. **Right-distributivity**: comm ∘ left-distrib ∘ comm.
6. **Associativity**: peqv(pmul(pmul(p, q), r), pmul(p, pmul(q, r))).
   Induction on p: needs (5) for `pmul(padd(x, y), r)`, plus
   `pmul(scale(c, q), r) ≡ scale(c, pmul(q, r))` and
   `pmul(shiftk(x, 1), r) ≡ shiftk(pmul(x, r), 1)` (the latter falls out of
   the definition: shiftk(x,1) = zero-cons, so pmul(shiftk(x,1), r) =
   padd(scale(zero, r), shiftk(pmul(x, r), 1)) ≡ shiftk(pmul(x,r), 1)).
7. **First-argument congruence** (full peqv, differing lengths): via comm +
   second-arg congruence, or directly via pad + `lemma_pmul_pad`.

**Done when:** all seven verified in tactus-algebra, `./check.sh` 0 errors,
committed. Keep each induction in its own lemma; follow the eqv-chain idioms
from cad-00's writeup.

**Blocked by:** nothing. This is the next climb.

## Progress

- (2026-07-16) Landed in one session: lemmas.rs additions + poly_ring.rs,
  two fix iterations from first draft (a stray `by{}` on a lemma call; a
  refl ordered after its consumer; a missing `=~=` bridge for len-0 Seqs).

## Writeup

All seven deliverables verified in `tactus-algebra/src/poly_ring.rs`
(commit "cad-01: pmul ring laws complete"), gated at 0 errors — with the
cad-15 caveat: the gate verifies proof fns via Z3 until tactus B6 lands.

- T-kit additions (lemmas.rs): `lemma_add_regroup` (4-term),
  `lemma_uniq_neg`, `lemma_neg_add`, `lemma_mul_neg_right`.
- Pointwise stock: zpoly-to-peqv/zpoly-padd, pneg/scale congruences,
  scale-over-padd/-scale/-one/-pneg/-shiftk, padd-pneg, padd-regroup,
  `lemma_cons_as_padd` (head + shifted tail), shiftk-pneg swap,
  inner-shift compose.
- Structural minis: pmul-empty-right (zpoly), pmul-singleton-right
  (= scale, via cons-as-padd — **this is where T-commutativity enters**,
  exactly as predicted in the fleshing), pmul-shiftk-right,
  pmul-shift1-left (definition unfold + zpoly absorb), pmul-scale-right.
- Mains: cong-right (induction), left-distrib over padd/pneg/psub
  (induction + regroup), **comm** (induction riding cons-as-padd +
  singleton + shiftk-right — the push-mirror item 3 of the original plan
  dissolved into these, as hoped), right-distrib over padd/psub (comm
  conjugates), **assoc** (induction consuming right-distrib +
  scale-right + shift1-left), cong-left/both (comm conjugates),
  pmul-one-left.

Notes for cad-02 (the consumer): everything Bézout needs is now present —
distribution over psub both sides, assoc, cong both args, one-mul. The
proof style throughout is eqv-chains with one `axiom_eqv_transitive` per
link; ~950 lines total for the kit. No tactus_tactic anywhere (deprecation
guidance) — all inline.
