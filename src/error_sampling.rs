use crate::{field::Fp, misc::{rnd_fp, rnd_fp_vec}};

use ff::Field;
use num_bigint::{BigInt, BigUint, Sign};
use num_rational::Ratio;
use prio::dp::distributions::DiscreteGaussian;
use rand::{distr::Distribution, rng, rngs::ThreadRng, Rng};

pub trait ErrorSampling {
    fn rnd_fp(&self) -> Fp;
    fn rnd_fp_vec(&self, size: usize) -> Vec<Fp>;
}

pub struct DiscrGaussian {
    sampler: DiscreteGaussian,
}

impl DiscrGaussian {
    pub fn new(stddev: Ratio<BigUint>) -> Self {
        let sampler = DiscreteGaussian::new(stddev).expect("Failed to create DiscreteGaussian");
        Self {
            sampler,
        }
    }
}
impl ErrorSampling for DiscrGaussian {
    fn rnd_fp(&self) -> Fp {
        let mut rng = rng();
        let (sign, digits) = self.sampler.sample(&mut rng).to_u64_digits();
        assert!(digits.len()<=1);
        
        match sign {
            Sign::Minus => -Fp::from(*digits.first().unwrap()),
            Sign::NoSign => Fp::ZERO,
            Sign::Plus => Fp::from(*digits.first().unwrap()),
        }
    } 

    fn rnd_fp_vec(&self, size: usize) -> Vec<Fp> {
        (0..size).map(|_| Self::rnd_fp(self)).collect()
    }
}

pub struct NaiveSampling;

impl ErrorSampling for NaiveSampling {
    fn rnd_fp(&self) -> Fp {
        let mut rng = rng();
        Fp::from(rng.random_range(0..10))
    }

    fn rnd_fp_vec(&self, size: usize) -> Vec<Fp> {
        rnd_fp_vec(size, 0, 10)
    }
}

#[cfg(test)]
mod test {
    use num_bigint::BigUint;
    use num_rational::Ratio;

    use crate::{error_sampling::{DiscrGaussian, ErrorSampling}, misc::rnd_fp_vec};

    #[test]
    fn gaussian_test() {
        // check that random numbers are not all equal
        let mut gaussian = DiscrGaussian::new(Ratio::<BigUint>::new(BigUint::from(3u32), BigUint::from(1u32)));
        let rnd_vec = gaussian.rnd_fp_vec(100);
        println!("First 5 samples: {:?}", &rnd_vec[..10.min(rnd_vec.len())]);

        assert_eq!(rnd_vec.len(), 100);
        let first = &rnd_vec[0];
        let all_same = rnd_vec.iter().all(|x| x == first);
        assert!(!all_same, "All sampled values are identical — Gaussian sampler may be broken");
    }
}