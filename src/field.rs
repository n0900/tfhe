use ff::{PrimeField};
use once_cell::sync::Lazy;

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


// Assume NUM_BITS < usize::MAX else panic (u32 on 32-bit systems, u64 on 64-bit systems)
pub const L: usize = Fp::NUM_BITS as usize;

pub static GADGET_VECTOR: Lazy<Vec<Fp>> = Lazy::new(|| {
    (0..Fp::NUM_BITS)
        .map(|l| Fp::from(1u64 << l))
        .collect()
});

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