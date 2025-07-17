pub mod misc;
pub mod field;
use ff::PrimeFieldBits;
use crate::field::Fp;
// fn secret_key_gen(params) {
//     return todo!()
// }

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
