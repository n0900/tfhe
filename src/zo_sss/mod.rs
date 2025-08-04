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
    fn share(&self, parameter: Self::ShareParameters) -> Vec<Party>;
    
    type CombineParameters;
    fn combine(&self, parameter: Self::CombineParameters) -> Vec<Fp>;
}

pub struct MBF {
    dimacs: DIMACS
}

impl SecretSharingScheme for MBF {
    type ShareParameters = Vec<Fp>;
    fn share(&self, parameter: Self::ShareParameters) -> Vec<Party> {
        mbf_share(parameter, &self.dimacs)
    }

    type CombineParameters = (Vec<Party>, bool);
    fn combine(&self, parameter: Self::CombineParameters) -> Vec<Fp> {
        mbf_combine(parameter.0, parameter.1, &self.dimacs)
    }
}


#[cfg(test)]
mod tests {
    use crate::{field::P, misc::rnd_fp_vec, zo_sss::{dimacs::{self, DIMACS, DIMACS_2_OF_3_SCHEME}, SecretSharingScheme, MBF}};

    #[test]
    fn mbf_struct_test() {
        let secrets = rnd_fp_vec(5, 0, P-1);
        let copy_secrets = secrets.clone();
        let mbf = MBF { dimacs: DIMACS::parse(DIMACS_2_OF_3_SCHEME) };
        let parties = mbf.share(secrets);
        let result = mbf.combine((parties, false));
        assert_eq!(copy_secrets, result);
    }
}