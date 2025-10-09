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



#[cfg(test)]
mod tests {
    use crate::{error_sampling::rnd_dvec, field::Fp, gsw::sk::GswSk, RingElement};

    #[test]
    fn test_v_decomp() {
        let sk = GswSk::new(rnd_dvec(5, 0, 10));

        for i in 0..Fp::Num_Bits {
            assert_eq!(sk.v[i], Fp::from(1<<i));
        }
    }
}