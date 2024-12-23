use std::fs::File;
use std::io::{self, BufRead};

fn is_safe_dampened(ints: &Vec<i32>) -> bool {
    // Brute force
    // (note - an unnecessary skip is always fine)
    for skip in 0..ints.len() {
        let mut safe = true;
        let mut ascending = true;
        let mut descending = true;
        for i in 0..ints.len() {
            let j = i + 1 + ((i + 1) == skip) as usize;
            if i != skip && j < ints.len() {
                ascending &= ints[i] < ints[j];
                descending &= ints[j] < ints[i];
                safe &= (ints[i] - ints[j]).abs() <= 3;
            }
        }
        if safe && (ascending || descending) {
            return true;
        }
    }
    return false;
}

fn main() {
    let mut safe_count = 0;
    let mut dampened_safe_count = 0;
    for line in io::BufReader::new(File::open("inputs/day02/input.txt").unwrap())
        .lines()
        .flatten()
    {
        let ints: Vec<i32> = line.split(" ").map(|s| s.parse::<i32>().unwrap()).collect();

        // Part one
        let safe = (ints.iter().is_sorted() || ints.iter().is_sorted_by(|a, b| b <= a))
            && (ints
                .iter()
                .zip(ints.iter().skip(1))
                .all(|(a, b)| a != b && (a - b).abs() <= 3));
        safe_count += safe as i32;

        // Part two
        dampened_safe_count += is_safe_dampened(&ints) as i32;
    }
    println!("part one: {safe_count}");
    println!("part two: {dampened_safe_count}");
}
