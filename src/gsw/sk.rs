use ff::Field;
use nalgebra::DVector;

use crate::field::{Fp};
use crate::gsw::helper::powers_of_2;
use crate::gsw::{build_gadget_vector, RingElement};


/// Contains all components of a private key. 
/// t n-dim random vector \in \mathbb(Z)_p
/// s (1, -t_1, -t_2,...) \in \mathbb(Z)_p
/// v = powers_of_two(s)
pub struct GswSk<R: RingElement> {
    pub t: DVector<R>,
    pub s: DVector<R>,
    pub v: DVector<R>,
}

impl GswSk<Fp> {
    pub fn new(t: DVector<Fp>) -> Self {
        let mut s = DVector::zeros(t.len() + 1);
        s[0] = Fp::ONE;

        s.rows_mut(1, t.len()).copy_from(&(-t.clone()));

        let v = powers_of_2(&s, &build_gadget_vector());
        Self { t, s, v }
    }
}