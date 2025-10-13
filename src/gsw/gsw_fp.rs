use std::any::TypeId;

use nalgebra::{DMatrix, DVector};

use crate::{
    error_sampling::{rnd_dmatrix, rnd_dvec, ErrorSampling}, field::Fp, gsw::{helper::bit_decomp_matrix, pk::GswPk, sk::GswSk, FheScheme, GSW}, RingElement
};

#[cfg(feature="use_flatten")]
use crate::{gsw::helper::flatten_matrix};

impl<R: RingElement + 'static, T: ErrorSampling<R>> FheScheme<R> for GSW<R, T> {
    type SecretKey = GswSk<R>;
    type PublicKey = GswPk<R>;
    type Ciphertext = DMatrix<R>;

    fn keygen(&self) -> (Self::SecretKey, Self::PublicKey) {
        let sk = GswSk::new(rnd_dvec(self.n as usize, 0, R::max_u64()));  
          
        let err: DVector<R> = self.err_sampling.rnd_error_dvec(self.m as usize);    
        let random_matrix: DMatrix<R> = rnd_dmatrix(self.m, self.n, 0, R::max_u64());
        
        let pk = GswPk::new(&random_matrix, &err, &sk.t);
        (sk, pk)   
    }

    fn encrypt(&self, pk: &Self::PublicKey, message: R) -> Self::Ciphertext {
        let big_n: usize = (R::Num_Bits as usize) * ((self.n + 1) as usize);

        let random_matrix = rnd_dmatrix(big_n, self.m, 0, 1);
        let mut product = &random_matrix * &pk.pk_matrix;
        bit_decomp_matrix(&mut product);
        // Add message to diagonal (matrix is square)
        for i in 0..product.ncols() {
            product[(i, i)] += message;
        }

        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut product);

        product
    }


    /**  
     * sk.v[i] == 2^{i-1} bc the first entry of s is 1 by definition and v = pow2(s)
     */
    fn decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> R {
        let i = R::Num_Bits -1;
        let cipher_row_dot_prod = ciphertext.row(i).transpose().dot(&sk.v);
        is_zero_one(cipher_row_dot_prod)
    }


    /**
     * collect LSBs
     * Let t_{l-1} denote the last element of slice. then this contains
     * 2^{l-1} * mu + noise
     * It holds that 
     * 2^{l-1} * mu = mu_1 mod 2^l 
     * where mu_1 is the LSB.
     * Following this logic it holds that
     * mu_2 = t_{l-2} - 2^{l-2} mu_1
     * <==> mu_2 = t_{l-2} - recovered_bits << l-2
     * etc.
     */
    fn mp_decrypt(&self, sk: &Self::SecretKey, ciphertext: &Self::Ciphertext) -> R {
        if TypeId::of::<R>() == TypeId::of::<Fp>() {
            panic!("Only supported for pow2 rings!");
        } else {
            let product = ciphertext * &sk.v;
            let slice = &product.as_slice()[..R::Num_Bits];
            let mut recovered_bits: u64 = 0;
            let mut recovered_exp;
            let mut current_exp;

            for (i,entry) in slice.into_iter().rev().enumerate() {
                recovered_exp = R::from(recovered_bits << (R::Num_Bits - i - 1));
                current_exp = *entry - recovered_exp;
                recovered_bits ^= (is_zero_one(current_exp).is_one() as u64) << i;
            }
            R::from(recovered_bits)
        }        
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
    fn mult_const(&self, ciphertext: &mut Self::Ciphertext, constant: R) {
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
        prod.neg_mut();
        for i in 0..prod.ncols() {
            prod[(i, i)] += R::one();
        } 
        #[cfg(feature="use_flatten")]
        flatten_matrix(&mut prod);
        prod
    }

}

fn is_zero_one<R: RingElement>(input: R) -> R {
    if input >= R::from(R::max_u64()/4) && input <= R::from(3*R::max_u64()/4)  {
        R::one()
    } else { R::zero() }
}

#[cfg(test)]
mod tests {
    use std::marker::PhantomData;
    use rand::Rng;

    use crate::error_sampling::rnd_dmatrix;
    use crate::error_sampling::rnd_dvec;
    use crate::error_sampling::DiscrGaussianSampler;
    use crate::error_sampling::NaiveSampler;
    use crate::field::Fp;
    use crate::gsw::pk::GswPk;
    use crate::gsw::FheScheme;
    use crate::gsw::sk::GswSk;
    use crate::gsw::GSW;
    use crate::pow2_ring::Zpow2;
    use crate::RingElement;

    #[test]
    fn sk_pk_invariant() {
        let n = 10;
        let m = 10 * Fp::Num_Bits;

        let sk: GswSk<Fp> = GswSk::new(rnd_dvec(n, 0, Fp::max_u64()));
        let err = rnd_dvec(m, 0, Fp::max_u64()>>15);
        let random_matrix = rnd_dmatrix(err.len(), n, 0, Fp::max_u64());
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
        let gaussian_gsw = GSW::<Zpow2<30>, DiscrGaussianSampler> {
            n:10, 
            m: 10*Zpow2::<30>::Num_Bits, 
            err_sampling: DiscrGaussianSampler::default(), 
            _marker: PhantomData
        };
        test_inputs(gaussian_gsw);
    }


    #[test]
    fn encryption_decryption_pow_of_two() {
        let fhe = GSW::<Zpow2<31>, DiscrGaussianSampler> {
            n:10, 
            m: 10 * Zpow2::<31>::Num_Bits, 
            err_sampling: DiscrGaussianSampler::default(),
            _marker: PhantomData 
        };
        
        let (sk, pk) = fhe.keygen();

        let mut rng = rand::rng();
        for _ in 0.. 20 {
            let msg = Zpow2::<31>::from(rng.random_range(0..Zpow2::<31>::max_u64()));
            let encr = fhe.encrypt(&pk, msg);
            let decr = fhe.mp_decrypt(&sk, &encr);
            assert_eq!(decr, msg, "{:064b} vs {:064b}", decr.value(), msg.value());
        }
    }

    fn test_inputs<R: RingElement + 'static, T: FheScheme<R>>(fhe: T) {
        let (sk, pk) = fhe.keygen();

        let mut encr = fhe.encrypt(&pk, R::zero());
        let decr = fhe.decrypt(&sk, &mut encr);
        assert_eq!(decr, R::zero());

        let mut encr = fhe.encrypt(&pk, R::one());
        let decr = fhe.decrypt(&sk, &mut encr);
        assert_eq!(decr, R::one());
    }
}