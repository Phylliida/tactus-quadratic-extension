# DESIGN v0.1 — verified real algebraic numbers for 2D CAD constraint satisfaction

*2026-07-16, Claude + Danielle. Status: proposed, not yet started.*

## 0. Summary

A formally verified 2D sketch constraint-satisfaction system, built as a
**verified certificate checker** over a new number type: **towers of simple real
extensions** (general-degree, not just quadratic), with zero-testing by the
**D5 dynamic evaluation principle** (no factoring, no irreducibility proofs)
and ordering specified by **Sturm–Tarski queries**. The constraint solver
itself (Newton, decomposition planning) stays untrusted plain Rust; only the
checker carries proofs. Verified under tactus (Lean backend), where the
recursive-induction and ring-identity work this needs is bread-and-butter
rather than the Z3 trigger/context fight the quadratic (dts) version endured.

The existing `verus-2d-constraint-satisfaction` constraint vocabulary and
`verus-geometry` predicates are generic over `OrderedField`, so they port
essentially unchanged: once the tower is a verified `OrderedField` instance,
`constraint_satisfied<T>` just works over it.

## 1. Goal and scope

Given a sketch (points, lines-through-points, circles, and constraints:
coincidence, distance, perpendicularity, tangency, collinearity, angle, ...),
verify end-to-end that a proposed solution *exactly* satisfies every
constraint — no floating point in the trusted path, no `assume`, no
`external_body`.

In scope: the number tower, its ordered-field proofs, the certificate format
and checker, a port of the constraint layer, an untrusted solving pipeline.

Out of scope (deferred): 3D, splines/NURBS beyond the existing
verification-only ellipse/arc constraints, solver completeness or robustness
guarantees, UI.

## 2. Where the degree blowup actually comes from

Two distinct problems hide inside "circles intersecting circles intersecting
circles gets very high degree", and they have different cures:

**Problem A — tower height (iterated pairwise intersection).** Ruler-and-
compass style solving — what the old greedy locus solver does — is degree ≤ 2
per step: circle∩circle reduces to the radical axis, then line∩circle.
Solutions live in √-towers. The blowup is *tower height*: a depth-n
construction can have flattened degree 2ⁿ. That worst case is intrinsic (it is
the actual field extension degree, not a representation artifact), so no
representation escapes it. The goal is *output-sensitive* cost: pay for the
degree a number really has, not for the depth of the construction that
produced it. Real sketches are full of symmetry and round numbers, so deep
elements collapse constantly — the representation must harvest those
collapses (see §5).

**Problem B — coupled clusters (the real reason to generalize).** Constraint
subsystems that cannot be solved one point at a time produce genuinely
high-degree *single* extensions, and Gao–Chou showed some well-constrained 2D
configurations are **not solvable by radicals at all**. So √-towers are not
just inefficient there — they are insufficient in principle. Any honest
general solver needs arbitrary real algebraic numbers.

Both problems are served by the same representation (§4): each tower level
adjoins one root of one polynomial — degree 2 levels for locus steps, higher
degree levels for coupled clusters — and per-level normalization keeps each
number at its true degree.

## 3. Architecture: verify the checker, not the solver

Solving is search; checking is algebra. The split:

```
untrusted (plain Rust, fast, floats allowed)        verified (tactus)
─────────────────────────────────────────────       ─────────────────────────
decomposition planning (DR-plan / clusters)         number tower + field ops
numeric root finding (Newton, homotopy)             D5 zero test + certificates
minimal-polynomial recognition (LLL/PSLQ, opt.)     sign determination (Sturm)
certificate assembly                                constraint residual checker
```

The untrusted side emits a `SketchCertificate` (§8); the verified checker
validates it. The checker's postcondition is the theorem: *if the checker
accepts, every constraint in the sketch is satisfied by the certified
coordinates* — where "satisfied" is the existing spec-level
`constraint_satisfied<T: OrderedField>` instantiated at the tower.

This matches the certificate philosophy used across the workspace (tactus
bootstrap R2, CompSpec checkers). It also means solver bugs cannot produce a
wrong "verified" answer — only a rejected certificate.

