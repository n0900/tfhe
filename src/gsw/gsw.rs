use crate::{field::{Fp}, RingElement};
use ff::{Field};
use nalgebra::{DMatrix, DVector};

/// BitDecomp: Expand every Fp entry into bit representation and
/// output a.len()*L =: N-dim array of Fp::ZERO and Fp::ONE entries.
/// in little endian.
pub fn bit_decomp<T: RingElement>(a: &mut Vec<T>) {
    let mut tmp = Vec::with_capacity(a.len() * T::Num_Bits);
    for elm in a.drain(..) {
        tmp.extend(elm.to_le_bits().iter().take(T::Num_Bits).map(|b| if *b { T::one() } else { T::zero() }));
    }
    *a = tmp;
}

/// "When A is a matrix, let BitDecomp(A), BitDecomp−1 , or Flatten(A) be 
/// the matrix formed by applying the operation to each row of A separately"
pub fn bit_decomp_matrix<T: RingElement + 'static>(a_matrix: &mut DMatrix<T>) {
    let mut vec_of_vec = dmatrix_to_vec_of_vecs(a_matrix);
    vec_of_vec.iter_mut().for_each(|row| bit_decomp(row));
    *a_matrix = vec_of_vecs_to_dmatrix(vec_of_vec);
}

/// Only supports inverses up to u64!
pub fn bit_decomp_inv<T: RingElement>(bits: &mut Vec<T>) {
    let out_len = bits.len() / T::Num_Bits;
    let mut tmp = Vec::with_capacity(out_len);

    for chunk in bits.chunks(T::Num_Bits) {
        let mut repr: u64 = 0;

        for (i, bit) in chunk.iter().enumerate() {
            if *bit == T::one() {
                repr |= 1u64 << i;
            }
        }

        tmp.push(T::from(repr));
    }

    *bits = tmp;
}

/// "When A is a matrix, let BitDecomp(A), BitDecomp−1 , or Flatten(A) be 
/// the matrix formed by applying the operation to each row of A separately"
pub fn bit_decomp_inv_matrix<T: RingElement + 'static>(a_matrix: &mut DMatrix<T>) {
    let mut vec_of_vec = dmatrix_to_vec_of_vecs(a_matrix);
    vec_of_vec.iter_mut().for_each(|row| bit_decomp_inv(row));
    *a_matrix = vec_of_vecs_to_dmatrix(vec_of_vec);
}

pub fn flatten<T: RingElement>(bits: &mut Vec<T>) {
    bit_decomp_inv(bits);
    bit_decomp(bits);
}

// "When A is a matrix, let BitDecomp(A), BitDecomp−1 , or Flatten(A) be 
// the matrix formed by applying the operation to each row of A separately" 
pub fn flatten_matrix<T: RingElement + 'static>(a_matrix: &mut DMatrix<T>) {
    let mut vec_of_vec = dmatrix_to_vec_of_vecs(a_matrix);
    vec_of_vec.iter_mut().for_each(|row| flatten(row));
    *a_matrix = vec_of_vecs_to_dmatrix(vec_of_vec);
}

/// PowersOf2: (b_1, 2b_1, ..., 2^{l-1}b_1, ..., b_k, ..., 2^{l-1}b_k)
pub fn powers_of_2<T: RingElement + 'static>(b: &DVector<T>, gadget_vector: &DVector<T>) -> DVector<T> {
    let mut out = DVector::from_element(b.len() * gadget_vector.len(), T::zero()); // Initialize with zero

    let mut index = 0;
    for x in b.iter() {
        for g in gadget_vector.iter() {
            out[index] = *x * *g;
            index += 1;
        }
    }

    out
}


pub fn dmatrix_to_vec_of_vecs<T: RingElement>(matrix: &DMatrix<T>) -> Vec<Vec<T>> {
    matrix
        .as_slice()  
        .chunks(matrix.ncols()) 
        .map(|chunk| chunk.to_vec()) 
        .collect()  
}

pub fn vec_of_vecs_to_dmatrix<T: RingElement + 'static>(vec_of_vecs: Vec<Vec<T>>) -> DMatrix<T> {
    let nrows = vec_of_vecs.len();
    let ncols = vec_of_vecs.get(0).map_or(0, |row| row.len()); 

    let flat_vec: Vec<T> = vec_of_vecs.into_iter().flat_map(|row| row.into_iter()).collect();

    DMatrix::from_vec(nrows, ncols, flat_vec)
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
    fn test_bit_decomp_and_inv() {
        for _ in 0..10 {
            let mut rng = rand::rng();
            let input: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();

            let mut decomposed = input.clone();
            bit_decomp(&mut decomposed);
            assert_ne!(decomposed, input);
            assert_eq!(decomposed.len(), input.len()*Fp::Num_Bits);

            let mut reconstructed = decomposed.clone();
            bit_decomp_inv(&mut reconstructed);
            assert_eq!(reconstructed.len(), input.len());
            assert_eq!(input, reconstructed);

            let matrix = vec_of_vecs_to_dmatrix(vec![vec![Fp::from(2u64),Fp::from(2u64)],vec![Fp::from(2u64),Fp::from(2u64)]]);
            let mut decomp_matrix = matrix.clone();
            bit_decomp_matrix(&mut decomp_matrix);
            assert_ne!(matrix, decomp_matrix);
            bit_decomp_inv_matrix(&mut decomp_matrix);
            assert_eq!(matrix, decomp_matrix);
        }
    }

    #[test]
    fn test_scalar_product_invariant() {
        let mut rng = rand::rng();
        
        for _ in 0..10{
            let a: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();
            let b: Vec<Fp> = (0..10).map(|_| Fp::from(rng.random::<u64>())).collect();

            let mut bd_a = a.clone();
            bit_decomp(&mut bd_a);
            let po2_b = powers_of_2(&DVector::from_vec(b.clone()), &GADGET_VECTOR);
            assert_eq!(bd_a.len(), po2_b.len(), "Length mismatch in decomp vs powers_of_two.");

            let dot_decomp = DVector::from_vec(bd_a).dot(&po2_b);
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

        let result = mult_matrix_vector_fp(&matrix, &vector);
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

        let result = mult_matrix_matrix_fp(&a, &b);
        let expected = vec![
            vec![Fp::from(19), Fp::from(22)], // [1*5 + 2*7, 1*6 + 2*8]
            vec![Fp::from(43), Fp::from(50)], // [3*5 + 4*7, 3*6 + 4*8]
        ];
        assert_eq!(result, expected);
    }

}
