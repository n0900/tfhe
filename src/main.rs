pub mod misc;
pub mod field;
pub mod sk;
pub mod pk;
use ff::PrimeFieldBits;
use crate::{field::{Fp, P}, sk::SK, pk::PK, misc::rnd_fp_vec};

pub fn sk_gen(n: u8) -> SK {
    SK::new(rnd_fp_vec(n,0,P))
}

pub fn pk_gen(n: u8, err: &Vec<Fp>, sk: &SK) -> PK {
    let B: Vec<Vec<Fp>> = (0..err.len()).map(|_| rnd_fp_vec(n, 0, P)).collect();
    PK::new(&B,&err, &sk.t)
}

// fn enc(params,pk,m) {
//     return todo!()
// }

// fn dec(params, sk, c) {
//     return todo!()
// }

// fn mp_dec(params, sk, c) {
//     return todo!()
// }
fn assert_bits<T: ff::PrimeFieldBits>(_: &T) {}

fn main() {
    let x = Fp::from(42u64);
    assert_bits(&x); // This will fail to compile if Fp doesn't implement PrimeFieldBits
    let bits = x.to_le_bits();
    println!("{:?}", bits);
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
