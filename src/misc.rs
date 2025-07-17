use crate::field::{Fp, GADGET_VECTOR, L};
use bitvec::{prelude::*, vec::BitVec};
use ff::{Field, PrimeField, PrimeFieldBits};

/// BitDecomp: little endian bit decomposition
/// Take vector and enforce little endianness
/// a.len() == k; l is implicit
pub fn bit_decomp(a: &[Fp]) -> BitVec<u8, Lsb0> {
    let total_bits = a.len() * L;
    let mut bv = BitVec::<u8, Lsb0>::with_capacity(total_bits);
    for elm in a {
        bv.extend(elm.to_le_bits().iter().take(L));
    }
    bv
}

/// Careful this only works up to 64 bits and then becomes annoying!
pub fn bit_decomp_inv(bits: &BitVec<u8, Lsb0>) -> Vec<Fp> {
    bits.chunks(L)
        .map(|chunk| {
            // Convert chunk into little-endian u64 representation
            let mut repr = 0u64; // because Fp uses [u64;1] in your definition

            // Fill bits into repr[0]
            for (i, bit) in chunk.iter().enumerate() {
                if *bit {
                    repr |= 1 << i;
                }
            }
            Fp::from(repr)
        })
        .collect()
}

pub fn flatten(bits: &BitVec<u8, Lsb0>) -> BitVec<u8, Lsb0> {
    bit_decomp(&bit_decomp_inv(bits))
}

/// PowersOf2: (b_1, 2b_1, ..., 2^{l-1}b_1, ..., b_k, ..., 2^{l-1}b_k)
pub fn powers_of_2(b: &[Fp]) -> Vec<Fp> {
    let mut out = Vec::with_capacity(b.len() * GADGET_VECTOR.len());
    for x in b {
        for g in GADGET_VECTOR.iter() {
            out.push(*x * *g);
        }
    }
    out
}

/// Compute l = floor(log2(q)) + 1
pub fn calc_l(q: u64) -> usize {
    64 - q.leading_zeros() as usize
}

/// BitDecomp: little endian bit decomposition
/// Take vector and enforce little endianness
/// a.len() == k; l = floor(log2(q)) + 1
pub fn bit_decomp_u64(a: &[u64], l: usize) -> BitVec<u64, Lsb0> {
    let mut bv = bitvec![u64, Lsb0;];
    for &x in a {
        let x_le = x.to_le();
        bv.extend(&x_le.view_bits::<Lsb0>()[..l]);
    }
    bv
}

/// BitDecomp^-1: reconstruct from little endian bit decomposition
pub fn bit_decomp_inv_u64(bits: &BitVec<u64, Lsb0>, l: usize) -> Vec<u64> {
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

pub fn flatten_u64(bits: &BitVec<u64, Lsb0>, l: usize) -> BitVec<u64, Lsb0> {
    bit_decomp_u64(&bit_decomp_inv_u64(bits, l), l)
}

// Helper: dot product over field elements
pub fn dot_product_fp(a: &[Fp], b: &[Fp]) -> Fp {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b).fold(Fp::ZERO, |acc, (x, y)| acc + (*x * *y))
}

// Helper: dot product between BitVec and Vec<Fp>
pub fn dot_product_bit_fp(bits: &BitVec<u8, Lsb0>, fp_vec: &[Fp]) -> Fp {
    assert_eq!(bits.len(), fp_vec.len());
    bits.iter()
        .zip(fp_vec)
        .fold(Fp::ZERO, |acc, (bit, f)| if *bit { acc + *f } else { acc })
}

#[cfg(test)]
mod tests {
    use crate::field::GADGET_VECTOR;
    use crate::field::Fp;
    use rand::{Rng};
    use super::*;

    #[test]
    fn gadget_vector_has_correct_size() {
        println!("Fp::NUM_BITS {}", Fp::NUM_BITS);
        assert_eq!(GADGET_VECTOR.len(), Fp::NUM_BITS as usize)
    }
    #[test]
    fn test_bit_decomp_and_inv_for_field() {
        for _ in 0..10 {
            let mut rng = rand::rng();
            // Generate 3 random u64 inputs
            let input: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();
            println!("input = {:?}", input);

            // Perform bit decomposition
            let decomposed = bit_decomp(&input);

            // Reconstruct
            let reconstructed = bit_decomp_inv(&decomposed);

            // Check roundtrip correctness
            assert_eq!(input, reconstructed);
        }
    }

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
            let decomposed = bit_decomp_u64(&input, l);

            // Check decomposition length
            assert_eq!(decomposed.len(), input.len() * l);

            // Reconstruct
            let reconstructed = bit_decomp_inv_u64(&decomposed, l);

            // Check roundtrip correctness
            assert_eq!(input, reconstructed);
        }
    }

    #[test]
    fn test_scalar_product_invariant() {
        let mut rng = rand::rng();
        
        for _ in 0..10{
            // Generate random input vectors a and b
            let a: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();
            let b: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();

            // Compute BitDecomp(a) and PowersOf2(b)
            let bd_a = bit_decomp(&a);
            let po2_b = powers_of_2(&b);

            // Ensure lengths match for scalar product
            assert_eq!(bd_a.len(), po2_b.len(), "Length mismatch in decomp vs powers_of_two.");

            // Compute both scalar products
            let dot_decomp = dot_product_bit_fp(&bd_a, &po2_b);
            let dot_ab = dot_product_fp(&a, &b);
            println!("Dot_Decomp {:?} = {:?} Dot_Ab", dot_decomp, dot_ab);

            // Assert equivalence
            assert_eq!(dot_decomp, dot_ab, "Scalar product invariant failed.");
        }
    }

}
