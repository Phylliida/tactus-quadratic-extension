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

## Writeup
