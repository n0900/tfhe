use nalgebra::{DMatrix, DVector};

use crate::{field::Fp};


/// Contains all components used in generating the public key.
/// (B,e as well for testing purposes!!)
/// b = Bt+e
/// pk = b||B
/// 
#[derive(PartialEq)]
#[derive(Debug)]
pub struct GswPk {
    pub b: DVector<Fp>,
    pub pk_matrix: DMatrix<Fp>
}


/// B mxn-dim random matrix \in \mathbb(Z)_p
/// e error vector \in \mathbb(Z)_p
/// t == SK.t
impl GswPk {
    pub fn new(random_matrix: &DMatrix<Fp>, e: &DVector<Fp>, t: &DVector<Fp>) -> Self {
        assert_eq!(random_matrix.ncols(), t.len(), "Dimension Mismatch ncols {}, {}", random_matrix.ncols(), t.len());
        assert_eq!(random_matrix.nrows(), e.len(), "Dimension Mismatch nrows {}, {}", random_matrix.nrows(), e.len());
        let b: DVector<Fp> = random_matrix * t + e;

        let mut pk_matrix = DMatrix::zeros(random_matrix.nrows(), random_matrix.ncols() + 1);

        // Set first column to b
        pk_matrix.set_column(0, &b);

        // Copy the rest of the random_matrix into remaining columns
        pk_matrix
            .view_mut((0, 1), (random_matrix.nrows(), random_matrix.ncols()))
            .copy_from(random_matrix);

        Self { b, pk_matrix }
    }
}


#[cfg(test)]
mod tests {
    use nalgebra::DMatrix;

    use crate::{error_sampling::{rnd_fp_dmatrix, rnd_fp_dvec}, field::{Fp, P}, gsw::pk::GswPk};

    #[test]
    fn different_err_lead_to_diff_pk() {  
        let n = 100;
        let m = 50;  
        let t =rnd_fp_dvec(n, 0, P-1);
        let random_matrix: DMatrix<Fp> = rnd_fp_dmatrix(m,n,0,1);
        
        let err1 = rnd_fp_dvec(m as usize, 0, P/2);

        let err2 = rnd_fp_dvec(m as usize, 0, P/2);

        let pk1 = GswPk::new(&random_matrix, &err1, &t);
        let pk2 = GswPk::new(&random_matrix, &err2, &t);

        assert_ne!(pk1, pk2);
    }
}
