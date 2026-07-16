---
title: "M4 equality-fragment certificate checker (first end-to-end summit)"
status: todo
claimed_by:
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

The first real summit: a verified checker for the equality fragment
(DESIGN.md §8), needing only M2 — no ordering.

- `SketchCertificate` type: Tower + coords (Map<EntityId, (TowerElem,
  TowerElem)>) + (branch signs deferred to cad-12).
- Checker: (1) structural tower wf (the cad-04/05-level checks; full
  interval-isolation wf upgrades when cad-09 lands); (2) for every equality
  constraint in the sketch, form the residual over tower elements (the
  `constraint_satisfied` bodies are polynomial identities in the
  coordinates — distance², perpendicular, collinear, midpoint, tangency,
  ...) and confirm zero via the D5 zero-test (cad-05/06).
- The theorem, in **relative form** (DESIGN §9.1 route η): checker accepts
  ⟹ `constraint_satisfied_rel(tower, c, coords)` for every equality
  constraint — where `constraint_satisfied_rel` is the mechanically
  relativized copy of the generic predicate (ops → tower-relative ops with
  wf preconditions). Budget the relativization pass of the 26 bodies + the
  geometry cone into this card; it's parallel typed-copy work of a kind
  this workspace has done at much larger scale. Ordering-dependent
  constraints (arc membership, NotCoincident) stay excluded from this
  card's fragment.
- Hand-built demo certificate: the DESIGN gate sketch (unit square +
  diagonals + midpoint constraint, all-rational — depth-0 tower), then a
  depth-1 sketch that actually exercises √: e.g. a point constrained to
  distance 1 from origin on the line x = y (coordinates (√2/2, √2/2)).
  Also the negative control: corrupt one coordinate, checker must reject.

**Done when:** checker verified + both demo certificates behave, 0 errors,
committed. This is the "useful verified system before any ordering theory"
milestone from DESIGN §10 — worth a summit poem.

**Blocked by:** cad-05, cad-06, cad-10.

## Progress

## Writeup
