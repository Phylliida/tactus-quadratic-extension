///  Cross-crate probe: exercises the tactus-algebra import end-to-end —
///  a generic lemma over the trait ladder, instantiated at the Rational
///  OrderedField instance. Deleted once real modules land (M1).
use vstd::prelude::*;
use tactus_algebra::traits::*;
use tactus_algebra::rational::Rational;

verus! {

///  Left additive identity, derived generically from the Ring axioms.
pub proof fn lemma_zero_add_left<T: Ring>(a: T)
    ensures T::zero().add(a).eqv(a),
{
    T::axiom_add_commutative(T::zero(), a);
    T::axiom_add_zero_right(a);
    T::axiom_eqv_transitive(T::zero().add(a), a.add(T::zero()), a);
}

///  The generic lemma instantiated at the imported Rational instance.
pub proof fn probe_rational_instance() {
    let a = Rational { num: 3, den: 1 };
    lemma_zero_add_left::<Rational>(a);
    assert(Rational::zero().add(a).eqv(a));
}

} //  verus!
