use crate::{field::Fp, gsw::gsw::mult_matrix_vector_fp, gsw::gsw::add_vec_vec_fp};


/// Contains all components used in generating the public key.
/// (B,e as well for testing purposes!!)
/// b = Bt+e
/// pk = b||B
pub struct GswPk {
    pub b: Vec<Fp>,
    pub A: Vec<Vec<Fp>>
}


/// B mxn-dim random matrix \in \mathbb(Z)_p
/// e error vector \in \mathbb(Z)_p
/// t == SK.t
impl GswPk {
        pub fn new(B: &Vec<Vec<Fp>>, e: &Vec<Fp>, t: &Vec<Fp>) -> Self {
        let b = add_vec_vec_fp(&mult_matrix_vector_fp(&B, &t), &e);

        let A = B.iter()
            .enumerate()
            .map(|(i, row)| {
                let mut new_row = Vec::with_capacity(row.len() + 1);
                new_row.push(b[i]);
                new_row.extend(row.iter().cloned());
                new_row
            })
            .collect();

        Self { b, A }
    }
}
