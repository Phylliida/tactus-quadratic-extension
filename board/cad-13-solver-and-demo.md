---
title: "M6 untrusted solver + end-to-end demo (three tangent circles)"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

The other half of the architecture — plain untrusted Rust (no proofs), plus
the end-to-end moment:

- Cluster/DR-plan decomposition of a sketch (port the greedy locus solver
  from verus-2d-constraint-satisfaction as one planner; add a small
  simultaneous-cluster fallback), f64 Newton for cluster placement, and
  certificate assembly: recover exact tower data from the numeric solution
  (for locus steps the defining quadratics are exact by construction; for
  coupled clusters, build the resultant/elimination polynomial or let the
  planner supply it symbolically — minimal-poly recognition via LLL is
  optional polish, NOT required for the demo).
- Exec-vs-ghost decision comes due here (DESIGN §12 Q3): the checker so far
  is ghost. Cost sketch (2026-07-16):
  (a) **exec mirror** — runtime BigInt-backed Rational (port
      verus-rational's runtime layer or fresh over verus-bigint-style
      limbs), exec Seq-ops (Vec mirrors), exec TowerElem + normalize +
      D5 + enclosures, each with `ensures result == spec_op(...)`
      invariants. Substantial: comparable to the whole ghost layer again,
      but mechanical; yields a real runnable `check(cert) -> bool`.
  (b) **ghost-only demo** — demo certificates are ghost constants; "running
      the checker" = the verification run itself discharging
      `check_certificate(sketch, cert)` as a proof obligation. Nearly free;
      the artifact is a theorem instance, not a program.
  (c) middle: exec only the HOT leaf (Rational arithmetic + interval
      refinement) and keep tower plumbing ghost — probably a false economy;
      the plumbing is where exec ergonomics bite anyway.
  Recommend (b) for the first demo (it proves the architecture end-to-end),
  then (a) as its own follow-on card when the artifact should become a
  usable library. Talk to Danielle before sinking effort — this is her
  call on what the artifact should BE.
- Demo: three mutually tangent circles (coupled cluster, genuinely
  quadratic-tower-escaping variants exist; the classic Descartes-circle
  configuration is a good spicy target) + one symmetric/degenerate variant
  (exercises D5 collapse + exact path).

**Done when:** sketch → solver → certificate → verified checker accepts,
end-to-end, on both demos; a corrupted certificate rejects; writeup includes
what the certificate looks like concretely (sizes, tower depths, degrees).

**Blocked by:** cad-11 (equality demo can go early); cad-12 for the full
version.

## Progress

## Writeup
