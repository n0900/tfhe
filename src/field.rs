use ff::{PrimeField};
use once_cell::sync::Lazy;

#[derive(PrimeField)]
// 32-ish bit prime
// #[PrimeFieldModulus = "2147483647"]
// #[PrimeFieldGenerator = "3"]

// 64-ish bit prime
#[PrimeFieldModulus = "1152921504606846971"]
#[PrimeFieldGenerator = "2"]
#[PrimeFieldReprEndianness = "little"]

pub struct Fp([u64; 1]);

// Assume NUM_BITS < usize::MAX else panic (u32 on 32-bit systems, u64 on 64-bit systems)
pub const L: usize = Fp::NUM_BITS as usize;
pub static GADGET_VECTOR: Lazy<Vec<Fp>> = Lazy::new(|| {
    (0..Fp::NUM_BITS)
        .map(|l| Fp::from(1u64 << l))
        .collect()
});