# DESIGN v0.2 — verified real algebraic numbers for 2D CAD constraint satisfaction

*2026-07-16, Claude + Danielle. Status: M0 mostly landed (see board/); v0.2
revises §6 (ordering: the model is the spec — Sturm demoted off the critical
path) and adds §9.1 (the relative-ring crux). Task board = `board/`.*

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

### 4.2 Well-formedness is checkable

`Level` well-formedness — monic, squarefree (gcd(p, p′) constant with
Bézout data), sign change across (lo, hi), exactly-one-root (v0.2: via a
derivative-monotonicity certificate rather than Sturm counts — see §6) — is
validated by the checker with certificate-supplied refinement fuel. The
checker validates the tower itself before validating anything built on it.
Nothing about the tower is trusted input.

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

## 6. Ordering: the model IS the spec (v0.2 revision)

The hard-won lesson of the dts arc: ordering is the mountain
(`dyn_tower_lemmas.rs` is 23k lines for degree 2). At degree 2 there is a
closed form — sign(a + b√d) by case analysis on sign(a), sign(b),
sign(a² − d·b²) — and that is what dts fought through. At general degree no
closed form exists.

**Key realization (v0.2):** Verus spec logic is classical, so the semantic
model can simply *define* the order — no algebraic decision procedure is
needed at the spec level at all, and Sturm's theorem (v0.1's biggest single
proof) drops off the critical path entirely.

