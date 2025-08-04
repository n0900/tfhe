use crate::{field::Fp, gsw::{gsw_keygen, pk::GswPk}, zo_sss::{dimacs::{DIMACS, DIMACS_2_OF_3_SCHEME}, Party, SecretSharingScheme}};

pub mod field;
pub mod misc;
pub mod gsw;
pub mod zo_sss;
pub mod error_sampling;

pub struct TfheScheme<S: SecretSharingScheme, E: FheScheme> {
    secret_sharing_scheme: S,
    fhe_scheme: E
}

pub fn tfhe_setup(dimacs_formula: &str, n: u8, m: u8) {
    let (sk, pk) = gsw_keygen(n, m);
    let dimacs = DIMACS::parse(&dimacs_formula);

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