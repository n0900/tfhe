pub mod dimacs;
pub mod mbf;


use crate::{field::Fp, zo_sss::{dimacs::DIMACS, mbf::{mbf_combine, mbf_share}}, RingElement};

#[derive(Clone)]
pub struct Party<R: RingElement> {
    pub name: u8,
    pub shares: Vec<Vec<R>>
}

pub trait SecretSharingScheme<R:RingElement> {
    fn share(&self, secrets: Vec<R>) -> Vec<Party<R>>;
    fn combine(&self, parties: Vec<Party<R>>, is_minimal: bool) -> Vec<R>;
}

pub struct MBF {
    dimacs: DIMACS
}

impl SecretSharingScheme<Fp> for MBF {
    fn share(&self, secrets: Vec<Fp>) -> Vec<Party<Fp>> {
        mbf_share(secrets, &self.dimacs)
    }

    fn combine(&self, parties: Vec<Party<Fp>>, is_minimal: bool) -> Vec<Fp> {
        mbf_combine(parties, is_minimal, &self.dimacs)
    }
}

#[cfg(test)]
mod tests {
    use crate::{error_sampling::rnd_dvec, field::{Fp, P}, zo_sss::{dimacs::{DIMACS, DIMACS_2_OF_3_SCHEME, DIMACS_AB_OR_CD}, SecretSharingScheme, MBF}};

    #[test]
    fn shamir_struct_test() {
        let shamir = MBF { dimacs: DIMACS::parse(DIMACS_2_OF_3_SCHEME) };
        sss_test(shamir);
    }

    #[test]
    fn mbf_struct_test() {
        let mbf = MBF { dimacs: DIMACS::parse(DIMACS_AB_OR_CD) };
        sss_test(mbf);
    }

    fn sss_test<T: SecretSharingScheme<Fp>>(scheme: T) {
        let secrets: Vec<Fp> = rnd_dvec(5, 0, P-1).iter().map(|elm| *elm).collect();
        let copy_secrets = secrets.clone();
        let parties = scheme.share(secrets);
        let result = scheme.combine(parties, false);
        assert_eq!(copy_secrets, result);
    }
}