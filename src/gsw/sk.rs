use ff::Field;

use crate::field::Fp;
use crate::gsw::gsw::powers_of_2;
use crate::gsw::RingElement;


/// Contains all components of a private key. 
/// t n-dim random vector \in \mathbb(Z)_p
/// s (1, -t_1, -t_2,...) \in \mathbb(Z)_p
/// v = powers_of_two(s)
pub struct GswSk<R: RingElement> {
    pub t: Vec<R>,
    pub s: Vec<R>,
    pub v: Vec<R>,
}

impl GswSk<Fp> {
    pub fn new(t: Vec<Fp>) -> Self {
        let mut s = Vec::with_capacity(t.len() + 1);
        s.push(Fp::ONE);
        for x in &t {
            s.push(-*x);
        }
        let v = powers_of_2(&s);
        Self { t, s, v }
    }
}