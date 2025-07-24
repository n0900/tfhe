pub mod share_matrix;
pub mod dimacs;

use std::collections::HashSet;

use crate::{field::{Fp, P}, misc::rnd_fp_vec};

//pub fn share(secret: Fp, access_struct: &AccessStructure) {
//    let l: u32 = 100; //TODO find correct/adequate number
//    let mut rng = rand::rng();
//    let share_matrix: Vec<Vec<Fp>> = (0..l).map(|_| 
//        rnd_fp_vec(access_struct.parties.len(), 0, P-1)).collect();    
//}