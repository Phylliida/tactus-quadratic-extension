---
title: "M4 constraint layer port: entities + constraints + geometry slice"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

Port from the Z3-era crates into this crate (or a sibling module), keeping
everything generic over `T: OrderedField`:

- `verus-2d-constraint-satisfaction/src/entities.rs` (EntityId,
  ResolvedPoints, ...) and `constraints.rs` (the 26-variant `Constraint<T>`
  enum + `constraint_satisfied` + well-formedness + entity-set specs).
- The slice of `verus-geometry` those specs reference: point2/line2/circle2
  types, `sq_dist_2d`, `orient2d`, `point_on_line2`, `point_on_circle2`,
  `line2_from_points`, and whichever predicate lemmas `constraint_satisfied`
  bodies pull in. Chase the actual imports; port only the cone.

Expect mostly mechanical work (the specs are already generic — the whole
point of the trait-ladder port), with the usual Lean-backend re-proof of any
lemma bodies that used Z3-specific idioms. The greedy locus solver
(solver.rs/locus.rs/construction*.rs) is NOT ported here — it's untrusted
planner material for cad-13.

**Done when:** `constraint_satisfied<T>` and its cone verify in this crate,
0 errors, committed; the probe module retired in favor of a real smoke test
(a couple of constraint_satisfied instances computed over Rational points).

**Blocked by:** nothing hard (generic over the trait ladder, which exists) —
can run parallel to the M1–M3 track. Its consumer is cad-11.

## Progress

## Writeup
