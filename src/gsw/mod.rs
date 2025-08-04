pub mod sk;
pub mod pk;

use std::ops::Mul;

use ff::PrimeField;

use crate::{
    error_sampling::{ErrorSampling, NaiveSampling}, field::{Fp, L, P}, gsw::{pk::GswPk, sk::GswSk}, misc::{
        add_to_diagonal, bit_decomp_matrix, dot_product_fp, flatten_matrix, matrix_matrix_fp,
        rnd_fp_vec,
    }
};

pub trait FheScheme {
    type SecretKey;
    type PublicKey;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey);
    fn encrypt(&self, pk: &GswPk, message: Fp) -> Vec<Vec<Fp>>;
    fn decrypt(&self, sk: &GswSk, cipher_matrix: Vec<Vec<Fp>>) -> Fp;
    fn eval(); //TODO
}

pub struct GSW<T: ErrorSampling> {
    n: u8,
    m: u8,
    err_sampling: T,
}

pub static NAIVE_GSW: GSW<NaiveSampling> = GSW { n: 10, m: 10, err_sampling: NaiveSampling{} };

impl<T: ErrorSampling> FheScheme for GSW<T> {
    type SecretKey = GswSk;
    type PublicKey = GswPk;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey) {
        let sk = GswSk::new(rnd_fp_vec(self.n as usize, 0, P-1));
        let err = T::rnd_fp_vec(self.m as usize);
        let B: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_vec(self.n as usize, 0, P-1)).collect();
        let pk = GswPk::new(&B, &err, &sk.t);
        (sk, pk)   
    }

    fn encrypt(&self, pk: &GswPk, message: Fp) -> Vec<Vec<Fp>> {
        let N: usize = L.mul((self.n + 1) as usize);
        let R = (0..N).map(|_| rnd_fp_vec(self.m as usize, 0, 1)).collect();
        flatten_matrix(
            &add_to_diagonal(
                &bit_decomp_matrix(
                    &matrix_matrix_fp(&R, &pk.A)
                ),
                message
            )
        )
    }

    fn decrypt(&self, sk: &GswSk, cipher_matrix: Vec<Vec<Fp>>) -> Fp {
        let i = 63 - (P / 3).leading_zeros() as usize; //efficient log2(P/3)
        let x_i = u64::from_le_bytes(dot_product_fp(&cipher_matrix[i], &sk.v).to_repr().0);
        let v_i = u64::from_le_bytes(sk.v[i].to_repr().0);
        Fp::from(x_i / v_i)
    }

    fn eval() {
        
    }
}

#[cfg(test)]
mod tests {
    use ff::{Field};

    use crate::field::Fp;
    use crate::field::P;
    use crate::gsw::FheScheme;
    use crate::gsw::NAIVE_GSW;
    use crate::misc::{matrix_vector_fp, rnd_fp_vec};
    use crate::gsw::pk::GswPk;
    use crate::gsw::sk::GswSk;
    #[test]
    fn sk_pk_invariant() {
        let n = 10;
        let m = 5;

        let sk = GswSk::new(rnd_fp_vec(n, 0, P-1));
        let err = rnd_fp_vec(m, 0, 10);
        let B: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_vec(n, 0, P-1)).collect();
        let pk = GswPk::new(&B, &err, &sk.t);
        let invariant = matrix_vector_fp(&pk.A, &sk.s);

        // As = e
        assert_eq!(invariant, err);
    }

    #[test]
    fn encryption_decryption() {
        
        let (sk, pk) = NAIVE_GSW.keygen();

        let encr = NAIVE_GSW.encrypt(&pk, Fp::ZERO);
        let decr = NAIVE_GSW.decrypt(&sk, encr);
        assert_eq!(decr, Fp::ZERO);

        let encr = NAIVE_GSW.encrypt(&pk, Fp::ONE);
        let decr = NAIVE_GSW.decrypt(&sk, encr);
        assert_eq!(decr, Fp::ONE);
    }
}
