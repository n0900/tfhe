use ff::{derive::{bitvec::{array::BitArray}, subtle::ConstantTimeEq}, Field, PrimeField, PrimeFieldBits};
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

// Provide num-traits Zero/One (nalgebra expects num_traits types)
impl num_traits::Zero for Fp {
    fn zero() -> Self { Fp::ZERO }
    fn is_zero(&self) -> bool { self.ct_eq(&Self::ZERO).unwrap_u8() != 0 }

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
    use ff::{Field, PrimeFieldBits};
    use nalgebra::{DMatrix, DVector};
    use crate::{error_sampling::rnd_ring_elm, field::{Fp, P}};

    // small heuristic to verify generator
    #[test]
    fn inverse_test() {
        let one = Fp::ONE;
        let minus_one = -one;
        assert_eq!(one+minus_one, Fp::ZERO);

        for _ in 1..100000 {
            let rnd = rnd_ring_elm::<Fp>(1, P-1);
            let inverse = rnd.invert().unwrap();
            assert_eq!(rnd * inverse, Fp::ONE);
        }
    }

    #[test]
    fn test_fp_to_u64() {
        for _ in 1..100 {
            let rnd: Fp = rnd_ring_elm(0, P-1);
            let rnd_u64 = u64::from_le_bytes(rnd.to_le_bits().data);
            let rnd_reconstr = Fp::from(rnd_u64);
            assert_eq!(rnd, rnd_reconstr);
        }
    }


    #[test]
    fn test_scalar_product_fp() {
        let a = vec![Fp::from(1), Fp::from(2), Fp::from(3)];
        let b = vec![Fp::from(4), Fp::from(5), Fp::from(6)];
        let result = DVector::from_vec(a).dot(&DVector::from_vec(b));
        assert_eq!(result, Fp::from(32)); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_matrix_vector_fp() {
        // DMatrix fills columns first so this matrix is actually
        // ( 1,2,3 )
        // ( 4,5,6 )
        let matrix = DMatrix::from_vec(2, 3, 
            vec![Fp::from(1), Fp::from(4), Fp::from(2), 
                      Fp::from(5), Fp::from(3), Fp::from(6)]);

        let vector = DVector::from_vec(vec![Fp::from(7), Fp::from(8), Fp::from(9)]);
        
        let result = matrix * vector;
        let expected = DVector::from_vec(vec![
            Fp::from(50), // 1*7 + 2*8 + 3*9
            Fp::from(122), // 4*7 + 5*8 + 6*9
        ]);
        assert_eq!(result, expected);
    }

}