A satisfying consequence of the certificate shape: sketches are purely
existential (here is a witness, check it), so we never need quantifier
elimination over the reals. The decision-procedure world's "CAD" —
cylindrical algebraic decomposition, doubly exponential — is exactly what we
get to avoid. CAD without CAD.

## 4. The number type: towers of simple real extensions

### 4.1 Representation

A tower is a sequence of levels. Level k adjoins α_k, a root of a monic
squarefree polynomial p_k whose coefficients live at levels < k, selected
among p_k's real roots by a rational isolating interval:

```rust
/// One level: adjoin the unique root of `defining` in (lo, hi).
pub ghost struct Level {
    /// Monic, squarefree; coefficients are elements of the tower below.
    /// Index i = coefficient of x^i.
    pub defining: Seq<TowerElem>,
    /// Rational isolating interval: sign change across it, and exactly
    /// one root inside (Sturm-certified — decidable well-formedness, §6).
    pub lo: Rational,
    pub hi: Rational,
}

pub ghost struct Tower { pub levels: Seq<Level> }

/// Element = nested polynomial representation.
/// At depth k: polynomial in α_k, coefficients at depth k−1,
/// degree < deg(levels[k].defining).
pub ghost enum TowerElem {
    Base(Rational),
    Poly(Seq<TowerElem>),
}
```

(Exact type shape is M1 design freedom — `DynTowerSpec`'s recursive-enum
style worked well before; a level-indexed wf predicate
`elem_wf(tower, k, e)` ties elements to their depth.)

Notes:

- **Do not flatten.** The primitive element theorem would let us collapse a
  tower to one big extension, but flattening destroys sparsity and locality.
  Nested representation means an element depending only on early levels stays
  small, and constraint residuals between nearby entities only touch their
  ancestor levels — independent parts of a sketch never pay for each other.
- **Degree 2 is still the common case.** Locus-style steps produce quadratic
  levels; the old dts tower is exactly the degree-2 specialization. The
  general machinery should not make the common case slower than necessary
  (per-level fast paths are fine, later).
- **Base field = ℚ** (arbitrary-precision rationals). Which rational to use
  under the Lean backend is an M0 groundwork question (§10).

### 4.2 Well-formedness is decidable

`Level` well-formedness — monic, squarefree (gcd(p, p′) constant), sign
change across (lo, hi), exactly-one-root (Sturm variation count = 1) — is
computed by the same recursion that computes signs (§6). The checker
validates the tower itself before validating anything built on it. Nothing
about the tower is trusted input.

## 5. Zero-testing without factoring: the D5 principle

(Della Dora–Dicrescenzo–Duval, "dynamic evaluation".) The classical trap:
quotient rings F[x]/(p) are only fields when p is irreducible, and verified
irreducibility is miserable (the `IrreduciblePoly` axiom trait in
`verus-field-extension` is the wrong trust anchor — it would have to be
discharged for every level of every certificate). D5 removes the requirement:
keep p merely **squarefree**, and let zero-tests repair the tower on demand.

To decide whether f(α) = 0 for f a representative at level k with defining
polynomial p:

1. Compute g = gcd(f, p) in (level k−1)[x], with Bézout data u·f + v·p = g.
2. **g constant** ⇒ f is invertible mod p ⇒ f(β) ≠ 0 for *every* root β of
   p, in particular α. The (scaled) Bézout identity u·f + v·p = 1 is a
   syntactic certificate checkable by polynomial arithmetic alone.
3. **g nonconstant** ⇒ p = g·(p/g) splits. Decide which factor owns α by
   refining the isolating interval (bisection; endpoint signs are
   level-(k−1) sign queries, recursing down the tower). Then **replace the
   level's defining polynomial with that factor**:
   - α owns g ⇒ f(α) = 0 exactly (g | f, exhibited by the explicit quotient),
     and the level's degree just dropped;
   - α owns p/g ⇒ f(α) ≠ 0, and the level's degree dropped anyway.

The tower simplifies itself as a side effect of computation. This is the
mechanism that harvests designer symmetry (§2, problem A): elements that are
"secretly" low-degree get discovered at the first zero-test that touches
them, and every discovery permanently shrinks the level.

