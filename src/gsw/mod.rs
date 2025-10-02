use std::marker::PhantomData;

use crate::{error_sampling::ErrorSampling, field::Fp, pow2_ring::Zpow2, RingElement};

pub mod sk;
pub mod pk;
pub mod helper;
pub mod gsw_fp;
pub mod gsw_zpow2;

pub trait FheScheme<R: RingElement> {
    type SecretKey;
    type PublicKey;
    type Ciphertext;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey);
    fn encrypt(&self, pk: &Self::PublicKey, message: R) -> Self::Ciphertext;
    fn decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> R;
    fn mp_decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> R;

    fn add(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
    fn mult_const(&self, ciphertext: &mut Self::Ciphertext, constant: R);
    fn mult(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
    fn nand(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GSW<R,T>
where 
 R: RingElement, T: ErrorSampling<R> 
{
    n: usize,
    m: usize,
    err_sampling: T,
    _marker: PhantomData<R>
}
