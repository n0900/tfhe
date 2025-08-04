use crate::{field::Fp, gsw::pk::GswPk, zo_sss::Party};

pub mod field;
pub mod misc;
pub mod gsw;
pub mod zo_sss;

pub fn tfhe_setup() {
    todo!()
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