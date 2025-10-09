use ff::{Field, PrimeField};
use nalgebra::{DMatrix, DVector};

use crate::{
    error_sampling::{rnd_fp_dmatrix, rnd_fp_dvec, ErrorSampling}, field::{Fp, P}, gsw::{helper::{bit_decomp_inv_matrix, bit_decomp_matrix, flatten_matrix}, pk::GswPk, sk::GswSk, FheScheme, GSW}, RingElement
};

impl<T: ErrorSampling<Fp>> FheScheme<Fp> for GSW<Fp, T> {
    type SecretKey = GswSk<Fp>;
    type PublicKey = GswPk;
    type Ciphertext = DMatrix<Fp>;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey) {
        let sk = GswSk::new(rnd_fp_dvec(self.n as usize, 0, P-1));  
          
        let err: DVector<Fp> = self.err_sampling.rnd_fp_dvec(self.m as usize);    
        let random_matrix: DMatrix<Fp> = rnd_fp_dmatrix(self.m, self.n, 0, P-1);
        
        let pk = GswPk::new(&random_matrix, &err, &sk.t);
        (sk, pk)   
    }

    fn encrypt(&self, pk: &Self::PublicKey, message: Fp) -> Self::Ciphertext {
        // #[cfg(feature="use_flatten")]
        let big_n: usize = (Fp::Num_Bits as usize) * ((self.n + 1) as usize);
        // #[cfg(not(feature="use_flatten"))]
        // let big_n: usize = (self.n + 1) as usize;

        //(0..big_n).map(|_| rnd_fp_dvec(self.m as usize, 0, 1)).collect();
        let random_matrix = rnd_fp_dmatrix(big_n, self.m, 0, 1);
        let mut product = &random_matrix * &pk.pk_matrix;
        bit_decomp_matrix(&mut product);
        // Add message to diagonal (matrix is square)
        for i in 0..product.ncols() {
            product[(i, i)] += message;
        }
        // #[cfg(not(feature="use_flatten"))]
        // {
        //     let mut test = DMatrix::identity((self.n + 1) * Fp::Num_Bits, (self.n + 1) * Fp::Num_Bits) * message;
        //     bit_decomp_inv_matrix(&mut test);
        //     product += test
        // }
        // #[cfg(feature="use_flatten")]

        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut product);

        product
    }

    fn decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> Fp {
        let i = Fp::Num_Bits -1;//10;//sk.v.len();
        assert!(sk.v[i] != Fp::ZERO, "We hit the bad case");
        let test = ciphertext.row(i).transpose().dot(&sk.v);
        // Bit Decomp is only necessary if its deactivated in cfg!
        // #[cfg(feature="use_flatten")]
        // {
        //     test = ciphertext.row(i).dot(&sk.v);

        // }
        // #[cfg(not(feature="use_flatten"))]
        // {
        //     // use crate::field::GADGET_VECTOR;

        //     // let mut flattened_ct = ciphertext.clone();
        //     // bit_decomp_matrix(&mut flattened_ct);
        //     // // flatten_matrix(&mut flattened_ct);
        //     // let testier = &flattened_ct.row(i);
        //     // let testiest = &sk.s * GADGET_VECTOR.clone();
        //     // test = testier.dot(&testiest);
        //     test = ciphertext.row(i).dot(&sk.s);
        // }
        if test >= Fp::from(P/4) && test <= Fp::from(3*P/4)  {
            Fp::ONE
        } else { Fp:: ZERO }
    }

    fn mp_decrypt(&self, _sk: &Self::SecretKey, _ciphertext: &Self::Ciphertext) -> Fp {
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
        assert_eq!(ciphertext1.nrows(), cipertext2.nrows(), "Cannot add Ciphertexts because they are different sizes");
        assert_eq!(ciphertext1.ncols(), cipertext2.ncols(), "Cannot add Ciphertexts because they are different sizes");
        let mut res = ciphertext1 + cipertext2;
        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut res);
        res
    }

    // flatten(C*a)
    fn mult_const(&self, ciphertext: &mut Self::Ciphertext, constant: Fp) {
        *ciphertext *= constant;
        #[cfg(feature="use_flatten")]
        flatten_matrix(ciphertext);
    }

    // flatten(C1*C2)
    fn mult(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        assert_eq!(ciphertext1.nrows(), cipertext2.nrows(), "Cannot add Ciphertexts because they are different sizes");
        assert_eq!(ciphertext1.ncols(), cipertext2.ncols(), "Cannot add Ciphertexts because they are different sizes");
        let mut res = ciphertext1 * cipertext2;
        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut res);
        res
    }

    // flatten(I - C1*C2)
    fn nand(&self, ciphertext1: &Self::Ciphertext, cipertext2: &Self::Ciphertext) -> Self::Ciphertext {
        let mut prod = ciphertext1 * cipertext2;
        // negate_matrix_fp(&mut prod);
        prod.neg_mut();
        // add_to_diagonal(&mut prod, Fp::ONE);
        for i in 0..prod.ncols() {
            prod[(i, i)] += Fp::ONE;
        } 
        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut prod);
        prod
    }
}


