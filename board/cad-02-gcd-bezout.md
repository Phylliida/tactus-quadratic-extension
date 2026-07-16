---
title: "trim + Euclidean gcd with Bézout certificate"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

The M0 crown: `gcd(a, b) -> (g, u, v)` with **Bézout ensures**
`peqv(g, padd(pmul(u, a), pmul(v, b)))`, in tactus-algebra.

Pieces:

1. **trim**: drop trailing eqv-zero coefficients. divmod's remainder can have
   eqv-zero trailing junk (e.g. `[1, 0₊]`), and divmod's precondition needs a
   non-eqv-zero last coefficient. Ghost proof fn, recursion on length,
   branching on the spec condition `p.last().eqv(T::zero())`:
   `trim(p) -> tp` ensures `peqv(tp, p)` and
   `tp.len() == 0 || !tp.last().eqv(T::zero())`. Reuses
   `lemma_drop_last_peqv`.
2. **zpoly decision bridge**: `trim(p).len() == 0 <==> zpoly(p)` (or just the
   direction needed: if trim is empty, p ≡ zero-poly — pointwise).
3. **Euclid**: recursion `gcd(a, b) = if zpoly-ish(b) { (a, one-poly, empty) }
   else { let (q, r) = divmod(a, trim(b)); let (g, u1, v1) = gcd(trim(b),
   trim(r)); (g, v1, u1 − v1·q ... ) }` — careful with which argument trims
   and what the decreases measure is: `trim(r).len() < trim(b).len()` since
   `r.len() < trim(b).len()` and trim never lengthens. decreases
   `trim(b).len()` (or thread trimmed-ness as a precondition and trim at the
   call boundary; design freedom).
4. **Bézout algebra**: from IH `g ≡ u1·b + v1·r` and `a ≡ q·b + r` (peqv),
   derive `g ≡ v1·a + (u1 − v1·q)·b`. Needs from cad-01: left-distributivity
   over psub, associativity (v1·(q·b) ≡ (v1·q)·b), second-arg congruence,
   plus padd/psub shuffles. Also congruence to move between b/r and their
   trims (first-arg congruence of pmul from cad-01 item 7).
5. Base case: `g = a` with `u = [one], v = []`: needs
   `peqv(pmul([one], a), a)` (one-scale lemma: scale(one, a) ≡ a pointwise +
   the empty-shift absorb) — small.

Deliberately NOT in scope (see cad-14): g divides a and b as a theorem;
"greatest"; uniqueness. The D5 checker (cad-05) consumes gcd outputs by
re-checking concrete identities, so Bézout is the only ensures it needs.

**Done when:** gcd verified with the Bézout ensures + a length/trimmed ensures
on g (whatever cad-05 turns out to need — at minimum
`g.len() == 0 || !g.last().eqv(zero)`), `./check.sh` 0 errors, committed.

**Blocked by:** cad-01.

## Progress

## Writeup
