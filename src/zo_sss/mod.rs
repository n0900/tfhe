pub mod dimacs;
pub mod mbf;


use crate::{field::Fp, zo_sss::{dimacs::DIMACS, mbf::{mbf_combine, mbf_share}}};

#[derive(Clone)]
pub struct Party {
    pub name: u8,
    pub shares: Vec<Vec<Fp>>
}

pub trait SecretSharingScheme {
    type ShareParameters;
    fn share(parameter: Self::ShareParameters) -> Vec<Party>;
    
    type CombineParameters;
    fn combine(parameter: Self::CombineParameters) -> Vec<Fp>;
}

pub struct MBF;

impl SecretSharingScheme for MBF {
    type ShareParameters = (Vec<Fp>, DIMACS);
    fn share(parameter: Self::ShareParameters) -> Vec<Party> {
        mbf_share(parameter.0, &parameter.1)
    }

    type CombineParameters = (Vec<Party>, bool, DIMACS);
    fn combine(parameter: Self::CombineParameters) -> Vec<Fp> {
        mbf_combine(parameter.0, parameter.1, &parameter.2)
    }
}