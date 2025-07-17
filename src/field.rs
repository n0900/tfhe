use ff::PrimeField;

#[derive(PrimeField)]
#[PrimeFieldModulus = "2147483647"]
#[PrimeFieldGenerator = "3"]
#[PrimeFieldReprEndianness = "little"]

pub struct Fp([u64; 1]);