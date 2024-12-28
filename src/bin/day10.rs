use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    // Parse
    let data = fs::read_to_string("inputs/day10/input.txt").unwrap();
    let size = data.split("\n").count();
    let height: Vec<u8> = data
        .replace("\n", "")
        .as_bytes()
        .iter()
        .map(|x| x - b'0')
        .collect();

    // Part one
    let mut p1_count = 0;
    for start in 0..size.pow(2) {
        if height[start] == 0 {
            let mut reachable = HashSet::from([start]);
            for h in 1..=9 {
                let mut next_reachable = HashSet::new();
                for idx in reachable.iter() {
                    let x = idx % size;
                    let y = idx / size;
                    if x > 0 && height[idx - 1] == h {
                        next_reachable.insert(idx - 1);
                    }
                    if x < size - 1 && height[idx + 1] == h {
                        next_reachable.insert(idx + 1);
                    }
                    if y > 0 && height[idx - size] == h {
                        next_reachable.insert(idx - size);
                    }
                    if y < size - 1 && height[idx + size] == h {
                        next_reachable.insert(idx + size);
                    }
                }
                reachable = next_reachable;
            }
            p1_count += reachable.len();
        }
    }
    println!("part one: {p1_count}");

    // Part two
    let mut p2_count = 0;
    for start in 0..size.pow(2) {
        if height[start] == 0 {
            let mut counts = HashMap::from([(start, 1)]);
            for h in 1..=9 {
                let mut next_counts = HashMap::new();
                for (idx, count) in counts.iter() {
                    let x = idx % size;
                    let y = idx / size;
                    if x > 0 && height[idx - 1] == h {
                        *next_counts.entry(idx - 1).or_insert(0) += count;
                    }
                    if x < size - 1 && height[idx + 1] == h {
                        *next_counts.entry(idx + 1).or_insert(0) += count;
                    }
                    if y > 0 && height[idx - size] == h {
                        *next_counts.entry(idx - size).or_insert(0) += count;
                    }
                    if y < size - 1 && height[idx + size] == h {
                        *next_counts.entry(idx + size).or_insert(0) += count;
                    }
                }
                counts = next_counts;
            }
            p2_count += counts.values().sum::<i32>();
        }
    }
    println!("part two: {p2_count}");
}
