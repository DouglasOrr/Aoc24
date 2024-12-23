use regex::Regex;
use std::{collections::HashMap, fs};

fn main() {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let pattern = Regex::new(r"(\d+)   (\d+)").unwrap();
    for line in fs::read_to_string("inputs/day1/input.txt").unwrap().lines() {
        let g = pattern.captures(line).unwrap();
        left.push(g.get(1).unwrap().as_str().parse::<i32>().unwrap());
        right.push(g.get(2).unwrap().as_str().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();
    let total_distance: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(a, b)| (a - b).abs())
        .sum();
    println!("part one: {}", total_distance);

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for x in right {
        *counts.entry(x).or_insert(0) += 1;
    }
    let similarity_score: i32 = left.iter().map(|x| x * counts.get(x).unwrap_or(&0)).sum();
    println!("part two: {}", similarity_score);
}