#[cfg(test)]
mod tests {
    use std::marker::PhantomData;

    use ff::{Field};
    use rand::Rng;

    use crate::error_sampling::rnd_fp_dmatrix;
    use crate::error_sampling::rnd_fp_dvec;
    use crate::error_sampling::DiscrGaussianSampler;
    use crate::error_sampling::NaiveSampler;
    use crate::field::Fp;
    use crate::field::P;
    use crate::gsw::pk::GswPk;
    use crate::gsw::FheScheme;
    use crate::gsw::sk::GswSk;
    use crate::gsw::GSW;
    use crate::RingElement;

    #[test]
    fn sk_pk_invariant() {
        let n = 10;
        let m = 5;

        let sk = GswSk::new(rnd_fp_dvec(n, 0, P-1));
        let err = rnd_fp_dvec(m, 0, P/2);
        // let random_matrix: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_dvec(n, 0, P-1)).collect();
        let random_matrix = rnd_fp_dmatrix(err.len(), n, 0, P-1);
        let pk = GswPk::new(&random_matrix, &err, &sk.t);
        let invariant = &pk.pk_matrix * &sk.s;

        // As = e
        assert_eq!(invariant, err);
    }

    #[test]
    fn encryption_decryption_naive() {
        let naive_gsw = GSW::<Fp, NaiveSampler> {
            n: 10,
            m: 10 * Fp::Num_Bits,
            err_sampling: NaiveSampler,
            _marker: PhantomData,
        };
        test_inputs(naive_gsw);
    }


    #[test]
    fn encryption_decryption_discr_gaussian() {
        let gaussian_gsw = GSW::<Fp, DiscrGaussianSampler> {
            n:10, 
            m: 10*Fp::Num_Bits, 
            err_sampling: DiscrGaussianSampler::default(), 
            _marker: PhantomData
        };
        test_inputs(gaussian_gsw);
    }


    #[test]
    fn encryption_decryption_pow_of_two() {
        let fhe = GSW::<Fp, DiscrGaussianSampler> {
            n:10, 
            m: 5, 
            err_sampling: DiscrGaussianSampler::default(),
            _marker: PhantomData 
        };
        
        let (sk, pk) = fhe.keygen();

        let encr = fhe.encrypt(&pk, Fp::ZERO);
        let decr = fhe.mp_decrypt(&sk, &encr);
        assert_eq!(decr, Fp::ZERO);

        let encr = fhe.encrypt(&pk, Fp::ONE);
        let decr = fhe.mp_decrypt(&sk, &encr);
        assert_eq!(decr, Fp::ONE);

        let mut rng = rand::rng();
        for _ in 0..10 {
            let msg = Fp::from(1u64<<rng.random_range(0..Fp::Num_Bits-1));
            let encr = fhe.encrypt(&pk, msg);
            let decr = fhe.mp_decrypt(&sk, &encr);
            assert_eq!(decr, msg);
        }
    }

    fn test_inputs<T: FheScheme<Fp>>(fhe: T) {
        let (sk, pk) = fhe.keygen();

        let mut encr = fhe.encrypt(&pk, Fp::ZERO);
        let decr = fhe.decrypt(&sk, &mut encr);
        assert_eq!(decr, Fp::ZERO);

        let mut encr = fhe.encrypt(&pk, Fp::ONE);
        let decr = fhe.decrypt(&sk, &mut encr);
        assert_eq!(decr, Fp::ONE);
    }
}