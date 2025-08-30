use std::{marker::PhantomData, ops::{Add, AddAssign, Mul, MulAssign, Neg, Sub, SubAssign}};

use ff::{derive::bitvec::array::BitArray, PrimeField, PrimeFieldBits};

use crate::{error_sampling::ErrorSampling, field::Fp, gsw::{FheScheme, GSW}, pow2_ring::Zpow2, zo_sss::{Party, SecretSharingScheme}};

pub mod field;
pub mod gsw;
pub mod zo_sss;
pub mod error_sampling;
pub mod pow2_ring;
pub mod tfhe_gsw_fp;

pub trait RingElement:
    Clone + Copy + PartialEq + Eq + std::fmt::Debug 
    + Add + Sub + Mul + Neg
    + AddAssign + SubAssign + MulAssign
    + num_traits::Zero + num_traits::One
    + From<u64>
{
    fn to_le_bits(&self) -> BitArray<[u8;8]>;
    const Num_Bits: usize;
}


/// The TFHE scheme is fully described by
/// the Ring over which it operates,
/// the secret sharing scheme and 
/// the FHE scheme it uses
pub struct TfheStructure<R, S, E>
where
    R: RingElement,
    S: SecretSharingScheme<R>,
    E: FheScheme<R>,
{
    pub secret_sharing_scheme: S,
    pub fhe_scheme: E,
    _marker: PhantomData<R>
}

/// Implements all TFHE funktionality as described by Boneh et al
pub trait TfheScheme<R: RingElement> {
    type SecretKey;
    type PublicKey;
    type Ciphertext;

    fn setup(&self) -> (Vec<Party<R>>, Self::PublicKey);
    fn encrypt(&self, pk: &Self::PublicKey, message: Fp) -> Self::Ciphertext;
    fn part_dec(&self, pk: &Self::PublicKey, ciphertext: Self::Ciphertext, party: Party<R>);
    // fn fin_dec();
    
    fn add(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
    fn mult_const(&self, ciphertext: &mut Self::Ciphertext, constant: Fp);
    fn mult(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
    fn nand(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
}
