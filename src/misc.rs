use bitvec::prelude::*;
use bitvec::{prelude::*, vec::BitVec};
use ff::PrimeFieldBits;
use crate::field;

/// Compute l = floor(log2(q)) + 1
pub fn calc_l(q: u64) -> usize {
    64 - q.leading_zeros() as usize
}

/// BitDecomp: little endian bit decomposition
/// Take vector and enforce little endianness
/// a.len() == k; l = floor(log2(q)) + 1
pub fn bit_decomp_field(a: &[field::Fp], l: usize) -> BitVec<u8, Lsb0> {
    let mut bv = BitVec::<u8, Lsb0>::new();

    for elm in a {
        
    }
    bv
}

/// BitDecomp: little endian bit decomposition
/// Take vector and enforce little endianness
/// a.len() == k; l = floor(log2(q)) + 1
pub fn bit_decomp(a: &[u64], l: usize) -> BitVec<u64, Lsb0> {
    let mut bv = bitvec![u64, Lsb0;];
    for &x in a {
        let x_le = x.to_le();
        bv.extend(&x_le.view_bits::<Lsb0>()[.. l]);
    }
    bv
}

/// BitDecomp^-1: reconstruct from little endian bit decomposition
pub fn bit_decomp_inv(bits: &BitVec<u64, Lsb0>, l: usize) -> Vec<u64> {
    bits.chunks(l)
        .map(|chunk| {
            let mut acc = 0u64;
            for (i, bit) in chunk.iter().enumerate() {
                if *bit {
                    acc |= 1 << i;
                }
            }
            acc
        })
        .collect()
}

pub fn flatten(bits: &BitVec<u64, Lsb0>, l: usize) -> BitVec<u64, Lsb0> {
    bit_decomp(&bit_decomp_inv(bits, l), l)
}


/// PowersOf2: (b_1, 2b_1, ..., 2^{l-1}b_1, ..., b_k, ..., 2^{l-1}b_k)
pub fn powers_of_2(b: &[u64], l: usize, p: u64) -> Vec<u64> {
    let mut out = Vec::with_capacity(b.len() * l);
    for (i, x) in b.iter().enumerate() {
        // for i in 0..l {
        //     out.push(x << i);
        // }
        // out.append(x * 2**i % p);
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::prelude::*;


    #[test]
    fn test_bit_decomp_and_inv_with_random_inputs() {
        for i in 0..10 {
                
            let mut rng = rand::rng();
            // Generate 3 random u64 inputs
            let input: Vec<u64> = (0..10).map(|_| rng.random::<u64>().to_le() >> i).collect();
            let l = input.iter().map(|&x| calc_l(x)).max().unwrap();
            println!("input = {:?}", input);
            println!("l = {}", l);

            // Perform bit decomposition
            let decomposed = bit_decomp(&input, l);

            // Check decomposition length
            assert_eq!(decomposed.len(), input.len() * l);

            // Reconstruct
            let reconstructed = bit_decomp_inv(&decomposed, l);

            // Check roundtrip correctness
            assert_eq!(input, reconstructed);
        }
    }
}