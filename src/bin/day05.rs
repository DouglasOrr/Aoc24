use regex;
use std::{collections::HashSet, fs};

fn main() {
    let mul_right = 1 << 10;
    let mut disallow: HashSet<i32> = HashSet::new();
    let mut sort_order: HashSet<(i32, i32)> = HashSet::new();
    let rule = regex::Regex::new(r"(\d+)\|(\d+)").unwrap();
    let mut p1_sum = 0;
    let mut p2_sum = 0;
    for line in fs::read_to_string("inputs/day05/input.txt")
        .unwrap()
        .split("\n")
    {
        if line.contains("|") {
            let m = rule.captures(line).unwrap();
            let left: i32 = m.get(1).unwrap().as_str().parse().unwrap();
            let right: i32 = m.get(2).unwrap().as_str().parse().unwrap();
            assert!(right < mul_right && left < mul_right);
            disallow.insert(right + left * mul_right);
            sort_order.insert((left, right));
        } else if !line.is_empty() {
            let mut pages: Vec<i32> = line.split(",").map(|p| p.parse::<i32>().unwrap()).collect();
            let mut ok = true;
            'outer: for i in 0..pages.len() {
                for j in (i + 1)..pages.len() {
                    if disallow.contains(&(pages[i] + pages[j] * mul_right)) {
                        ok = false;
                        break 'outer;
                    }
                }
            }
            if ok {
                p1_sum += pages[pages.len() / 2];
            } else {
                // Kindof like an insertion/bubble sort (is it guaranteed to terminate?)
                while !ok {
                    ok = true;
                    for i in 0..pages.len() {
                        for j in (i + 1)..pages.len() {
                            if disallow.contains(&(pages[i] + pages[j] * mul_right)) {
                                let pj = pages.remove(j);
                                pages.insert(i, pj);
                                ok = false;
                            }
                        }
                    }
                }
                p2_sum += pages[pages.len() / 2];
            }
        }
    }
    println!("part one: {p1_sum}");
    println!("part two: {p2_sum}");
}
