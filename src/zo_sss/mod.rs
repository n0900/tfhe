pub mod dimacs;
pub mod mbf;


use crate::{field::Fp, zo_sss::{dimacs::DIMACS, mbf::{mbf_combine, mbf_share}}};

#[derive(Clone)]
pub struct Party {
    pub name: u8,
    pub shares: Vec<Vec<Fp>>
}

pub trait SecretSharingScheme {
    fn share(&self, secrets: Vec<Fp>) -> Vec<Party>;
    fn combine(&self, parties: Vec<Party>, is_minimal: bool) -> Vec<Fp>;
}

/// It is the users responsibility to use a correct dimacs!
/// Only checks for k-out-of-n
pub struct Shamir {
    _n: u8,
    k: u8,
    dimacs: DIMACS
}

impl SecretSharingScheme for Shamir {
    fn share(&self, secrets: Vec<Fp>) -> Vec<Party> {
        mbf_share(secrets, &self.dimacs)
    }

    fn combine(&self, parties: Vec<Party>, _is_minimal: bool) -> Vec<Fp> {
        let subset: Vec<Party> = if parties.len() >= (self.k as usize) {
            parties.iter().take(self.k as usize).cloned().collect()
        } else { panic!("Invalid party size") };
        mbf_combine(subset, true, &self.dimacs)
    }
}

pub struct MBF {
    dimacs: DIMACS
}

impl SecretSharingScheme for MBF {
    fn share(&self, secrets: Vec<Fp>) -> Vec<Party> {
        mbf_share(secrets, &self.dimacs)
    }

    fn combine(&self, parties: Vec<Party>, is_minimal: bool) -> Vec<Fp> {
        mbf_combine(parties, is_minimal, &self.dimacs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{field::P, misc::rnd_fp_vec, zo_sss::{dimacs::{DIMACS, DIMACS_2_OF_3_SCHEME, DIMACS_AB_OR_CD}, SecretSharingScheme, Shamir, MBF}};

    #[test]
    fn shamir_struct_test() {
        let shamir = Shamir { _n:3, k:2, dimacs: DIMACS::parse(DIMACS_2_OF_3_SCHEME) };
        sss_test(shamir);
    }

    #[test]
    fn mbf_struct_test() {
        let mbf = MBF { dimacs: DIMACS::parse(DIMACS_AB_OR_CD) };
        sss_test(mbf);
    }

    fn sss_test<T: SecretSharingScheme>(scheme: T) {
        let secrets = rnd_fp_vec(5, 0, P-1);
        let copy_secrets = secrets.clone();
        let parties = scheme.share(secrets);
        let result = scheme.combine(parties, false);
        assert_eq!(copy_secrets, result);
    }
}