The intricate part (flagging honestly): gcd over level-(k−1) coefficients
needs leading-coefficient invertibility tests, which are themselves
level-(k−1) zero-tests, which may themselves split lower levels. The
verified version must thread splitting through the polynomial remainder
sequence. This is well-trodden algorithmically (Duval; the regular-chains
literature) and terminates — degrees strictly decrease on every split — but
it is the fiddliest single algorithm in the plan. M2's core task.

Division/`recip` follows for free: the Bézout data from a successful
nonzero-test *is* the inverse.

## 6. Ordering: Sturm–Tarski sign as the spec, a small real model for its axioms

The hard-won lesson of the dts arc: ordering is the mountain
(`dyn_tower_lemmas.rs` is 23k lines for degree 2). At degree 2 there is a
closed form — sign(a + b√d) by case analysis on sign(a), sign(b),
sign(a² − d·b²) — and that is what dts fought through. At general degree no
closed form exists, so we need the systematic tool the closed form was a
shadow of.

**The spec-level sign function is a Sturm–Tarski query.** For f at level k
with defining (p, lo, hi): the Tarski query TaQ(f, p; lo, hi) — computed
from the signed remainder sequence of (p, p′·f), evaluating sign variations
at lo and hi — equals the sign of f at the unique root of p in the interval.
Every step of that computation is arithmetic and sign-testing in the
coefficient field, i.e. level k−1, so by structural recursion **sign is a
total computable spec function** on well-formed towers, bottoming out in ℚ.
No limits, no fuel, no termination escrow. Well-formedness of levels (§4.2)
is the same machinery pointed at (p, p′).

The correctness obligations — sign is well-defined on eqv-classes, trichotomy
against the D5 zero-test, sign(f·g) = sign(f)·sign(g), sign compatibility
with addition — give the tower its `OrderedField` instance, and they are
theorems *about Sturm queries*. The natural proofs go through a model in
which p actually has a root:

**M3a — a minimal constructive-real model over ℚ.** Cauchy sequences of
rationals with explicit moduli; arithmetic; sign-with-witness; polynomial
evaluation continuity; and root existence by sign-change bisection. The
usual constructive-IVT delicacy (undecidable signs at test points) does not
bite here: our test points are rational and coefficient signs are decidable
by the induction hypothesis, so bisection is fully decidable at every step
(a zero at a rational midpoint just means we found the root exactly). This
is a bounded, textbook chunk — not a full real-analysis library, just enough
to give each tower an evaluation map into "honest numbers".

**M3b — Sturm machinery.** Signed remainder sequences, sign-variation
counts, the Sturm–Tarski theorem relating TaQ to the sign at the root, root
counting for the exactly-one-root wf condition.

**M3c — the `OrderedField` instance**, by induction on tower height, using
M3a's model to interpret M3b's statements.

Prior art says this shape is formalizable: Cyril Cohen's real-closure
construction in Coq/math-comp, Wenda Li's Sturm theory in Isabelle, the
algorithmic backbone in Basu–Pollack–Roy ch. 2. And the degree-2
specialization of the Sturm query *is* the dts case analysis — so this is
the direct generalization of a fight already won once, with better tools:
the Lean backend handles recursive induction and ring identities natively
(recursive-induction discharge solved 2026-07-15, probe32 idioms), where Z3
needed trigger surgery and context-pollution workarounds for every lemma.

Alternatives considered and set aside:
- *Interval-refinement sign as primary spec* — refinement is only
  semi-decidable at zero, so sign is not a total function without the D5
  test woven in, and well-definedness ("refinement terminates when f ≠ 0")
  already requires the model. Sturm gives totality for free.
- *dts-style closed-form case analysis* — does not exist at degree > 2.
- *Fuel-indexed sign predicates* (the `dts_nonneg_fuel` pattern) — worked at
  degree 2, but fuel-threading across towers and eqv was a large share of
  the 23k-line pain; a total function avoids the whole genre.

## 7. Fast path: interval filtering, soundness-only

Exact geometric computation practice (Yap, Mehlhorn; LEDA/CORE): almost all
sign queries in real geometry are decided cheaply by adaptive-precision
interval arithmetic; exact algebra should only fire near degeneracy.

