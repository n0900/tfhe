use crate::{error_sampling::ErrorSampling, field::Fp, gsw::{pk::GswPk, sk::GswSk, FheScheme, GSW}, zo_sss::{dimacs::DIMACS, Party, SecretSharingScheme}};

pub mod field;
pub mod misc;
pub mod gsw;
pub mod zo_sss;
pub mod error_sampling;

pub struct TfheStructure<S: SecretSharingScheme, E: FheScheme> {
    pub secret_sharing_scheme: S,
    pub fhe_scheme: E
}

pub trait TfheScheme {
    type SecretKey;
    type PublicKey;
    type Ciphertext;

    fn setup(&self) -> (Vec<Party>, Self::PublicKey);
    fn encrypt(&self, pk: &Self::PublicKey, message: Fp) -> Self::Ciphertext;
    fn part_dec(&self, pk: &Self::PublicKey, ciphertext: Self::Ciphertext, party: Party);
    // fn fin_dec();
    // fn eval();
}

/// Implements GSW with arbitrary SSS scheme and arbitrary error distribition
impl<S, T> TfheScheme for TfheStructure<S, GSW<T>>
where
    S: SecretSharingScheme,
    T: ErrorSampling,
{
    type SecretKey = <GSW<T> as FheScheme>::SecretKey;
    type PublicKey = <GSW<T> as FheScheme>::PublicKey;
    type Ciphertext = <GSW<T> as FheScheme>::CipherText;

    fn setup(&self) -> (Vec<Party>, Self::PublicKey) {
        let (sk, pk) = self.fhe_scheme.keygen();
        let parties = self.secret_sharing_scheme.share(sk.s);
        (parties, pk)
    }

    /// # Parameters:
    ///  - `pk`: Public Key
    ///  - `message`: Fp element. Must be either Fp::ZERO or Fp::ONE (After Boneh et al.)
    fn encrypt(&self, pk: &Self::PublicKey, message: Fp) -> Self::Ciphertext {
        self.fhe_scheme.encrypt(pk, message)
    }

    fn part_dec(&self, pk: &Self::PublicKey, ciphertext: Self::Ciphertext, party: Party) {
        todo!()
    } 
}

pub fn tfhe_final_decrypt(pk: GswPk, parties: Vec<Party>) -> Fp {
    todo!()
}

pub fn tfhe_eval() {
    todo!()
}