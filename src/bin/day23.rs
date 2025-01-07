use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn find_max_party<'a>(
    edges: &HashMap<&str, HashSet<&'a str>>,
    open: &HashSet<&'a str>,
    party: &mut Vec<&'a str>,
    max_party: &mut Vec<&'a str>,
) {
    if party.len() > max_party.len() {
        max_party.clone_from(party);
    }
    for p in open.iter() {
        if party.last().unwrap() < p {
            party.push(p);
            let next_open: HashSet<&str> = open
                .intersection(edges.get(p).unwrap())
                .map(|x| *x)
                .collect();
            find_max_party(edges, &next_open, party, max_party);
            party.pop();
        }
    }
}

fn main() {
    // Parse
    let data = fs::read_to_string("inputs/day23/input.txt").unwrap();
    let mut edges: HashMap<&str, HashSet<&str>> = HashMap::new();
    for line in data.split("\n") {
        let p: Vec<&str> = line.split("-").collect();
        edges.entry(p[0]).or_insert(HashSet::new()).insert(p[1]);
        edges.entry(p[1]).or_insert(HashSet::new()).insert(p[0]);
    }

    // Part one
    let mut p1_count_x6 = 0; // count all permutations
    for (n1, tails1) in edges.iter() {
        for n2 in tails1 {
            for n3 in edges.get(n2).unwrap() {
                if tails1.contains(n3)
                    && (n1.starts_with("t") || n2.starts_with("t") || n3.starts_with("t"))
                {
                    p1_count_x6 += 1;
                }
            }
        }
    }
    println!("part one: {}", p1_count_x6 / 6);

    // Part two
    let mut max_party = Vec::new();
    for (root, root_edges) in edges.iter() {
        find_max_party(&edges, root_edges, &mut vec![root], &mut max_party);
    }
    println!("part two: {}", max_party.join(","));
}
