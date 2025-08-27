use crate::error_sampling::ErrorSampling;

pub mod sk;
pub mod pk;
pub mod gsw;
pub mod gsw_fp;

pub trait FheScheme {
    type SecretKey;
    type PublicKey;
    type Message;
    type Constant;
    type Ciphertext;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey);
    fn encrypt(&self, pk: &Self::PublicKey, message: Self::Message) -> Self::Ciphertext;
    fn decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Self::Message;
    fn mp_decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Self::Message;

    fn add(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
    fn mult_const(&self, ciphertext: &mut Self::Ciphertext, constant: Self::Constant);
    fn mult(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
    fn nand(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GSW<T: ErrorSampling> {
    n: usize,
    m: usize,
    err_sampling: T,
}