Plan: evaluate sign queries first with verified interval arithmetic
(port/reuse `verus-interval-arithmetic`), refining a bounded number of
rounds. The only verified statement needed is the trivial direction: *if the
evaluated interval excludes 0, the sign is as computed*. If inconclusive,
fall back to the exact path (§5/§6), which is complete on its own.

Deliberately **not** verifying separation bounds (BFMSS/Davenport–Mahler):
because the exact path is a complete decision procedure, separation bounds
would only tune *when to give up on refinement* — a pure performance
heuristic that can live untrusted. This makes M5 much lighter than EGC
folklore suggests. (If profiling later shows the heuristic matters, verified
bounds can be added without touching the trust story.)

Degenerate-case economics, for intuition: in CAD, exact zeros are usually
*intended* coincidences (symmetries the designer drew on purpose), and those
are precisely the cases where D5's gcd collapses fast. The expensive path is
rare and self-shrinking.

## 8. Certificate format

```rust
pub struct SketchCertificate {
    /// One level per root adjunction — quadratic for locus steps,
    /// higher-degree for coupled clusters. The tower's own wf is checked.
    pub tower: Tower,
    /// Coordinates for every resolved entity.
    pub coords: Map<EntityId, (TowerElem, TowerElem)>,
    /// Branch/orientation selections (which intersection, which side),
    /// each a sign claim checked by §6 (or §7 fast path).
    pub branch_signs: Seq<SignClaim>,
}
```

Checking:
1. Tower well-formedness (§4.2) — includes all defining polys squarefree,
   intervals isolating.
2. Every equality constraint: form the polynomial residual (the existing
   `constraint_satisfied` bodies are already polynomial identities —
   `DistanceSq`, `Perpendicular`, `Collinear`, `TangentCircles`, ... — over
   the coordinate elements), normalize in the tower, confirm zero via D5.
3. Every ordering-flavored condition (arc membership, non-degeneracy
   `NotCoincident`, branch signs): sign query via §7-then-§6.

The checker's `ensures` ties acceptance to `constraint_satisfied` for every
constraint — that spec is the trusted meaning of the sketch, unchanged from
the Z3-era crate.

The untrusted side is free to be clever (hand over recognized minimal
polynomials, pre-collapsed towers, tight intervals); the checker never cares
how the certificate was produced.

## 9. Constraint layer

Port `verus-2d-constraint-satisfaction`'s `entities.rs` + `constraints.rs`
(26 constraint kinds, all generic over `OrderedField`) and the needed slice
of `verus-geometry` (point2/line2/circle2 + the predicate lemmas the
constraint specs reference). The old greedy locus solver (`solver.rs`,
`locus.rs`, `construction*.rs`) is *not* in the trusted path anymore — it
becomes one untrusted planner among several (the special case where every
cluster is a single point placed by two loci, degree ≤ 2 per level). Port it
later, unverified or lightly verified, as a certificate producer.

## 10. Milestones

Each milestone gates on `./check.sh` = 0 errors (crate-local, Lean backend,
tactus-group-theory conventions: `-V cache`, tee to log).

- **M0 — groundwork.** Crate skeleton (Cargo.toml, check.sh, src/lib.rs).
  Resolve the trait-layer question: does `verus-algebra` (traits +
  Rational) verify under the Lean backend as a dependency, or does this
  crate want a tactus-native minimal trait layer? Then: polynomial module
  over a generic field — add/mul/divmod, gcd with Bézout data, squarefree
  part, evaluation — with the standard lemma set.
  *Gate: division/Bézout/squarefree postconditions proven.*
- **M1 — tower core.** `Tower`/`Level`/`TowerElem` + wf predicates;
  quotient-ring ops (add/mul/neg/normalize mod defining); `Ring` instance by
  induction on height.
  *Gate: Ring axioms for TowerElem.*
- **M2 — D5.** Zero-test with gcd splitting and tower rewriting; Bézout
  inverses; field ops gated on nonzero certificates. The splitting-threaded
  PRS is the fiddly core (§5).
  *Gate: zero-test sound and complete against eqv; div/recip specs.*
