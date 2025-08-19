pub mod sk;
pub mod pk;
pub mod gsw;

use std::ops::Mul;

use ff::{Field, PrimeFieldBits};

use crate::{
    error_sampling::{rnd_fp_vec, ErrorSampling}, field::{Fp, L, P}, gsw::{gsw::{
        add_matrix_matrix_fp, add_to_diagonal, bit_decomp_matrix, dot_product_fp, flatten_matrix, mult_const_matrix_fp, mult_matrix_matrix_fp, mult_matrix_vector_fp, negate_matrix_fp
    }, pk::GswPk, sk::GswSk}
};

pub const USE_FLATTEN: bool = false;

pub trait FheScheme {
    type SecretKey;
    type PublicKey;
    type Ciphertext;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey);
    fn encrypt(&self, pk: &Self::PublicKey, message: Fp) -> Self::Ciphertext;
    fn decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Fp;
    fn mp_decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Fp;
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
        let res =
            add_to_diagonal(
                &bit_decomp_matrix(
                    &mult_matrix_matrix_fp(&random_matrix, &pk.pk_matrix)
                ),
                message
            );
        if USE_FLATTEN {flatten_matrix(&res)} else {res}
    }

    fn decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Fp {
        let i = 64 - (P / 3).leading_zeros() as usize; //efficient log2(P/3) (only for u64!)
        let x_i = dot_product_fp(&ciphertext[i], &sk.v);
        let v_i = sk.v[i].invert().unwrap();
        x_i * v_i
    }

    fn mp_decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Fp {
        let test = mult_matrix_vector_fp(&ciphertext, &sk.v);
        let mut out: u64 = 0;
        let mut i = 0;

        // collect LSBs
        for entry in test.iter().rev() {
            out ^= entry.to_le_bits()[i] as u64;
            i+=1;
            if i>= L {
                break;
            }
        }

        Fp::from(out)
    }

    // flatten(C1+C2)
    fn add(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        assert_eq!(ciphertext1.len(), cipertext2.len(), "Cannot add Ciphertexts because they are different sizes");
        assert_eq!(ciphertext1.first().unwrap().len(), cipertext2.first().unwrap().len(), "Cannot add Ciphertexts because they are different sizes");
        let res = add_matrix_matrix_fp(ciphertext1, cipertext2);
        if USE_FLATTEN {flatten_matrix(&res)} else {res}
    }

    // flatten(C*a)
    fn mult_const(&self, ciphertext: &Self::Ciphertext, constant: Fp) -> Self::Ciphertext {
        let res = mult_const_matrix_fp(&ciphertext, constant);
        if USE_FLATTEN {flatten_matrix(&res)} else {res}
    }

    // flatten(C1*C2)
    fn mult(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        assert_eq!(ciphertext1.len(), cipertext2.len(), "Cannot mult Ciphertexts because they are different sizes");
        assert_eq!(ciphertext1.first().unwrap().len(), cipertext2.first().unwrap().len(), "Cannot mult Ciphertexts because they are different sizes");
        let res = mult_matrix_matrix_fp(ciphertext1, cipertext2);
        if USE_FLATTEN {flatten_matrix(&res)} else {res}
    }

    // flatten(I - C1*C2)
    fn nand(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        let res = add_to_diagonal(
                &negate_matrix_fp(&mult_matrix_matrix_fp(ciphertext1, cipertext2)), 
                Fp::ONE);
            
        if USE_FLATTEN {flatten_matrix(&res)} else {res}
    }
}

#[cfg(test)]
mod tests {
    use ff::{Field};
    use rand::Rng;

    use crate::error_sampling::rnd_fp_vec;
    use crate::error_sampling::DiscrGaussianSampler;
    use crate::error_sampling::NaiveSampler;
    use crate::field::Fp;
    use crate::field::L;
    use crate::field::P;
    use crate::gsw::FheScheme;
    use crate::gsw::gsw::{mult_matrix_vector_fp};
    use crate::gsw::pk::GswPk;
    use crate::gsw::sk::GswSk;
    use crate::gsw::GSW;

    #[test]
    fn sk_pk_invariant() {
        let n = 10;
        let m = 5;

        let sk = GswSk::new(rnd_fp_vec(n, 0, P-1));
        let err = rnd_fp_vec(m, 0, P/2);
        let random_matrix: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_vec(n, 0, P-1)).collect();
        let pk = GswPk::new(&random_matrix, &err, &sk.t);
        let invariant = mult_matrix_vector_fp(&pk.pk_matrix, &sk.s);

        // As = e
        assert_eq!(invariant, err);
    }

    #[test]
    fn encryption_decryption_naive() {
        let naive_gsw = GSW {n:10, m: 5, err_sampling: NaiveSampler };
        test_inputs(naive_gsw);
    }


    #[test]
    fn encryption_decryption_discr_gaussian() {
        let gaussian_gsw = GSW {n:10, m: 5, err_sampling: DiscrGaussianSampler::default() };
        test_inputs(gaussian_gsw);
    }


    #[test]
    fn encryption_decryption_pow_of_two() {
        let fhe = GSW {n:10, m: 5, err_sampling: DiscrGaussianSampler::default() };
        let (sk, pk) = fhe.keygen();

        let encr = fhe.encrypt(&pk, Fp::ZERO);
        let decr = fhe.mp_decrypt(&sk, &encr);
        assert_eq!(decr, Fp::ZERO);

        let encr = fhe.encrypt(&pk, Fp::ONE);
        let decr = fhe.mp_decrypt(&sk, &encr);
        assert_eq!(decr, Fp::ONE);

        let mut rng = rand::rng();
        for _ in 0..10 {
            let msg = Fp::from(1u64<<rng.random_range(0..L-1));
            let encr = fhe.encrypt(&pk, msg);
            let decr = fhe.mp_decrypt(&sk, &encr);
            assert_eq!(decr, msg);
        }
    }

    fn test_inputs<T: FheScheme>(fhe: T) {
        let (sk, pk) = fhe.keygen();

        let encr = fhe.encrypt(&pk, Fp::ZERO);
        let decr = fhe.decrypt(&sk, &encr);
        assert_eq!(decr, Fp::ZERO);

        let encr = fhe.encrypt(&pk, Fp::ONE);
        let decr = fhe.decrypt(&sk, &encr);
        assert_eq!(decr, Fp::ONE);
    }
}
