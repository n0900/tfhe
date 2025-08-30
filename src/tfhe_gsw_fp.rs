// use crate::{error_sampling::ErrorSampling, field::Fp, gsw::{FheScheme, GSW}, zo_sss::{Party, SecretSharingScheme}, TfheScheme, TfheStructure};


// /// Implements GSW with arbitrary SSS scheme and arbitrary error distribition over Fp
// impl<S, T> TfheScheme<Fp> for TfheStructure<Fp, S, GSW<Fp, T>>
// where
//     S: SecretSharingScheme<Fp>,
//     T: ErrorSampling<Fp>,
// {
//     type SecretKey = <GSW<Fp, T> as FheScheme<Fp>>::SecretKey;
//     type PublicKey = <GSW<Fp,T> as FheScheme<Fp>>::PublicKey;
//     type Ciphertext = <GSW<Fp,T> as FheScheme<Fp>>::Ciphertext;

//     fn setup(&self) -> (Vec<Party<Fp>>, Self::PublicKey) {
//         let (sk, pk) = self.fhe_scheme.keygen();
//         let parties = self.secret_sharing_scheme.share(sk.s);
//         (parties, pk)
//     }

//     /// # Parameters:
//     ///  - `pk`: Public Key
//     ///  - `message`: Ring element. Must be either Fp::ZERO or Fp::ONE (After Boneh et al.)
//     fn encrypt(&self, pk: &Self::PublicKey, message: Fp) -> Self::Ciphertext {
//         self.fhe_scheme.encrypt(pk, message)
//     }

//     fn part_dec(&self, pk: &Self::PublicKey, ciphertext: Self::Ciphertext, party: Party<Fp>) {
//         todo!()
//     } 

//     fn add(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext {
//         self.fhe_scheme.add(ciphertext1, ciphertext2)
//     }
    
//     fn mult_const(&self, ciphertext: &mut Self::Ciphertext, constant: Fp) {
//         self.fhe_scheme.mult_const(ciphertext, constant)
//     }

//     fn mult(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext {
//         self.fhe_scheme.mult(ciphertext1, ciphertext2)
//     }

//     fn nand(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext {
//         self.fhe_scheme.nand(ciphertext1, ciphertext2)
//     }
// }