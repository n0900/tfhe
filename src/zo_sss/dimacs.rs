use std::collections::HashSet;


/// (A or B) and (A or C) and (B or C)
pub const DIMACS_2_OF_3_SCHEME: &str = "\
c Example DIMACS CNF
p cnf 3 3
1 2 0
1 3 0
2 3 0
";

pub const DIMACS_AB_OR_CD: &str = "\
c Example non-k-of-n Access structure
p cnf 4 4 
1 3 0
1 4 0
2 3 0
2 4 0
";


pub struct DIMACS {
    pub num_var: u8,
    pub num_clauses: u8,
    pub clauses: Vec<Vec<u8>>,
    // Partitions T_i denotes shares of share matrix associated with the party (identified via index)
    pub partitions: Vec<HashSet<u8>>
}

impl DIMACS {
    pub fn parse(input: &str) -> Self {
        let mut clauses = vec![];
        let mut num_var: Option<u8> = None;
        let mut num_clauses: Option<u8> = None;

        for line in input.lines() {
            let line = line.trim();
            //ignore comments
            if line.is_empty() || line.starts_with('c') {
                continue;
            }
            //header 
            else if line.starts_with('p') {
                (num_var, num_clauses) = parse_header(line);
            } 
            //body
            else {
                let clause: Vec<u8> = line.split_whitespace()
                                        .map(|s| s.parse::<u8>().unwrap())
                                        .take_while(|&n| n != 0)
                                        .collect();
                clauses.push(clause);
            }
        }
        let partitions = get_partitions(num_var.unwrap(), &clauses);

        DIMACS { num_var: num_var.unwrap(), num_clauses: num_clauses.unwrap(), clauses: clauses, partitions: partitions }
    }
}


fn get_partitions(num_var: u8, clauses: &Vec<Vec<u8>>) -> Vec<HashSet<u8>> {
    let mut partitions: Vec<HashSet<u8>> = Vec::with_capacity(num_var as usize);
    for i in 1..num_var+1 {
        let mut partition: HashSet<u8> = HashSet::new();
        for (j, cl) in clauses.iter().enumerate() {
            if cl.contains(&i) { partition.insert(j as u8); }
        }
        partitions.push(partition);
    }

    partitions
}

fn parse_header(line: &str) -> (Option<u8>, Option<u8>) {
    let parsed:Vec<u8> = line.split_whitespace()
        .skip(2)
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    assert_eq!(parsed.len(), 2);
    (Some(parsed[0]), Some(parsed[1]))
}