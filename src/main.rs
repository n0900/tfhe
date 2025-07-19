pub mod misc;
pub mod field;
pub mod sk;
use ff::PrimeFieldBits;
use rand::Rng;
use crate::{field::{Fp, P}, sk::SK};

fn sk_gen(n: u8) -> SK {
    let mut rng = rand::rng();
    let t = (0..n).map(|_| Fp::from(rng.random_range(0..P))).collect();
    SK::new(t)
}
// fn public_key_gen(params, sk) {
//     return todo!()
// }

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
