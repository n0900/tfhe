
pub struct DIMACS {
    pub num_var: u8,
    pub num_clauses: u8,
    pub clauses: Vec<Vec<u8>>
}

/// (A or B) and (A or C) and (B or C)
pub const DIMACS_2_OF_3_SCHEME: &str = "\
c Example DIMACS CNF
p cnf 3 3
1 2 0
1 3 0
2 3 0
";

pub fn parse_dimacs(input: &str) -> DIMACS {
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
    DIMACS { num_var: num_var.unwrap(), num_clauses: num_clauses.unwrap(), clauses: clauses }
}

fn parse_header(line: &str) -> (Option<u8>, Option<u8>) {
    let parsed:Vec<u8> = line.split_whitespace()
        .skip(2)
        .map(|s| s.parse::<u8>().unwrap())
        .collect();
    assert_eq!(parsed.len(), 2);
    (Some(parsed[0]), Some(parsed[1]))
}
