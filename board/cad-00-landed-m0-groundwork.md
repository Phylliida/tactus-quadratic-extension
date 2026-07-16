---
title: "LANDED: M0 groundwork — tactus-algebra (traits, Rational, poly framework, divmod), cross-crate import"
status: done
claimed_by: fable
created: 2026-07-16T22:30:00Z
updated: 2026-07-16T22:30:00Z
---

## Description

The foundation card: everything landed on 2026-07-16, recorded so later cards
have a single pointer to what exists and how it was built.

## Progress

- (2026-07-16) DESIGN v0.1 written and committed (`7570097` in this repo).
- (2026-07-16) tactus-algebra built from blank repo through divmod, five
  commits, every slice 0 errors, most first-try.

## Writeup

**What exists (all verified under `./check.sh`, Lean backend, 0 errors):**

In `../../tactus-algebra`:
- `src/traits/` — the verus-algebra ladder ported verbatim: `Equivalence` →
  `AdditiveCommutativeMonoid`/`PartialOrder` → `AdditiveGroup` → `Ring` →
  `OrderedRing` → `Field` → `OrderedField`, plus the `int` instance.
  Falsification-probed (an injected false assert IS caught).
- `src/rational.rs` — `Rational { num: int, den: nat }` (denominator
  `den + 1`, positive by construction), full `OrderedField` instance in ~530
  lines of fresh Lean-backend-native proofs (the Z3-era instance path was
  ~3k lines).
- `src/lemmas.rs` — derived generic lemmas from the trait axioms: zero-left
  laws, congruence variants (`lemma_add_cong_right/both`,
  `lemma_mul_cong_right/both`), `lemma_recover` (y + (x + −y) ≡ x),
  `lemma_mul_recip_cancel`, `lemma_kill_top` (the division-step kill).
- `src/poly.rs` — polynomials as `Seq<T>`: total `coeff` accessor (zero
  beyond length), **length-agnostic** `peqv` (pointwise eqv of total
  coefficients), `zpoly`, `padd/pneg/psub/scale/shiftk/pad`, recursive
  `pmul` (math-comp style, NOT convolution — no summation machinery), the
  peqv framework (refl/sym/trans), total coefficient-characterization
  lemmas, pointwise algebra (comm/assoc/cong/zpoly-absorb/drop-last/
  precover).
- `src/poly_mul.rs` — the inductive workhorses: `lemma_pmul_push`
  (p.push(c)·q ≡ p·q + x^len(p)·(c·q)) and `lemma_pmul_pad` (trailing
  syntactic zeros in the first factor absorb), plus shiftk
  compose/cong/padd-distribution helpers.
- `src/poly_div.rs` — `divmod(a, b)`: a ≡ q·b + r (peqv), len(r) < len(b),
  exact quotient length, requiring only a non-eqv-zero leading coefficient
  on b. The quotient is built **positionally** (`pad(q1, s).push(f)`), so
  correctness needs neither commutativity nor associativity nor general
  distributivity of pmul.
- `build-export.sh` — cross-crate export (.vir verification artifact +
  ghost-erased .rlib), gt pattern; `export/` gitignored.

In this repo: `Cargo.toml`/`check.sh` wired to the tactus-algebra export
(`--import/--extern`); `src/probe.rs` proves a generic Ring lemma
instantiated at the imported Rational (cross-crate path validated).

**Idioms discovered (reuse these):**
- `by (nonlinear_arith)` blocks see ONLY their `requires` clauses and do not
  unfold spec fns — pass every definitional closed form explicitly.
- "rlimit exceeded" under the Lean backend = maxHeartbeats; fix by
  extracting an int-atom helper lemma and splitting into single-product
  steps, never by raising limits.
- Total-coeff characterization lemmas (uniform, no case splits at use
  sites) make every pointwise poly proof a mechanical eqv-chain.
- Long eqv-chains: let-bind intermediates, one `axiom_eqv_transitive` per
  link. Verbose but the Lean backend discharges each link first-try.
