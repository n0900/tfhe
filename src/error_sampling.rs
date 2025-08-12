use crate::{field::{Fp, P}};

use ff::Field;
use num_bigint::{BigUint, Sign};
use num_rational::Ratio;
use once_cell::sync::Lazy;
use prio::dp::distributions::DiscreteGaussian;
use rand::{distr::Distribution, rng, Rng};

pub const NOISE_CONST: Lazy<Fp> = Lazy::new(||Fp::from(5));
pub const NOISE_CONST_INV: Lazy<Fp> = Lazy::new(||Fp::from(5).invert().unwrap());

pub fn rnd_fp_vec(size: usize, min: u64, max: u64) -> Vec<Fp> {
    (0..size).map(|_| rnd_fp(min, max)).collect()
}

pub fn rnd_fp(min: u64, max: u64) -> Fp {
    assert!(max <= P);
    let mut rng = rand::rng();
    Fp::from(rng.random_range(min..max))
}

pub trait ErrorSampling {
    fn rnd_fp(&self) -> Fp;
    fn rnd_fp_vec(&self, size: usize) -> Vec<Fp>;
}

pub struct DiscrGaussianSampler {
    sampler: DiscreteGaussian,
}

impl DiscrGaussianSampler {
    pub fn new(stddev: Ratio<BigUint>) -> Self {
        let sampler = DiscreteGaussian::new(stddev).expect("Failed to create DiscreteGaussian");
        Self {
            sampler,
        }
    }

    //completely made up number
    pub fn default() -> Self {
        Self {
            sampler: DiscreteGaussian::new(Ratio::<BigUint>::new(BigUint::from(15u32), BigUint::from(1u32))).unwrap()
        }
    }
}

impl ErrorSampling for DiscrGaussianSampler {
    fn rnd_fp(&self) -> Fp {
        let mut rng = rng();
        let (sign, digits) = self.sampler.sample(&mut rng).to_u64_digits();
        assert!(digits.len()<=1);
        
        match sign {
            Sign::Minus => -Fp::from(*digits.first().unwrap()) * *NOISE_CONST,
            Sign::NoSign => Fp::ZERO,
            Sign::Plus => Fp::from(*digits.first().unwrap()) * *NOISE_CONST,
        }
    } 

    fn rnd_fp_vec(&self, size: usize) -> Vec<Fp> {
        (0..size).map(|_| Self::rnd_fp(self)).collect()
    }
}

pub struct NaiveSampler;

impl ErrorSampling for NaiveSampler {
    fn rnd_fp(&self) -> Fp {
        let mut rng = rng();
        Fp::from(rng.random_range(0..P/2)) * *NOISE_CONST
    }

    fn rnd_fp_vec(&self, size: usize) -> Vec<Fp> {
        (0..size).map(|_| Self::rnd_fp(self)).collect()
    }
}

#[cfg(test)]
mod test {

    use crate::{error_sampling::{DiscrGaussianSampler, ErrorSampling}};

    #[test]
    fn gaussian_test() {
        // check that random numbers are not all equal
        let gaussian = DiscrGaussianSampler::default();
        let rnd_vec = gaussian.rnd_fp_vec(100);
        println!("First 5 samples: {:?}", &rnd_vec[..10.min(rnd_vec.len())]);

        assert_eq!(rnd_vec.len(), 100);
        let first = &rnd_vec[0];
        let all_same = rnd_vec.iter().all(|x| x == first);
        assert!(!all_same, "All sampled values are identical — Gaussian sampler may be broken");
    }
}