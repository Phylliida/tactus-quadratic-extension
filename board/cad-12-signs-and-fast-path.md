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
- **Interval fast path (v0.2: mostly already built)**: under the v0.2
  ordering route, cad-08's enclosure machinery with certificate-supplied
  fuel IS the fast path — this card only wires branch-sign claims through
  it (cad-09 item 5 is the soundness lemma) and adds the D5 fallback
  dispatch for exact zeros. Deliberately NO verified separation bounds
  (DESIGN §7) and no checker-owned termination story — refinement depth is
  certificate data.

**Done when:** full 26-constraint vocabulary checkable; fast path
demonstrably taken on a non-degenerate demo (log/count which path fired) and
exact path on a degenerate one; 0 errors; committed.

**Blocked by:** cad-09, cad-11.

## Progress

## Writeup
