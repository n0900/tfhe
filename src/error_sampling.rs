use crate::{field::Fp, misc::{rnd_fp, rnd_fp_vec}};

pub trait ErrorSampling {
    fn rnd_fp() -> Fp;
    fn rnd_fp_vec(size: usize) -> Vec<Fp>;
}

pub struct NaiveSampling;

impl ErrorSampling for NaiveSampling {
    fn rnd_fp() -> Fp {
        rnd_fp(0, 10);
    }

    fn rnd_fp_vec(size: usize) -> Vec<Fp> {
        rnd_fp_vec(size, 0, 10);
    }
}