- **M3 — ordering (the mountain).** M3a mini Cauchy model → M3b Sturm
  machinery → M3c `OrderedField` instance (§6). Expect this to be the long
  arc; expect it to be much shorter than dts's 23k lines per unit of
  generality, and revisit the route if M3b suggests a cheaper path.
  *Gate: OrderedField axioms for the tower.*
- **M4 — equality-fragment checker** (needs only M2, can run parallel to
  M3). Port entities/constraints; certificate type; verified checking of
  all equality constraints end-to-end.
  *Gate: a hand-built certificate for a small sketch (square + diagonals +
  midpoint) accepted; a corrupted one rejected.*
- **M5 — signs + fast path** (needs M3). Branch-sign checking; interval
  filter (soundness-only) in front of exact signs; arc/non-degeneracy
  constraints live.
  *Gate: full 26-constraint vocabulary checkable.*
- **M6 — untrusted solver + demo.** Plain-Rust Newton + cluster
  decomposition emitting certificates; demo on something spicy — three
  mutually tangent circles with a symmetry-induced degeneracy exercises
  every code path (coupled cluster, D5 collapse, branch signs).
  *Gate: end-to-end sketch → certificate → verified accept.*

M0–M2 + M4 already constitute a useful verified system (equality-only
checking) before any ordering theory lands — a real intermediate summit.

## 11. Asset map

| Existing | Fate |
|---|---|
| `verus-2d-constraint-satisfaction` entities/constraints | port (M4), near-verbatim |
| `verus-2d-constraint-satisfaction` solver/locus | later, untrusted planner (M6) |
| `verus-geometry` point2/line2/circle2 + predicates | port the needed slice (M4) |
| `verus-algebra` traits (Ring/OrderedField), Rational | M0 decision: dep vs port |
| `verus-quadratic-extension` dts towers | superseded by general towers; its ordering lemmas are the degree-2 shadow of §6 and a proof-strategy reference |
| `verus-field-extension` `SpecExt` | seed for M1's per-level quotient ring; drop the `IrreduciblePoly` axiom trait (replaced by D5 + squarefree wf) |
| `verus-interval-arithmetic` | port/reuse for M5 fast path |

## 12. Open questions (for Danielle)

1. **Trait layer** (M0): try `verus-algebra` under the Lean backend first,
   or start with a fresh minimal tactus-native trait file? (Affects how
   verbatim the geometry/constraint port can be.)
2. **Crate name**: the design outgrew "quadratic" — rename to something like
   `tactus-real-algebraic` / `tactus-cad`, or keep the name and let it grow?
3. **Ghost-first vs exec-first**: old crate is almost all
   `#[cfg(verus_keep_ghost)]` with a thin runtime. Same here (ghost checker
   first, exec mirror later), or exec from the start?
4. **First demo scope**: which constraint subset should M4's demo sketch
   exercise?

## 13. References

- Della Dora, Dicrescenzo, Duval — *About a new method for computing in
  algebraic number fields* (EUROCAL '85): the D5 principle.
- Duval — *Algebraic numbers: an example of dynamic evaluation* (J. Symbolic
  Computation, 1994).
- Basu, Pollack, Roy — *Algorithms in Real Algebraic Geometry*, ch. 2
  (Sturm–Tarski, signed remainder sequences) and ch. 8 (real root counting).
- Yap — *Towards exact geometric computation* (separation-bound EGC
  paradigm); Mehlhorn et al. — LEDA reals; Karamcheti et al. — CORE.
- Fudos, Hoffmann — *A graph-constructive approach to solving systems of
  geometric constraints* (cluster/DR-plan decomposition); Owen —
  *Algebraic solution for geometry from dimensional constraints*.
- Gao, Chou — *Solving geometric constraint systems II: a symbolic approach
  and decision of Rc-constructibility* (radical-unsolvable well-constrained
  configurations).
- Cohen — real closed fields / real closure in Coq–math-comp; Wenda Li —
  Sturm sequences in Isabelle/HOL.
- Sage `QQbar` (lazy algebraic reals with minimal-poly collapse) and Maple
  `RegularChains` — engineering prior art for §5's self-simplifying towers.
