use crate::{field::Fp, gsw::gsw::mult_matrix_vector_fp, gsw::gsw::add_vec_vec_fp};


/// Contains all components used in generating the public key.
/// (B,e as well for testing purposes!!)
/// b = Bt+e
/// pk = b||B
/// 
#[derive(PartialEq)]
#[derive(Debug)]
pub struct GswPk {
    pub b: Vec<Fp>,
    pub pk_matrix: Vec<Vec<Fp>>
}


/// B mxn-dim random matrix \in \mathbb(Z)_p
/// e error vector \in \mathbb(Z)_p
/// t == SK.t
impl GswPk {
        pub fn new(random_matrix: &Vec<Vec<Fp>>, e: &Vec<Fp>, t: &Vec<Fp>) -> Self {
            let b = add_vec_vec_fp(&mult_matrix_vector_fp(&random_matrix, &t), &e);

            let pk_matrix = random_matrix.iter()
                .enumerate()
                .map(|(i, row)| {
                    let mut new_row = Vec::with_capacity(row.len() + 1);
                    new_row.push(b[i]);
                    new_row.extend(row.iter().cloned());
                    new_row
                })
                .collect();

            Self { b, pk_matrix }
        }
}


#[cfg(test)]
mod tests {
    use crate::{error_sampling::rnd_fp_vec, field::{Fp, P}, gsw::{pk::GswPk}};


    #[test]
    fn different_err_lead_to_diff_pk() {  
        let n = 100;
        let m = 50;  
        let t =rnd_fp_vec(n, 0, P-1);
        let random_matrix: Vec<Vec<Fp>> = (0..m)
            .map(|_| rnd_fp_vec(n as usize, 0, P-1))
            .collect();
        
        let err1 = rnd_fp_vec(m as usize, 0, P/2);

        let err2 = rnd_fp_vec(m as usize, 0, P/2);

        let pk1 = GswPk::new(&random_matrix, &err1, &t);
        let pk2 = GswPk::new(&random_matrix, &err2, &t);

        assert_ne!(pk1, pk2);
    }
}
