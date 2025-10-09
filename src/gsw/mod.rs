use std::marker::PhantomData;

use nalgebra::{DMatrix, DVector};

use crate::{error_sampling::{ErrorSampling, NaiveSampler}, field::{Fp}, RingElement};

pub mod sk;
pub mod pk;
pub mod helper;
pub mod gsw_fp;
pub mod gsw_boneh;
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


#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GSW<R,T>
where 
 R: RingElement, T: ErrorSampling<R> 
{
    n: usize,
    m: usize,
    err_sampling: T,
    _marker: PhantomData<R>
}

const EXAMPLE_GSW: GSW<Fp, NaiveSampler> = GSW::<Fp, NaiveSampler> {
        n: 10,
        m: 10 * Fp::Num_Bits,
        err_sampling: NaiveSampler,
        _marker: PhantomData,
    };

fn build_gadget_matrix<R: RingElement + 'static>(n: usize, m: usize) -> DMatrix<R> {
    // Result of Boneh's definition
    assert!(m%R::Num_Bits == 0, 
        "m must be a multiple of R::Num_Bits"); 
    
    let gadget_vec = build_gadget_vector().transpose();
    println!("{:?}", gadget_vec);
    // Kronecker Product: nxn \cdot 1xNum_Bits = n*1 x n * Num_bits = nxm
    DMatrix::identity(n, n).kronecker(&gadget_vec)
}

fn build_gadget_vector<R: RingElement + 'static>() -> DVector<R> {
    DVector::from_vec((0..R::Num_Bits)
        .map(|l| R::from(1u64 << l))
        .collect())
}