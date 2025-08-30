use ff::{derive::{bitvec::{array::BitArray, field}, subtle::ConstantTimeEq}, Field, PrimeField, PrimeFieldBits};
use nalgebra::DVector;
use once_cell::sync::Lazy;

use crate::RingElement;

// 32-ish bit prime
#[derive(PrimeField)]
#[PrimeFieldModulus = "2147483647"]
#[PrimeFieldGenerator = "3"] //I did not verify this number
#[PrimeFieldReprEndianness = "little"]

// 64-ish bit prime
// #[derive(PrimeField)]
// #[PrimeFieldModulus = "1152921504606847009"]
// #[PrimeFieldGenerator = "2"] //I did not verify this number
// #[PrimeFieldReprEndianness = "little"]

pub struct Fp([u64; 1]);
pub const P: u64 = 2147483647;

pub static GADGET_VECTOR: Lazy<DVector<Fp>> = Lazy::new(|| {
    DVector::from_vec((0..Fp::NUM_BITS)
        .map(|l| Fp::from(1u64 << l))
        .collect())
});


// Provide num-traits Zero/One (nalgebra expects num_traits types)
impl num_traits::Zero for Fp {
    fn zero() -> Self { Fp::ZERO }
    fn is_zero(&self) -> bool { self.ct_eq(&Self::ZERO).unwrap_u8() == 0 }
}
impl num_traits::One for Fp {
    fn one() -> Self { Fp::ONE }
}

impl RingElement for Fp {
    fn to_le_bits(&self) -> BitArray<[u8;8]> {
        PrimeFieldBits::to_le_bits(self)
    }
    const Num_Bits: usize = Fp::NUM_BITS as usize;
}

#[cfg(test)]
mod tests {
    use ff::{Field};
    use crate::{error_sampling::rnd_fp, field::{Fp, P}};

    // small heuristic to verify generator
    #[test]
    fn inverse_test() {
        let one = Fp::ONE;
        let minus_one = -one;
        assert_eq!(one+minus_one, Fp::ZERO);

        for _ in 1..100000 {
            let rnd = rnd_fp(1, P-1);
            let inverse = rnd.invert().unwrap();
            assert_eq!(rnd * inverse, Fp::ONE);
        }
    }
}