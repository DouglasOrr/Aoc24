use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    // Parse
    let data = fs::read_to_string("inputs/day19/input.txt").unwrap();
    let parts: Vec<&str> = data.split("\n\n").collect();
    let patterns: Vec<&[u8]> = parts[0].split(", ").map(|x| x.as_bytes()).collect();
    let designs: Vec<&[u8]> = parts[1].split("\n").map(|x| x.as_bytes()).collect();

    // A trie would be better, but the same search without prefix sharing is a bit easier

    // Part one
    let mut p1_count = 0;
    for design in designs.iter() {
        let terminal_pointers: HashSet<(usize, usize)> =
            (0..(patterns.len())).map(|i| (i, 0)).collect();
        let mut pointers = terminal_pointers.clone();
        for ch in *design {
            let mut next_pointers = HashSet::new();
            for (p, i) in pointers.iter() {
                if patterns[*p][*i] == *ch {
                    if i + 1 == patterns[*p].len() {
                        next_pointers.extend(terminal_pointers.iter());
                    } else {
                        next_pointers.insert((*p, i + 1));
                    }
                }
            }
            pointers = next_pointers;
        }
        p1_count += pointers.iter().any(|(_, i)| *i == 0) as i32;
    }
    println!("part one: {p1_count}");

    // Part two
    let mut p2_count = 0_usize;
    for design in designs.iter() {
        let terminal_pointers: Vec<(usize, usize)> =
            (0..(patterns.len())).map(|i| (i, 0)).collect();
        let mut pointers: HashMap<(usize, usize), usize> =
            terminal_pointers.iter().map(|pi| (*pi, 1)).collect();
        for ch in *design {
            let mut next_pointers = HashMap::new();
            for ((p, i), count) in pointers.iter() {
                if patterns[*p][*i] == *ch {
                    if i + 1 == patterns[*p].len() {
                        for t in terminal_pointers.iter() {
                            *next_pointers.entry(*t).or_insert(0) += count;
                        }
                    } else {
                        *next_pointers.entry((*p, i + 1)).or_insert(0) += count;
                    }
                }
            }
            pointers = next_pointers;
        }
        // All root nodes should have the same count, so just take the first, if it exists
        p2_count += pointers
            .iter()
            .find(|((_, i), _)| (*i == 0))
            .map_or(0, |(_, count)| *count);
    }
    println!("part two: {p2_count}");
}
