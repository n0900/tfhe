pub mod dimacs;

use std::{collections::HashSet};

use ff::Field;

use crate::{field::{Fp, P}, zo_sss::dimacs::DIMACS};
use crate::{misc::rnd_fp};

#[derive(Clone)]
pub struct Party {
    pub name: u8,
    pub shares: Vec<Fp>
}

/// This follows Appendix C.2 of Boneh et al. "Threshold Cryptosystems From
/// Threshold Fully Homomorphic Encryption"
/// 
/// We leverage that if the Monotone boolean formula is given in CNF we can describe the 
/// share matrix vectors by the following code
/// The matrix has num_clauses columns and 
/// 
/// for i in 0..num {
///     row = vec![Fp::ZERO; num];
///     if i == 0 {
///         row[0] = Fp::ONE;
///         row[1] = Fp::ONE;
///     } else {
///         row[i] = minus_one;
///         if i != (num-1) { row[i + 1] = Fp::ONE; }
///     }
///     out.push(row);
/// }
/// producing a share matrix like this
/// 
/// 1  1  0 ... 0
/// ...
/// 1  1  0 ... 0
/// 0 -1  1 ... 0
/// ...
/// This allows us to never having to calculate the full matrix
/// or secret||random vector.
pub fn share(secret: Fp, dimacs: &DIMACS) -> Vec<Party> {
    let num = dimacs.num_clauses as usize;
    let mut w: Vec<Fp> = vec![];
    let mut var1 = secret;
    let mut var2 = Fp::ONE;//rnd_fp(0, P-1);
    let mut out: Vec<Party> = vec![];

    for _ in 0..num-1 {
        w.push(var1 + var2);
        var1 = -var2;
        var2 = Fp::ONE;//rnd_fp(0, P-1);
    }
    w.push(var1);
    
    for (i,p) in dimacs.partitions.iter().enumerate() {
        let shares = p.into_iter().map(|j | w[*j as usize]).collect();
        out.push(Party { name: (i+1) as u8, shares });
    }
    out
}

/// Assume parties are identified by numbers 0..N-1
/// We then find a min cardinality subset of parties
/// to use {0,1} LSSS property
pub fn combine(parties: Vec<Party>, dimacs: &DIMACS) -> Fp {
    let min_set_names: HashSet<u8> = find_min_sat(parties.iter().map(|p| p.name as u8).collect(), dimacs).unwrap();

    let min_set: Vec<Party> = parties
        .iter()
        .filter(|p| min_set_names.contains(&(p.name as u8)))
        .cloned()
        .collect();

    let mut all_shares: Vec<Fp> = min_set
        .iter()
        .flat_map(|p| p.shares.iter().cloned())
        .collect();

    // Cannot use HashSet<Fp> bc Fp does not implement Hash
    // -> manual deduplication
    all_shares.sort();
    all_shares.dedup();

    all_shares.iter().sum()
}

fn find_min_sat(party: HashSet<u8>, dimacs: &DIMACS) -> Option<HashSet<u8>> {
    let mut current_best = Some(party.clone());

    if check_sat(&party, dimacs) {
        if party.len() > 1 {
            for &p in party.iter() {
                let mut new_party = party.clone();
                new_party.remove(&p);

                if let Some(res) = find_min_sat(new_party, dimacs) {
                    current_best = Some(res);
                    break;
                }
            }
        }
        current_best
    } else {
        None
    }
}

fn check_sat(parties: &HashSet<u8>, dimacs: &DIMACS) -> bool {
    for cl in &dimacs.clauses {
        if !parties.iter().any(|it| cl.contains(it)) {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use ff::Field;

    use crate::{field::{Fp, P}, misc::rnd_fp, zo_sss::{combine, dimacs::{DIMACS, DIMACS_2_OF_3_SCHEME, DIMACS_AB_OR_CD}, find_min_sat, share, Party}};

    #[test]
    fn share_test_two_of_three() {
        let secret = rnd_fp(0, P-1);
        let dimacs = DIMACS::parse(DIMACS_2_OF_3_SCHEME);
        let parties = share(secret, &dimacs);
        assert_eq!(parties.len(), 3);
        for p in parties {
            assert_eq!(p.shares.len(), 2);
        }
    }

    #[test]
    fn share_test_AB_or_CD() {
        let secret = rnd_fp(0, P-1);
        let dimacs = DIMACS::parse(DIMACS_AB_OR_CD);
        let parties = share(secret, &dimacs);
        assert_eq!(parties.len(), 4);
        for p in parties {
            assert_eq!(p.shares.len(), 2);
        }
    }

    #[test]
    fn secret_sharing_2_of_3_test(){
        let secret = Fp::ZERO;//rnd_fp(0, P-1);
        let dimacs = DIMACS::parse(DIMACS_2_OF_3_SCHEME);
        let parties = share(secret, &dimacs);
        let hashset_of_party_names = parties.iter().map(|p| p.name).collect();
        let subset = find_min_sat(hashset_of_party_names, &dimacs).unwrap();
        assert_eq!(subset.len(), 2);
        let result = combine(parties, &dimacs);
        assert_eq!(result, secret);
    }

    #[test]
    fn secret_sharing_AB_CD_test(){
        let secret = rnd_fp(0, P-1);
        let dimacs = DIMACS::parse(DIMACS_AB_OR_CD);
        let parties = share(secret, &dimacs);
        let hashset_of_party_names = parties.iter().map(|p| p.name).collect();
        let subset = find_min_sat(hashset_of_party_names, &dimacs).unwrap();
        assert_eq!(subset.len(), 2);
        let result = combine(parties, &dimacs);
        assert_eq!(result, secret);
    }
}
