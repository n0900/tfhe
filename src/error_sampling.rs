use crate::{field::P, RingElement};

use nalgebra::{DVector, DMatrix};
use num_bigint::{BigUint, Sign};
use num_rational::Ratio;
use prio::dp::distributions::DiscreteGaussian;
use rand::{distr::Distribution, rng, Rng};

const NOISE_CONST: u64 = 1u64;


pub fn rnd_dmatrix<R: RingElement + 'static>(nrows: usize, ncols: usize, min: u64, max: u64) -> DMatrix<R> {
    DMatrix::from_fn(nrows, ncols, |_, _| rnd_ring_elm(min, max))
}

pub fn rnd_dvec<R: RingElement + 'static>(size: usize, min: u64, max: u64) -> DVector<R> {
    DVector::from_fn(size,  |_,_| rnd_ring_elm(min, max))
}

pub fn rnd_ring_elm<R: RingElement>(min: u64, max: u64) -> R {
    assert!(max <= P);
    let mut rng = rand::rng();
    R::from(rng.random_range(min..=max))
}

//Do not use for sampling random numbers as domain of error functions is restricted!
pub trait ErrorSampling<R: RingElement> {
    fn rnd_error_elm(&self) -> R;
    fn rnd_error_dvec(&self, size: usize) -> DVector<R>;
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
            sampler: DiscreteGaussian::new(Ratio::<BigUint>::new(BigUint::from(2u32), BigUint::from(1u32))).unwrap()
        }
    }
}

impl<R: RingElement+ 'static> ErrorSampling<R> for DiscrGaussianSampler {
    fn rnd_error_elm(&self) -> R {
        let mut rng = rng();
        let (sign, digits) = self.sampler.sample(&mut rng).to_u64_digits();
        let noise_const = R::from(NOISE_CONST);

        assert!(digits.len()<=1);
        
        match sign {
            Sign::Minus => -(R::from(*digits.first().unwrap()) * noise_const),
            Sign::NoSign => R::zero(),
            Sign::Plus => R::from(*digits.first().unwrap()) * noise_const,
        }
    } 

    fn rnd_error_dvec(&self, size: usize) -> DVector<R> {
        DVector::from_iterator(size, (0..size).map(|_| Self::rnd_error_elm(self)))
    }

}

pub struct NaiveSampler;

impl<R: RingElement + 'static> ErrorSampling<R> for NaiveSampler {
    fn rnd_error_elm(&self) -> R {
        let noise_const = R::from(NOISE_CONST);
        rnd_ring_elm::<R>(0, P/4) * noise_const
    }

    fn rnd_error_dvec(&self, size: usize) -> DVector<R> {
        DVector::from_fn(size,  |_,_| self.rnd_error_elm())
    }
}

#[cfg(test)]
mod test {

    use nalgebra::DVector;

    use crate::{error_sampling::{DiscrGaussianSampler, ErrorSampling}, field::Fp};

    #[test]
    fn gaussian_test() {
        // check that random numbers are not all equal
        let gaussian = DiscrGaussianSampler::default();
        let rnd_vec: DVector<Fp> = gaussian.rnd_error_dvec(100);
        println!("First 5 samples: {:?}", &rnd_vec.as_slice()[..5]);

        assert_eq!(rnd_vec.len(), 100);
        let first = &rnd_vec[0];
        let all_same = rnd_vec.iter().all(|x| x == first);
        assert!(!all_same, "All sampled values are identical — Gaussian sampler may be broken");
    }
}