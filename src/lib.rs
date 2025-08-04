use crate::{error_sampling::{ErrorSampling}, field::Fp, gsw::{pk::GswPk, FheScheme, GSW}, zo_sss::{dimacs::{DIMACS}, Party, SecretSharingScheme}};

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
    type PublicKey;
    fn setup(&self) -> (Vec<Party>, Self::PublicKey);
    // fn encrypt();
    // fn part_dec();
    // fn fin_dec();
    // fn eval();
}

/// Implements GSW with arbitrary SSS scheme and arbitrary error distribition
impl<S, T> TfheScheme for TfheStructure<S, GSW<T>>
where
    S: SecretSharingScheme,
    T: ErrorSampling,
{
    type PublicKey = GswPk;

    fn setup(&self) -> (Vec<Party>, <GSW<T> as FheScheme>::PublicKey) {
        let (sk, pk) = self.fhe_scheme.keygen();
        let parties = self.secret_sharing_scheme.share(sk.s);
        (parties, pk)
    }
}

/// # Parameters:
///  - `pk`: Public Key
///  - `message`: Fp element. Must be either Fp::ZERO or Fp::ONE (After Boneh et al.)
pub fn tfhe_encrypt(pk: GswPk, message: Fp) -> Vec<Vec<Fp>> {
    todo!()
}

pub fn tfhe_part_decrypt(pk: GswPk, ciphertext: Vec<Vec<Fp>>, secret_key_share: Vec<Fp>) {
    todo!()
}

pub fn tfhe_final_decrypt(pk: GswPk, parties: Vec<Party>) -> Fp {
    todo!()
}

pub fn tfhe_eval() {
    todo!()
}