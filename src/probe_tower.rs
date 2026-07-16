///  Derisking probe for cad-04 (tower core). Throwaway — findings get
///  recorded in board/cad-04-tower-core.md.
///
///  Questions this probe answers:
///  1. Does tactus accept a ghost enum recursive THROUGH a Seq
///     (`Poly(Seq<TowerElem>)`)?
///  2. Do recursive spec fns over it work with container-decreases, with
///     the recursive call under a `forall`?
///  3. Does a nested-induction proof fn (recursive call on `xs[i]` inside
///     an assert-forall) discharge under the Lean backend?
///  4. Does the generic poly layer (padd/peqv/divmod) instantiate
///     cross-crate at T = Rational?
use vstd::prelude::*;
use tactus_algebra::rational::Rational;
use tactus_algebra::poly::*;
use tactus_algebra::poly_div::divmod;
use tactus_algebra::traits::equivalence::Equivalence;
use tactus_algebra::traits::additive_commutative_monoid::AdditiveCommutativeMonoid;

verus! {

//  ---- Q1: the recursive-through-Seq datatype ----

pub ghost enum TowerElem {
    Base(Rational),
    Poly(Seq<TowerElem>),
}

//  ---- Q2: recursive spec fn, container decreases, call under forall ----

///  Same-shape structural equivalence (cross-depth broadcasting is cad-04
///  detail work; the probe only needs the recursion mechanics).
pub open spec fn te_eqv(a: TowerElem, b: TowerElem) -> bool
    decreases a,
{
    match (a, b) {
        (TowerElem::Base(x), TowerElem::Base(y)) => x.eqv_spec(y),
        (TowerElem::Poly(xs), TowerElem::Poly(ys)) => {
            xs.len() == ys.len()
                && (forall|i: int| 0 <= i < xs.len() ==> te_eqv(#[trigger] xs[i], ys[i]))
        },
        _ => false,
    }
}

//  ---- Q3: nested induction in a proof fn ----

pub proof fn te_eqv_refl(a: TowerElem)
    ensures te_eqv(a, a),
    decreases a,
{
    match a {
        TowerElem::Base(x) => {
            Rational::axiom_eqv_reflexive(x);
        },
        TowerElem::Poly(xs) => {
            assert forall|i: int| 0 <= i < xs.len() implies te_eqv(#[trigger] xs[i], xs[i]) by {
                te_eqv_refl(xs[i]);
            }
        },
    }
}

//  ---- Q4: cross-crate generic instantiation of the poly layer ----

pub proof fn probe_poly_layer_at_rational() {
    let one = Rational::from_int_spec(1);
    let two = Rational::from_int_spec(2);
    let three = Rational::from_int_spec(3);
    let p: Seq<Rational> = seq![one, two];
    let q: Seq<Rational> = seq![three];
    //  generic pointwise lemma instantiated cross-crate
    lemma_padd_comm::<Rational>(p, q);
    assert(peqv(padd(p, q), padd(q, p)));
    //  divmod instantiated at Rational: divide [1,2] by the constant poly [3]
    assert(q.last() == three);
    assert(three.num == 3 && three.denom() == 1);
    assert(Rational::from_int_spec(0).num == 0 && Rational::from_int_spec(0).denom() == 1);
    assert(3int * 1 != 0int * 1);
    assert(!three.eqv_spec(Rational::from_int_spec(0)));
    let (qq, r) = divmod::<Rational>(p, q);
    assert(r.len() < 1);
    assert(peqv(p, padd(pmul(qq, q), r)));
}

} //  verus!
