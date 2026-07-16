---
title: "M5 branch signs + interval fast path (soundness-only)"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

Complete the constraint vocabulary (DESIGN.md §7–8):

- Branch/orientation sign claims in the certificate (which intersection,
  which side), checked by the verified sign function (cad-09). Wire the
  ordering-dependent constraints (ArcMembership, NotCoincident, and
  orient2d-based predicates) into the checker.
- **Interval fast path**: adaptive-precision rational interval evaluation of
  sign queries, refining a bounded number of rounds, with ONLY the
  soundness direction verified ("evaluated interval excludes 0 ⟹ the sign
  is as computed"); inconclusive falls back to the exact Sturm/D5 path,
  which is complete on its own. Deliberately NO verified separation bounds
  (DESIGN §7) — the give-up heuristic is untrusted tuning.
  Port/adapt what's useful from `verus-interval-arithmetic` (Z3-era, ~193
  fns) rather than reinventing; the soundness lemma is the only proof
  obligation.

**Done when:** full 26-constraint vocabulary checkable; fast path
demonstrably taken on a non-degenerate demo (log/count which path fired) and
exact path on a degenerate one; 0 errors; committed.

**Blocked by:** cad-09, cad-11.

## Progress

## Writeup
