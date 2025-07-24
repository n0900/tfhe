use ff::Field;

use crate::zo_sss::dimacs::{self, parse_dimacs, DIMACS, DIMACS_2_OF_3_SCHEME};
use crate::field::Fp;

/// This definition follows "Threshold Cryptosystems From Threshold Fully Homomorphic Encryption"
/// Dan Boneh et al.
/// Appendix C.2

/// We assume that the monotone boolean formula is given in DIMACS CNF
/// (Monotone means no negation)
struct ShareMatrix {
    pub dimacs: DIMACS,
    pub matrix: Vec<Vec<Fp>>
}

impl ShareMatrix {
    pub fn from_dimacs(input: &str) -> ShareMatrix {
        let dimacs = parse_dimacs(&input);
        let mut first_label: Vec<Fp> = vec![Fp::ONE, Fp::ONE];
        let mut second_label: Vec<Fp> = vec![Fp::ZERO, Fp::ONE.invert().unwrap()];
        let mut matrix: Vec<Vec<Fp>> = vec![];

        first_label.push(Fp::ONE);
        second_label.push(Fp::ZERO);
        // all AND gates have 
        for _ in (1..dimacs.clauses.len()/2) {
            matrix.push(first_label.clone());
            matrix.push(second_label.clone());
            first_label.push(Fp::ONE);
            second_label.insert(0, Fp::ZERO);
        }

        ShareMatrix { dimacs: dimacs, matrix: pad_matrix(matrix) }
    }

    pub fn two_of_three() -> ShareMatrix {
        ShareMatrix::from_dimacs(&DIMACS_2_OF_3_SCHEME)
    }
}

// Pad each row with Fp::ZERO to match max_len
fn pad_matrix(matrix: Vec<Vec<Fp>>) -> Vec<Vec<Fp>> {
    let max_len = matrix.iter().map(|row| row.len()).max().unwrap();

    matrix.into_iter()
          .map(|mut row| {
              while row.len() < max_len {
                  row.push(Fp::ZERO);
              }
              row
          })
          .collect()
}


#[cfg(test)]
mod tests {
    use crate::zo_sss::share_matrix::ShareMatrix;

    #[test]
    fn two_of_three_can_be_parsed() {
        let share_matrix = ShareMatrix::two_of_three();
        assert_eq!(share_matrix.matrix.len(), share_matrix.dimacs.num_clauses as usize)
    }
}