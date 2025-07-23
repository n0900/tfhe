use crate::field::{Fp, GADGET_VECTOR, L};
use ff::{Field, PrimeFieldBits};
use rand::Rng;

pub fn rnd_fp_vec(size: u8, min: u64, max: u64) -> Vec<Fp> {
    let mut rng = rand::rng();
    (0..size).map(|_| Fp::from(rng.random_range(min..max))).collect()
}

/// BitDecomp: Expand every Fp entry into bit representation and
/// output a.len()*L =: N-dim array of Fp::ZERO and Fp::ONE entries.
/// in little endian.
pub fn bit_decomp(a: &Vec<Fp>) -> Vec<Fp> {
    let mut bv = Vec::with_capacity(a.len() * L);
    for elm in a.iter() {
        for b in elm.to_le_bits().iter().take(L) {
            bv.push(if *b { Fp::ONE } else { Fp::ZERO });
        }
    }
    bv
}

/// "When A is a matrix, let BitDecomp(A), BitDecomp−1 , or Flatten(A) be 
/// the matrix formed by applying the operation to each row of A separately"
pub fn bit_decomp_matrix(a_matrix: &Vec<Vec<Fp>>) -> Vec<Vec<Fp>> {
    a_matrix.iter().map(|a| 
    bit_decomp(a)).collect()
}

/// Inverse operation; only supports [u64;1] for now but could be expanded.
pub fn bit_decomp_inv(bits: &Vec<Fp>) -> Vec<Fp> {
    bits.chunks(L)
        .map(|chunk| {
            // Convert chunk into little-endian u64 representation
            let mut repr = 0u64; // because Fp uses [u64;1]

            // Fill bits into repr[0]
            for (i, bit) in chunk.iter().enumerate() {
                if *bit == Fp::ONE {
                    repr |= 1 << i;
                }
            }
            Fp::from(repr)
        })
        .collect()
}

/// "When A is a matrix, let BitDecomp(A), BitDecomp−1 , or Flatten(A) be 
/// the matrix formed by applying the operation to each row of A separately"
pub fn bit_decomp_inv_matrix(a_matrix: &Vec<Vec<Fp>>) -> Vec<Vec<Fp>> {
    a_matrix.iter().map(|a| 
    bit_decomp_inv(a)).collect()
}

pub fn flatten(bits: &Vec<Fp>) -> Vec<Fp> {
    bit_decomp(&bit_decomp_inv(&bits))
}

/// "When A is a matrix, let BitDecomp(A), BitDecomp−1 , or Flatten(A) be 
/// the matrix formed by applying the operation to each row of A separately"
pub fn flatten_matrix(a_matrix: &Vec<Vec<Fp>>) -> Vec<Vec<Fp>> {
    a_matrix.iter().map(|a| 
    flatten(a)).collect()
}

/// PowersOf2: (b_1, 2b_1, ..., 2^{l-1}b_1, ..., b_k, ..., 2^{l-1}b_k)
pub fn powers_of_2(b: &Vec<Fp>) -> Vec<Fp> {
    let mut out = Vec::with_capacity(b.len() * GADGET_VECTOR.len());
    for x in b {
        for g in GADGET_VECTOR.iter() {
            out.push(*x * *g);
        }
    }
    out
}

// Helper: dot product over field elements
pub fn dot_product_fp(a: &Vec<Fp>, b: &Vec<Fp>) -> Fp {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).fold(Fp::ZERO, |acc, (x, y)| acc + (*x * *y))
}

pub fn matrix_vector_fp(matrix: &Vec<Vec<Fp>>, vector: &Vec<Fp>) -> Vec<Fp> {
    assert!(matrix.iter().all(|row| row.len() == vector.len()), "Matrix column count must match vector length");

    matrix.iter()
        .map(|row| {
            row.iter()
                .zip(vector.iter())
                .fold(Fp::ZERO, |acc, (x, y)| acc + (*x * *y))
        })
        .collect()
}

pub fn vec_vec_fp_add(a: &Vec<Fp>, b: &Vec<Fp>) -> Vec<Fp> {
    assert_eq!(a.len(), b.len());
    a.iter().zip(b.iter()).map(|(x, y)| *x + *y).collect()
}


