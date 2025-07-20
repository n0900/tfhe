pub mod misc;
pub mod field;
pub mod sk;
pub mod pk;
use crate::{field::{Fp, P}, misc::{add_to_diagonal, bit_decomp_matrix, flatten_matrix, matrix_matrix_fp, rnd_fp_vec}, pk::PK, sk::SK};

pub fn sk_gen(n: u8) -> SK {
    SK::new(rnd_fp_vec(n,0,P))
}

pub fn pk_gen(n: u8, err: &Vec<Fp>, sk: &SK) -> PK {
    let B: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_vec(n, 0, P)).collect();
    PK::new(&B,&err, &sk.t)
}

pub fn enc(n: u8, m: u8, pk: &PK, mu: Fp) -> Vec<Vec<Fp>> {
    let N = (n+1) * m;
    let R = (0..N).map(|_| rnd_fp_vec(m, 0, 1)).collect();
    let temp = matrix_matrix_fp(&R, &pk.A);
    let temp2 = bit_decomp_matrix(&temp);
    let temp3: Vec<Vec<Fp>> = add_to_diagonal(&temp2, mu);
    flatten_matrix(&temp3)
}

// fn dec(params, sk, c) {
//     return todo!()
// }

// fn mp_dec(params, sk, c) {
//     return todo!()
// }

fn main() {
}


#[cfg(test)]
mod tests {
    use crate::{misc::{matrix_vector_fp, rnd_fp_vec}, pk_gen, sk_gen};
    #[test]
    fn pk_invariant() {
        let n = 10;
        let m = 5;
        let err = rnd_fp_vec(m, 0, 10);  //TODO replace with real err dist
        let sk = sk_gen(n);
        let pk = pk_gen(n, &err, &sk);

        let invariant = matrix_vector_fp(&pk.A, &sk.s);

        // As = e
        assert_eq!(invariant, err);
    }
}
