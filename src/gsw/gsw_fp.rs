
use ff::{Field, PrimeFieldBits};

use crate::{
    error_sampling::{rnd_fp_vec, ErrorSampling}, field::{Fp, L, P}, gsw::{gsw::{
        add_matrix_matrix_fp, add_to_diagonal, bit_decomp_matrix, dot_product_fp, flatten_matrix, mult_const_matrix_fp, mult_matrix_matrix_fp, mult_matrix_vector_fp, negate_matrix_fp
    }, pk::GswPk, sk::GswSk, FheScheme, GSW}
};

impl<T: ErrorSampling> FheScheme for GSW<T> {
    type SecretKey = GswSk;
    type PublicKey = GswPk;
    type Message = Fp;
    type Constant = Fp;
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

    fn encrypt(&self, pk: &Self::PublicKey, message: Self::Message) -> Self::Ciphertext {
        #[cfg(feature="use_flatten")]
        let big_n: usize = L * ((self.n + 1) as usize);
        #[cfg(not(feature="use_flatten"))]
        let big_n: usize = (self.n + 1) as usize;

        let random_matrix = (0..big_n).map(|_| rnd_fp_vec(self.m as usize, 0, 1)).collect();
        let mut product = mult_matrix_matrix_fp(&random_matrix, &pk.pk_matrix);

        #[cfg(feature="use_flatten")]
        bit_decomp_matrix(&mut product);

        add_to_diagonal(&mut product,message);

        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut product);

        product
    }

    fn decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Self::Message {
        let i = 64 - (P / 3).leading_zeros() as usize; //efficient log2(P/3) (only for u64!)
        let v_i = sk.v[i].invert().unwrap();
        // Bit Decomp is only necessary if its deactivated in cfg!
        #[cfg(feature="use_flatten")]
        {
            dot_product_fp(&ciphertext[i], &sk.v)* v_i
        }
        #[cfg(not(feature="use_flatten"))]
        {
            let mut flattened_ct = ciphertext.clone();
            // bit_decomp_matrix(&mut flattened_ct);
            flatten_matrix( &mut flattened_ct);
            let testier = &flattened_ct[i];
            let test = dot_product_fp(testier, &sk.v);
            test * v_i
        }
    }

    fn mp_decrypt(&self, _sk: &Self::SecretKey, _ciphertext: &Self::Ciphertext) -> Self::Message {
        panic!("Only supported for pow2 rings!")
        // let test = mult_matrix_vector_fp(&ciphertext, &sk.v);
        // let mut out: u64 = 0;
        // let mut i = 0;

        // // collect LSBs
        // for entry in test.iter().rev() {
        //     out ^= entry.to_le_bits()[i] as u64;
        //     i+=1;
        //     if i>= L {
        //         break;
        //     }
        // }

        // Fp::from(out)
    }

    // flatten(C1+C2)
    fn add(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        assert_eq!(ciphertext1.len(), cipertext2.len(), "Cannot add Ciphertexts because they are different sizes");
        assert_eq!(ciphertext1.first().unwrap().len(), cipertext2.first().unwrap().len(), "Cannot add Ciphertexts because they are different sizes");
        let mut res = add_matrix_matrix_fp(ciphertext1, cipertext2);
        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut res);
        res
    }

    // flatten(C*a)
    fn mult_const(&self, ciphertext: &mut Self::Ciphertext, constant: Self::Constant) {
        mult_const_matrix_fp(ciphertext, constant);
        #[cfg(feature="use_flatten")]
        flatten_matrix(ciphertext);
    }

    // flatten(C1*C2)
    fn mult(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        assert_eq!(ciphertext1.len(), cipertext2.len(), "Cannot mult Ciphertexts because they are different sizes");
        assert_eq!(ciphertext1.first().unwrap().len(), cipertext2.first().unwrap().len(), "Cannot mult Ciphertexts because they are different sizes");
        let mut res = mult_matrix_matrix_fp(ciphertext1, cipertext2);
        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut res);
        res
    }

    // flatten(I - C1*C2)
    fn nand(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        let mut prod = mult_matrix_matrix_fp(ciphertext1, cipertext2);
        negate_matrix_fp(&mut prod);
        add_to_diagonal(&mut prod, Fp::ONE);
        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut prod);
        prod
    }
}


#[cfg(test)]
mod tests {
    use ff::PrimeField;
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
        let naive_gsw = GSW {n:10, m: 10*L, err_sampling: NaiveSampler };
        test_inputs(naive_gsw);
    }


    #[test]
    fn encryption_decryption_discr_gaussian() {
        let gaussian_gsw = GSW {n:10, m: 10*L, err_sampling: DiscrGaussianSampler::default() };
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

    fn test_inputs<T: FheScheme<Message = Fp>>(fhe: T) {
        let (sk, pk) = fhe.keygen();

        let mut encr = fhe.encrypt(&pk, Fp::ZERO);
        let decr = fhe.decrypt(&sk, &mut encr);
        assert_eq!(decr, Fp::ZERO);

        let mut encr = fhe.encrypt(&pk, Fp::ONE);
        let decr = fhe.decrypt(&sk, &mut encr);
        assert_eq!(decr, Fp::ONE);
    }
}