pub fn matrix_matrix_fp(a: &Vec<Vec<Fp>>, b: &Vec<Vec<Fp>>) -> Vec<Vec<Fp>> {
    let a_rows = a.len();
    let a_cols = if a_rows > 0 { a[0].len() } else { 0 };
    let b_rows = b.len();
    let b_cols = if b_rows > 0 { b[0].len() } else { 0 };

    assert!(a_cols == b_rows, "Incompatible dimensions: a_cols must equal b_rows");

    (0..a_rows).map(|i| {
        (0..b_cols).map(|j| {
            let mut sum = Fp::ZERO;
            for k in 0..a_cols {
                sum += a[i][k] * b[k][j];
            }
            sum
        }).collect()
    }).collect()
}

//more efficient addition of \mu * I_n
pub fn add_to_diagonal(a: &Vec<Vec<Fp>>, mu: Fp) -> Vec<Vec<Fp>> {
    a.iter().enumerate().map(| (i, row) |
        {
        let mut new_row = row.clone();
        new_row[i] = new_row[i] + mu;
        new_row
    }).collect()
}


#[cfg(test)]
mod tests {
    use crate::field::GADGET_VECTOR;
    use crate::field::Fp;
    use ff::PrimeField;
    use rand::{Rng};
    use super::*;

    #[test]
    fn gadget_vector_has_correct_size() {
        assert_eq!(GADGET_VECTOR.len(), Fp::NUM_BITS as usize)
    }
    #[test]
    fn test_bit_decomp_and_inv_for_field() {
        for _ in 0..10 {
            let mut rng = rand::rng();
            let input: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();

            let decomposed = bit_decomp(&input);
            assert_eq!(decomposed.len(), input.len()*L);

            let reconstructed = bit_decomp_inv(&decomposed);
            assert_eq!(reconstructed.len(), input.len());

            assert_eq!(input, reconstructed);
        }
    }

    #[test]
    fn test_scalar_product_invariant() {
        let mut rng = rand::rng();
        
        for _ in 0..10{
            let a: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();
            let b: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();

            let bd_a = bit_decomp(&a);
            let po2_b = powers_of_2(&b);
            assert_eq!(bd_a.len(), po2_b.len(), "Length mismatch in decomp vs powers_of_two.");

            let dot_decomp = dot_product_fp(&bd_a, &po2_b);
            let dot_ab = dot_product_fp(&a, &b);

            assert_eq!(dot_decomp, dot_ab, "Scalar product invariant failed.");
        }
    }

    #[test]
    fn test_scalar_product_fp() {
        let a = vec![Fp::from(1), Fp::from(2), Fp::from(3)];
        let b = vec![Fp::from(4), Fp::from(5), Fp::from(6)];
        let result = dot_product_fp(&a, &b);
        assert_eq!(result, Fp::from(32)); // 1*4 + 2*5 + 3*6 = 32
    }

    #[test]
    fn test_matrix_vector_fp() {
        let matrix = vec![
            vec![Fp::from(1), Fp::from(2), Fp::from(3)],
            vec![Fp::from(4), Fp::from(5), Fp::from(6)],
        ];
        let vector = vec![Fp::from(7), Fp::from(8), Fp::from(9)];

        let result = matrix_vector_fp(&matrix, &vector);
        let expected = vec![
            Fp::from(50), // 1*7 + 2*8 + 3*9
            Fp::from(122), // 4*7 + 5*8 + 6*9
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_matrix_matrix_multiplication_fp() {
        let a = vec![
            vec![Fp::from(1), Fp::from(2)],
            vec![Fp::from(3), Fp::from(4)],
        ];
        let b = vec![
            vec![Fp::from(5), Fp::from(6)],
            vec![Fp::from(7), Fp::from(8)],
        ];

        let result = matrix_matrix_fp(&a, &b);
        let expected = vec![
            vec![Fp::from(19), Fp::from(22)], // [1*5 + 2*7, 1*6 + 2*8]
            vec![Fp::from(43), Fp::from(50)], // [3*5 + 4*7, 3*6 + 4*8]
        ];
        assert_eq!(result, expected);
    }

}
