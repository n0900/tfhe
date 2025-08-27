use std::collections::HashSet;

use crate::{error_sampling::rnd_fp, field::{Fp, P}, zo_sss::{dimacs::DIMACS, Party}};

/// Secret Sharing via Monotone Boolean Formula Access Structure
/// Access Structure is fully defined via DIMACS.
/// # Parameters
/// - `secrets`: A set of secrets in Fp.
/// - `dimacs`: The monotone boolean formula (MBF) defining the access structure.
pub fn mbf_share(secrets: Vec<Fp>, dimacs: &DIMACS) -> Vec<Party<Fp>> {
    let num = dimacs.num_clauses as usize;
    let w_matrix: Vec<Vec<Fp>> = build_w_matrix(secrets, num);
    get_parties(w_matrix, dimacs)
}

fn get_parties(w_matrix: Vec<Vec<Fp>>, dimacs: &DIMACS) -> Vec<Party<Fp>> {
    dimacs.partitions
        .iter()
        .enumerate()
        .map(|(i, p)| {
            let shares = w_matrix
                .iter()
                .map(|w| p.iter().map(|&j| w[j as usize]).collect())
                .collect();
            Party { name: (i + 1) as u8, shares }
        })
        .collect()
}

fn build_w_matrix(secrets: Vec<Fp>, num: usize) -> Vec<Vec<Fp>> {
    secrets
        .into_iter()
        .map(|secret| {
            build_w(secret, num)
        })
        .collect()
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
fn build_w(secret: Fp, num: usize) -> Vec<Fp> {
    let mut v1 = secret;
    let mut v2 = rnd_fp(0, P - 1);
    let mut w = Vec::with_capacity(num);

    for _ in 0..num - 1 {
        w.push(v1 + v2);
        v1 = -v2;
        v2 = rnd_fp(0, P - 1);
    }
    w.push(v1);
    w
}

/// Assumes parties are named 1 to N and that they have the same number of sets of shares.
/// Finds a minimum-cardinality subset of parties 
/// satisfying the {0,1}-LSSS property.
///
/// # Parameters
/// - `parties`: A set of parties, each with a name and a set of share sets.
/// - `index`: Indicates which secret to reconstruct.
/// - `is_minimal`: If `true`, skips computing the minimal subset.
/// - `dimacs`: The monotone boolean formula (MBF) defining the access structure.
pub fn mbf_combine(parties: Vec<Party<Fp>>, is_minimal: bool, dimacs: &DIMACS) -> Vec<Fp> {
    let min_set: Vec<Party<Fp>> = if !is_minimal {
        get_min_party(&parties, dimacs)
    } else { parties };

    let num_secrets = min_set.first().unwrap().shares.len();
    (0..num_secrets)
        .map(|i| get_party_shares(&min_set, i).iter().sum())
        .collect()
}

pub fn get_min_party(parties: &Vec<Party<Fp>>, dimacs: &DIMACS) -> Vec<Party<Fp>> {
    let min_set_names: HashSet<u8> = find_min_sat(parties.iter().map(|p| p.name as u8).collect(), dimacs).unwrap();
    get_parties_by_name(&parties, &min_set_names)
}

// Cannot use HashSet<Fp> bc Fp does not implement Hash
// -> manual deduplication
fn get_party_shares(parties: &Vec<Party<Fp>>, index: usize) -> Vec<Fp> {
    let mut all_shares: Vec<Fp> = parties.iter()
        .flat_map(|p| p.shares[index].iter().cloned())
        .collect();

    all_shares.sort();
    all_shares.dedup();
    all_shares
}

fn get_parties_by_name(parties: &Vec<Party<Fp>>, names: &HashSet<u8>) -> Vec<Party<Fp>> {
    parties
        .iter()
        .filter(|p| names.contains(&(p.name as u8)))
        .cloned()
        .collect()
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

    use crate::{error_sampling::rnd_fp, field::{Fp, P}, zo_sss::{dimacs::{DIMACS, DIMACS_2_OF_3_SCHEME, DIMACS_AB_OR_CD}, mbf::{get_min_party, mbf_combine, mbf_share}}};

    #[test]
    fn share_test_two_of_three() {
        let secret = rnd_fp(0, P-1);
        let dimacs = DIMACS::parse(DIMACS_2_OF_3_SCHEME);
        let parties = mbf_share(vec![secret], &dimacs);
        assert_eq!(parties.len(), 3);
        for p in parties {
            assert_eq!(p.shares[0].len(), 2);
        }
    }

    #[test]
    fn share_test_ab_or_cd() {
        let secret = rnd_fp(0, P-1);
        let dimacs = DIMACS::parse(DIMACS_AB_OR_CD);
        let parties = mbf_share(vec![secret], &dimacs);
        assert_eq!(parties.len(), 4);
        for p in parties {
            assert_eq!(p.shares[0].len(), 2);
        }
    }

    #[test]
    fn secret_sharing_2_of_3_test(){
        let secret = rnd_fp(0, P-1);
        let dimacs = DIMACS::parse(DIMACS_2_OF_3_SCHEME);
        execute_mbf_test(secret, &dimacs);
    }

    #[test]
    fn secret_sharing_ab_cd_test(){
        let secret = rnd_fp(0, P-1);
        let dimacs = DIMACS::parse(DIMACS_AB_OR_CD);
        execute_mbf_test(secret, &dimacs);
    }

    fn execute_mbf_test(secret: Fp, dimacs: &DIMACS) {
        let parties = mbf_share(vec![secret], &dimacs);
        let subset = get_min_party(&parties, &dimacs);
        assert_eq!(subset.len(), 2);
        let result = *mbf_combine(subset, true, &dimacs).first().unwrap();
        assert_eq!(result, secret);

        let result2 = *mbf_combine(parties, false, &dimacs).first().unwrap();
        assert_eq!(result2, secret);
    }
}
