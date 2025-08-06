pub mod sk;
pub mod pk;
pub mod gsw;

use std::ops::Mul;

use ff::{Field, PrimeField};

use crate::{
    error_sampling::{rnd_fp_vec, ErrorSampling, NaiveSampler}, field::{Fp, L, P}, gsw::{gsw::{
        add_matrix_matrix_fp, add_to_diagonal, bit_decomp_matrix, dot_product_fp, flatten_matrix, mult_const_matrix_fp, mult_matrix_matrix_fp, negate_matrix_fp
    }, pk::GswPk, sk::GswSk}
};

pub trait FheScheme {
    type SecretKey;
    type PublicKey;
    type Ciphertext;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey);
    fn encrypt(&self, pk: &GswPk, message: Fp) -> Self::Ciphertext;
    fn decrypt(&self, sk: &GswSk, ciphertext: &Self::Ciphertext) -> Fp;
    fn add(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
    fn mult_const(&self, ciphertext: &Self::Ciphertext, constant: Fp) -> Self::Ciphertext;
    fn mult(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
    fn nand(&self, ciphertext1: &Self::Ciphertext, ciphertext2: &Self::Ciphertext) -> Self::Ciphertext;
}

pub struct GSW<T: ErrorSampling> {
    n: u8,
    m: u8,
    err_sampling: T,
}

pub static NAIVE_GSW: GSW<NaiveSampler> = GSW { n: 10, m: 10, err_sampling: NaiveSampler{} };

impl<T: ErrorSampling> FheScheme for GSW<T> {
    type SecretKey = GswSk;
    type PublicKey = GswPk;
    type Ciphertext = Vec<Vec<Fp>>;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey) {
        let sk = GswSk::new(rnd_fp_vec(self.n as usize, 0, P-1));  
          
        let err = self.err_sampling.rnd_fp_vec(self.m as usize);    
        let random_matrix: Vec<Vec<Fp>> = (0..err.len())
            .map(|_| rnd_fp_vec(self.n as usize, 0, P-1))
            .collect();
        
        let pk = GswPk::new(&random_matrix, &err, &sk.t);
        (sk, pk)   
    }

    fn encrypt(&self, pk: &Self::PublicKey, message: Fp) -> Self::Ciphertext {
        let big_n: usize = L.mul((self.n + 1) as usize);
        let random_matrix = (0..big_n).map(|_| rnd_fp_vec(self.m as usize, 0, 1)).collect();
        flatten_matrix(
            &add_to_diagonal(
                &bit_decomp_matrix(
                    &mult_matrix_matrix_fp(&random_matrix, &pk.pk_matrix)
                ),
                message
            )
        )
    }

    fn decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Fp {
        let i = 63 - (P / 3).leading_zeros() as usize; //efficient log2(P/3)
        let x_i = u64::from_le_bytes(dot_product_fp(&ciphertext[i], &sk.v).to_repr().0);
        let v_i = u64::from_le_bytes(sk.v[i].to_repr().0);
        Fp::from(x_i / v_i)
    }

    // flatten(C1+C2)
    fn add(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        assert_eq!(ciphertext1.len(), cipertext2.len(), "Cannot add Ciphertexts because they are different sizes");
        assert_eq!(ciphertext1.first().unwrap().len(), cipertext2.first().unwrap().len(), "Cannot add Ciphertexts because they are different sizes");
        flatten_matrix(&add_matrix_matrix_fp(ciphertext1, cipertext2))
    }

    // flatten(C*a)
    fn mult_const(&self, ciphertext: &Self::Ciphertext, constant: Fp) -> Self::Ciphertext {
        flatten_matrix(&mult_const_matrix_fp(&ciphertext, constant))
    }

    // flatten(C1*C2)
    fn mult(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        assert_eq!(ciphertext1.len(), cipertext2.len(), "Cannot mult Ciphertexts because they are different sizes");
        assert_eq!(ciphertext1.first().unwrap().len(), cipertext2.first().unwrap().len(), "Cannot mult Ciphertexts because they are different sizes");
        flatten_matrix(&mult_matrix_matrix_fp(ciphertext1, cipertext2))
    }

    // flatten(I - C1*C2)
    fn nand(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        flatten_matrix(
            &add_to_diagonal(
                &negate_matrix_fp(&mult_matrix_matrix_fp(ciphertext1, cipertext2)), 
                Fp::ONE)
            )
    }
}

#[cfg(test)]
mod tests {
    use ff::{Field};

    use crate::error_sampling::rnd_fp_vec;
    use crate::field::Fp;
    use crate::field::P;
    use crate::gsw::FheScheme;
    use crate::gsw::NAIVE_GSW;
    use crate::gsw::gsw::{mult_matrix_vector_fp};
    use crate::gsw::pk::GswPk;
    use crate::gsw::sk::GswSk;

    #[test]
    fn sk_pk_invariant() {
        let n = 10;
        let m = 5;

        let sk = GswSk::new(rnd_fp_vec(n, 0, P-1));
        let err = rnd_fp_vec(m, 0, 10);
        let random_matrix: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_vec(n, 0, P-1)).collect();
        let pk = GswPk::new(&random_matrix, &err, &sk.t);
        let invariant = mult_matrix_vector_fp(&pk.pk_matrix, &sk.s);

        // As = e
        assert_eq!(invariant, err);
    }

    #[test]
    fn encryption_decryption() {
        
        let (sk, pk) = NAIVE_GSW.keygen();

        let encr = NAIVE_GSW.encrypt(&pk, Fp::ZERO);
        let decr = NAIVE_GSW.decrypt(&sk, &encr);
        assert_eq!(decr, Fp::ZERO);

        let encr = NAIVE_GSW.encrypt(&pk, Fp::ONE);
        let decr = NAIVE_GSW.decrypt(&sk, &encr);
        assert_eq!(decr, Fp::ONE);
    }
}
