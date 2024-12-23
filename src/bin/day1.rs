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
    let mut total_distance = 0;
    for (a, b) in left.iter().zip(right.iter()) {
        total_distance += (a - b).abs();
    }
    println!("part one: {}", total_distance);

    let mut counts: HashMap<i32, i32> = HashMap::new();
    for x in right {
        if counts.contains_key(&x) {
            *counts.get_mut(&x).unwrap() += 1;
        } else {
            counts.insert(x, 1);
        }
    }
    let mut similarity_score = 0;
    for x in left {
        similarity_score += x * counts.get(&x).unwrap_or(&0);
    }
    println!("part two: {}", similarity_score);
}
