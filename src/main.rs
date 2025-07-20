pub mod field;
pub mod misc;
pub mod pk;
pub mod sk;
use std::ops::Mul;

use ff::PrimeField;

use crate::{
    field::{Fp, L, P},
    misc::{
        add_to_diagonal, bit_decomp_matrix, dot_product_fp, flatten_matrix, matrix_matrix_fp,
        rnd_fp_vec,
    },
    pk::GswPk,
    sk::GswSk,
};

//TODO add error distribution as parameter
pub fn gsw_keygen(n: u8, m: u8) -> (GswSk, GswPk) {
    let sk = GswSk::new(rnd_fp_vec(n, 0, P));
    let err = rnd_fp_vec(m, 0, 10);
    let B: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_vec(n, 0, P)).collect();
    let pk = GswPk::new(&B, &err, &sk.t);
    (sk, pk)
}

pub fn enc(n: u8, m: u8, pk: &GswPk, mu: Fp) -> Vec<Vec<Fp>> {
    let N: usize = L.mul((n + 1) as usize);
    let R = (0..N).map(|_| rnd_fp_vec(m, 0, 1)).collect();
    flatten_matrix(
        &add_to_diagonal(
            &bit_decomp_matrix(
                &matrix_matrix_fp(&R, &pk.A)
            ),
            mu
        )
    )
}

pub fn dec(sk: &GswSk, C: Vec<Vec<Fp>>) -> Fp {
    let i = 63 - (P / 3).leading_zeros() as usize; //efficient log2(P/3)
    let x_i = u64::from_le_bytes(dot_product_fp(&C[i], &sk.v).to_repr().0);
    let v_i = u64::from_le_bytes(sk.v[i].to_repr().0);
    Fp::from(x_i / v_i)
}

fn main() {}

#[cfg(test)]
mod tests {
    use ff::{Field, derive};

    use crate::field::Fp;
    use crate::field::P;
    use crate::misc::{matrix_vector_fp, rnd_fp_vec};
    use crate::pk::GswPk;
    use crate::sk::GswSk;
    use crate::{dec, enc, gsw_keygen};
    #[test]
    fn sk_pk_invariant() {
        let n = 10;
        let m = 5;

        let sk = GswSk::new(rnd_fp_vec(n, 0, P));
        let err = rnd_fp_vec(m, 0, 10);
        let B: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_vec(n, 0, P)).collect();
        let pk = GswPk::new(&B, &err, &sk.t);
        let invariant = matrix_vector_fp(&pk.A, &sk.s);

        // As = e
        assert_eq!(invariant, err);
    }

    #[test]
    fn encryption_decryption() {
        let n = 10;
        let m = 5;

        let (sk, pk) = gsw_keygen(n, m);

        let encr = enc(n, m, &pk, Fp::ZERO);
        let decr = dec(&sk, encr);
        assert_eq!(decr, Fp::ZERO);

        let encr = enc(n, m, &pk, Fp::ONE);
        let decr = dec(&sk, encr);
        assert_eq!(decr, Fp::ONE);
    }
}
