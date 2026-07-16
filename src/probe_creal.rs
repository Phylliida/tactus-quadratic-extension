///  Derisking probe for cad-07 (Cauchy-real model). Throwaway — findings
///  get recorded in board/cad-07-cauchy-model.md.
///
///  Questions this probe answers:
///  1. Does tactus accept a ghost struct with a `spec_fn(nat) -> Rational`
///     field, and spec-closure construction of one?
///  2. Do quantifier-heavy predicates over the sequence (Bishop-regularity)
///     state and prove cleanly (trigger via a named accessor)?
///  3. Does pointwise arithmetic on such sequences (cr_add as a closure
///     reading two other closures) work end-to-end in a small proof?
use vstd::prelude::*;
use tactus_algebra::rational::Rational;
use tactus_algebra::traits::equivalence::Equivalence;
use tactus_algebra::traits::additive_commutative_monoid::AdditiveCommutativeMonoid;

verus! {

//  ---- Q1: spec_fn field ----

pub ghost struct CReal {
    pub at: spec_fn(nat) -> Rational,
}

///  Named accessor — quantify over THIS, never over the raw closure
///  application (workspace idiom: spec_fn applications make bad triggers).
pub open spec fn cr_at(x: CReal, n: nat) -> Rational {
    (x.at)(n)
}

///  |a| for the bound statements.
pub open spec fn rabs(a: Rational) -> Rational {
    if a.num >= 0 { a } else { a.neg_spec() }
}

///  1/n as a Rational (n >= 1).
pub open spec fn rinv_nat(n: nat) -> Rational
    recommends n >= 1,
{
    Rational { num: 1, den: (n - 1) as nat }
}

//  ---- Q2: Bishop regularity as a quantified predicate ----

///  Bishop-regular sequence: |x_m - x_n| <= 1/m + 1/n for m, n >= 1.
pub open spec fn regular(x: CReal) -> bool {
    forall|m: nat, n: nat| m >= 1 && n >= 1 ==>
        rabs(#[trigger] cr_at(x, m).sub_spec(cr_at(x, n)))
            .le_spec(rinv_nat(m).add_spec(rinv_nat(n)))
}

//  ---- Q3: construction + a small end-to-end proof ----

///  Embed a rational as the constant sequence.
pub open spec fn cr_const(q: Rational) -> CReal {
    CReal { at: |n: nat| q }
}

///  Pointwise sum with Bishop's index doubling (regularity-preserving).
pub open spec fn cr_add(x: CReal, y: CReal) -> CReal {
    CReal { at: |n: nat| cr_at(x, 2 * n).add_spec(cr_at(y, 2 * n)) }
}

///  The constant sequence is regular: |q - q| = 0 <= 1/m + 1/n.
pub proof fn probe_const_regular(q: Rational)
    ensures regular(cr_const(q)),
{
    assert forall|m: nat, n: nat| m >= 1 && n >= 1 implies
        rabs(#[trigger] cr_at(cr_const(q), m).sub_spec(cr_at(cr_const(q), n)))
            .le_spec(rinv_nat(m).add_spec(rinv_nat(n)))
    by {
        let d = q.sub_spec(q);
        //  d = q + (-q): num is q.num*dq - q.num*dq = 0, so |d| = d with num 0.
        assert(d.num == q.num * q.denom() + (-q.num) * q.denom());
        assert(d.num == 0) by (nonlinear_arith)
            requires d.num == q.num * q.denom() + (-q.num) * q.denom(),
        ;
        assert(rabs(d) == d);
        //  goal: 0/dd <= 1/m + 1/n, i.e. 0 * denom(sum) <= sum.num * dd.
        let s = rinv_nat(m).add_spec(rinv_nat(n));
        //  s.num = 1*n + 1*m > 0, both denominators positive.
        assert(s.num == 1 * (n as int) + 1 * (m as int));
        assert(s.num > 0) by (nonlinear_arith)
            requires s.num == 1 * (n as int) + 1 * (m as int), m >= 1, n >= 1,
        ;
        assert(d.num * s.denom() <= s.num * d.denom()) by (nonlinear_arith)
            requires d.num == 0, s.num > 0, s.denom() > 0, d.denom() > 0,
        ;
    }
}

///  cr_add reads its arguments where it should (closure plumbing check).
pub proof fn probe_add_pointwise(x: CReal, y: CReal, n: nat)
    ensures cr_at(cr_add(x, y), n) == cr_at(x, 2 * n).add_spec(cr_at(y, 2 * n)),
{
}

} //  verus!
