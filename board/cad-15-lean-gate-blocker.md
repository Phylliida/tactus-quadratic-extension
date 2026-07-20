---
title: "full-Lean discharge of the trait corpus — blocked on tactus lean-all-proofs user-trait support (B6)"
status: todo
claimed_by:
created: 2026-07-16T23:59:00Z
updated: 2026-07-16T23:59:00Z
---

## Description

Discovered 2026-07-16 (Danielle asked "make sure you're using lean as the
backend"): the arc's verification-gate story needed correcting, and full-Lean
discharge of our corpus is currently blocked on a tactus translator gap.

The facts (evidence + repro in `../../tactus/BUG-lean-all-proofs-user-traits.md`):
1. `--emit-lean` skips the Lean run entirely (floor-only measurement mode) —
   it was in our check.sh files (copied from tactus-group-theory's leftover);
   now removed. Never reintroduce it.
2. Under plain `--lean-backend`, ordinary proof fns are verified by **Z3**
   (that is tactus's current default posture); Lean handles tactus_tactic
   fns, exec packages, and the spec world. Our crates are pure proof fns, so
   the current honest gate for them is Z3 (sound — falsification-probed —
   just not Lean).
3. `--lean-all-proofs` (the route-everything-to-Lean flag) fails on our
   corpus with a systematic translator gap: user trait classes are not
   emitted into island preambles (cross-module refs = unknown identifiers)
   and impl obligations aren't connected to instance bodies. tactus-algebra:
   2/182 under lean-all-proofs vs 182/182 under Z3 — the only passing fn is
   the only trait-free one, so this is emission, not proof power.

This card tracks the blocker, NOT tactus-side work (that happens in the
tactus repo; tactus-algebra is named there as the acceptance corpus).

**What stays true for this arc meanwhile:** proofs continue to be written
inline (no tactus_tactic/tactus_auto — deprecation guidance from Danielle
2026-07-16), backend-portable, and gated on the default `--lean-backend`
check (Z3 for proof fns). The bet that Lean-native induction makes M3
tractable is UNTESTED until B6 lands — plan accordingly: re-measure
`--lean-all-proofs` on tactus-algebra whenever tactus's translator moves,
and treat a green lean-all-proofs run as this card's close condition.

**Done when:** `verus --lean-backend --lean-all-proofs` on tactus-algebra
and tactus-quadratic-extension = 0 errors (or a Danielle-approved revised
gate definition), and check.sh files updated to make that the standing gate.

**Blocked by:** tactus-side B6 fix (outside this board).

## Progress

- (2026-07-16) Diagnosed; BUG doc filed and committed in tactus (`6810558`);
  check.sh files fixed (--emit-lean removed); memory updated
  (emit-lean-skips-lean, no-tactus-tactic-in-new-code).
- (2026-07-19) Re-measured post-B6-emission-closure (tactus `e3da0f9` +
  `d8195ed` "force all lean" — plain `--lean-backend` now routes everything
  to Lean, no separate flag): tactus-algebra **59 → 85 verified**. Landed
  crate-side (`4fdb4f4` in tactus-algebra): check.sh exports LEAN_PATH
  (Mathlib — `by (nonlinear_arith)` emits `import Mathlib.Tactic.Linarith`;
  was an entire 27-fn error class, now 0); `OrderedRing.ge/gt` made required
  methods (Lean emission mishandles trait defaults); Rational `eqv/le/lt`
  inlined in impls; ordered lemma kit in lemmas.rs (also pulls
  PartialOrder/OrderedRing classes into the defs umbrella — the
  unknown-identifier class is now 0). New BUG doc filed in tactus
  (`0d0f9b4`): default-method class/instance emission + impl-only class
  walk-gap. **Remaining 205 errors are one family: the fixed closer script
  (rfl/decide/omega + simp set) can't recursively unfold spec defs in goals
  or apply dictionary axiom fields** — exactly tactus's N3 arc
  (`e2631b2` DESIGN-N3, committed same day). Validated the N3
  recursive-unfold direction by hand: adding `eqv_spec, denom, denom_nat`
  to the failing simp set closes the goal (probe in session log).
- (2026-07-19, N3-M1) First N3 milestone landed in tactus (`a2f70be`):
  UnfoldOnce arm (form B — `rw [f]` one-step unfold when a goal's LHS
  head is a recursive spec fn; detection sees through
  SpanMark/TypeAnnot/nested-App + N1's let-eq wrapper; guard simp
  excludes broadcast haves by name or ext_equal explodes the goal's
  Seq equality) + two-phase form E arm (targeted unfold, then guarded
  split — one arm, or `split` never sees the guards). tactus-algebra
  **85 → 87 verified, 205 → 177 failing obligations**, no regressions;
  rust_verify_test 138/140 (2 pre-existing state_machines failures,
  stash-verified). Remaining = form C (eqv-chaining, M4), Rational
  nonlinear (~16, own story), termination/bound-normalization (M2
  scripts), and the N3-M0 census harness.
- (2026-07-19, N3-M0) Provenance + census harness landed in tactus
  (`9a88b6c`): `HypProvenance::Other` split into
  Requires/HoistEq/CtorEq/LoopInv/AssertFact/AssumeFact; CallFact
  carries the ensures shape summary (form D input); every emitted
  theorem carries `-- tactus-closer: <class>` and the N4 summary line
  prints at crate end (algebra: 806 formE / 28 formB+formE / 3 formB /
  37 rung-only across 874 theorems). The M2 script IR now has its
  substrate: typed frames + named hyps + the census ratchet.
- (2026-07-19, N3-M2) Script IR + author v1 landed in tactus
  (`733546a`): scripts emitted primary (form A branch+woven-fact,
  form B recursive-unfold) with the derived chain as fallback; script
  census classes live. tactus-algebra **205 → 166 failing
  obligations, 85 → 87 verified**. Key wins: GuardSimpStar `at *`
  (fact hyps normalize each other's bounds — the ↑(len x)/↑1 case M1
  couldn't reach); StructuralTail excludes ext_equal haves by local
  name; `| done` terminates every close (rfl/omega error on zero
  goals). Phantom audit: 27 direct-Mathlib files elaborate clean.
  Remaining = form C (eqv chains — the bulk of what fails now),
  Rational nonlinear (~16), termination obligations.
- (2026-07-19, form C / M4) Equivalence chaining landed in tactus
  (`d5706f2`): the §11.2 bet paid off — the goal's spine IS the
  user's proof trace, so ExactHyp after let/hoist substs closes the
  eqv-chain preconditions AND postconditions; no axiom-field
  instantiation needed in v1. tactus-algebra **166 → 157 failing
  obligations**; 488/874 theorems (56%) now script-authored
  (A:292 B:31 C:165). Remaining: Rational nonlinear (~16, needs
  `ring`-class power), termination obligations, and the harder eqv
  chains where the final fact is a 2-link transitivity away
  (ApplyLemma on trans, not ExactHyp).
- (2026-07-19, the Rational story) R1 defeq-bridge + R2 nonlinear
  ladder landed in tactus (`4f166a8`): `exact h` by defeq for
  projection-vs-raw-form bridges; congrArg-multiplied-hyp pool
  (beta-reduced types) + mul_eq_zero cancel for the equality chains;
  transitive non-recursive unfold closure (denom→denom_nat); targeted
  unfold `at ⊢ <mentioning hyps>` (never bare `at *` — whnf timeout
  on divmod-sized contexts). **axiom_eqv_transitive,
  axiom_le_transitive, axiom_add_associative, axiom_mul_associative
  fully green**; tactus-algebra 157 → 139 errors, 85 → 91 verified.
  Remaining Rational: congruence classes needing cancel in more
  shapes, recip sign-splits, small den-equalities.

## Writeup