**M3a — the model.** Cauchy sequences of rationals with explicit moduli
(`CReal`); arithmetic; order (classical: `pos(x)` = exists ε>0 and N with
x_n ≥ ε beyond N — not decidable, and it doesn't need to be); an
ordered-ring lemma kit; polynomial-evaluation homomorphism lemmas; root
existence in a sign-change interval by bisection (test points are rational,
so the classical argument is untroubled); and monotone-uniqueness (a
polynomial with sign-constant derivative on an interval has at most one
root there). Each wf level's root is then a spec-level object:
`α := choose|x: CReal| p(x) = 0 && lo < x < hi`, and each tower gets an
evaluation homomorphism `eval: TowerElem → CReal` by structural recursion.

**The spec-level order:** `sign(f) := the sign of eval(f)` — the honest
definition ("the order of the actual real root"), total as a spec function,
noncomputable, which is fine: spec fns don't run.

**M3b — checker-side sign computation, fuel from the certificate.** The
checker computes signs by interval refinement: recursive interval
enclosures of tower elements (a level's α is enclosed by its interval,
refined by bisection whose endpoint sign queries recurse one level down;
polynomial values by interval Horner), refined for **d steps where d is
supplied by the certificate** — the untrusted solver knows the numbers, so
it hands over the fuel. Only the soundness direction is verified (model
value lies in the enclosure; enclosure excluding 0 fixes the sign). Exact
zeros are D5's job, never refinement's. This machinery is *shared with the
§7 fast path* — it is the fast path, with D5 as the exact fallback.

**M3c — the welding lemmas + OrderedField.** D5 certificates translate into
model facts through the evaluation homomorphism:
- Bézout `u·f + v·p ≡ 1` (peqv, checkable) evaluated at α gives
  `u(α)·f(α) = 1` since `p(α) = 0`, hence **f(α) ≠ 0** — zero-test
  soundness, nonzero side.
- `f ≡ w·g` (explicit-quotient divisibility, checkable) with α rooting g
  gives **f(α) = 0** — zero side.
So `eqv` (D5) matches model equality, trichotomy welds, and the
`OrderedField` axioms are inherited from CReal's ordered-ring lemma kit
via the homomorphism — no Sturm-query algebra anywhere.

**Well-formedness (§4.2, revised):** a wf level certificate =
endpoint-sign-change of p (endpoint values are level-(k−1) elements; signs
recurse) + squarefreeness (gcd(p, p′) constant with Bézout — which also
gives p′(α) ≠ 0 at any root by the same Bézout-at-α argument) + a
**monotonicity certificate**: p′ has constant sign on the interval,
witnessed by interval evaluation with certificate fuel. Monotone + sign
change = exactly one root (M3a's uniqueness lemma). Monotonicity is not a
completeness loss: around a simple root p′ is nonzero and continuous, so
the solver can always shrink the interval until the certificate exists.

Uniqueness genuinely matters (not just hygiene): with multiple roots in the
interval, `choose` still picks a consistent α, but the checker's refinement
could isolate a *different* root and compute the wrong sign — the fast-path
soundness lemma needs the interval to pin α.

Alternatives considered and set aside:
- *Sturm–Tarski query as the spec-level sign* (v0.1's route) — total and
  purely algebraic, but it puts Sturm's theorem (the program's biggest
  single proof: BPR ch. 2, root-crossing analysis, proven against the model
  anyway) on the critical path, and its machinery is not shared with
  anything else. Kept as a documented fallback if the enclosure route hits
  an unexpected wall; also a fine later addition for a solver-independent
  wf checker. Prior art if revived: Wenda Li (Isabelle), Cohen (math-comp).
- *Interval-refinement sign as primary spec* — not total at zeros;
  well-definedness already needs the model. Subsumed: the model is now the
  spec and refinement is only the computation.
- *dts-style closed-form case analysis* — does not exist at degree > 2.
- *Fuel-indexed sign predicates* (the `dts_nonneg_fuel` pattern) — the fuel
  now lives in the *certificate*, where it is data, not proof burden; the
  23k-line fuel-threading genre is avoided.

## 7. Fast path: interval filtering, soundness-only

Exact geometric computation practice (Yap, Mehlhorn; LEDA/CORE): almost all
sign queries in real geometry are decided cheaply by adaptive-precision
interval arithmetic; exact algebra should only fire near degeneracy.

Plan: evaluate sign queries first with verified interval arithmetic
(port/reuse `verus-interval-arithmetic`), refining a bounded number of
rounds. The only verified statement needed is the trivial direction: *if the
evaluated interval excludes 0, the sign is as computed*. If inconclusive,
fall back to the exact path (§5/§6), which is complete on its own.

v0.2 note: this is no longer a separate subsystem — §6's M3b enclosure
machinery *is* this fast path (one build, two roles), and the "bounded
number of rounds" is fuel carried in the certificate rather than a
heuristic the checker owns.

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

### 9.1 The relative-ring crux (v0.2 — the honest asterisk on "ports unchanged")

The trait ladder's methods are *absolute*: `zero()`/`one()` are static, and
`eqv`/`mul`-with-reduction for tower elements need the tower — but the tower
is **runtime certificate data**, so there is no honest type to instantiate
`T: OrderedField` with. This is not hypothetical: it is exactly why the dts
crate's Ring/Field impls were removed (`dyn_tower.rs`'s header comment —
the planned `WellFormedDTS<W>` wrapper with `same_radicand` preconditions
is this same crux at degree 2). Routes:

- **(η) Relativize the constraint layer** — baseline, known-viable. Keep the
  generic layer as-is (it still serves Rational and any true instance), and
  produce a mechanically-derived tower-relative copy:
  `constraint_satisfied_rel(tower, c, coords)` with ops
  `tadd(tower, a, b)`, `teqv(tower, a, b)`, ... and wf preconditions. The 26
  constraint bodies and the small geometry cone relativize mechanically
  (this workspace has done far larger parallel typed-copies —
  the pred-Britton copy). Checker theorem targets the relative predicate; a
  bridging lemma equates the two on any genuine instance.
- **(δ) Compatibility-guarded trait ladder** — a parallel `RelRing`/…/
  `RelOrderedField` ladder whose axioms carry `compatible(a, b)`
  preconditions (elements carry their tower; compatible = same tower). More
  principled, heavier: every generic geometry lemma consumed must be
  re-proven against the guarded axioms. Consider only if (η)'s duplication
  becomes painful.
- **(γ) Tower-merging total instance** — make ops total by merging
  mismatched towers deterministically. Worked through and rejected: the
  order-compatibility axioms (`le_add_monotone`) fail for mismatched-tower
  junk elements; making them hold needs real tower compositum, the
  exponential thing this design exists to avoid.

Decision: start (η) at cad-04/cad-11 time; it also keeps the checker's
statement concrete (a plain predicate over certificate data, no typeclass
indirection in the trusted statement — arguably a trust *improvement*).

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

  *Status 2026-07-16: groundwork DONE except the polynomial module.
  Trait layer = new `tactus-algebra` crate (Danielle's call): ported
  verus-algebra ladder verbatim (30 fns, falsification-probed) + fresh
  Rational OrderedField instance (74 fns, ~530 lines vs ~3k Z3-era —
  key idiom: `by (nonlinear_arith)` blocks see only their `requires`,
  so every axiom carries its closed-form facts explicitly; oversized
  identities decompose into int-atom helper lemmas). Cross-crate
  import validated end-to-end via `build-export.sh` (.vir/.rlib) +
  `--import/--extern` in this crate's check.sh; see src/probe.rs.*
- **M1 — tower core.** `Tower`/`Level`/`TowerElem` + wf predicates;
  quotient-ring ops (add/mul/neg/normalize mod defining); `Ring` instance by
  induction on height.
  *Gate: Ring axioms for TowerElem.*
- **M2 — D5.** Zero-test with gcd splitting and tower rewriting; Bézout
  inverses; field ops gated on nonzero certificates. The splitting-threaded
  PRS is the fiddly core (§5).
  *Gate: zero-test sound and complete against eqv; div/recip specs.*
- **M3 — ordering (the mountain, v0.2 route).** M3a Cauchy model (classical,
  with root-existence and monotone-uniqueness) → M3b interval-enclosure
  machinery with certificate fuel (shared with M5's fast path) → M3c
  welding lemmas (Bézout-at-α) + `OrderedField` axioms via the evaluation
  homomorphism (§6). Sturm is off the critical path (documented fallback).
  Still the long arc; expect it to be much shorter than dts's 23k lines per
  unit of generality.
  *Gate: OrderedField axioms for the tower (relative form per §9.1).*
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

1. ~~**Trait layer** (M0)~~ — *answered 2026-07-16: new `tactus-algebra`
   crate with the ported ladder + fresh Rational; Mathlib enters through
   Lean-side tactics, not type reuse.*
2. ~~**Crate name**~~ — *answered 2026-07-16: keep the name, let it grow;
   easy to rename later.*
3. **Ghost-first vs exec-first**: old crate is almost all
   `#[cfg(verus_keep_ghost)]` with a thin runtime. Same here (ghost checker
   first, exec mirror later), or exec from the start? (Currently
   proceeding ghost-first.)
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
