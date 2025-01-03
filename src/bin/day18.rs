use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    fs,
};

fn path_length(corrupted: &Vec<bool>, size: usize) -> Option<usize> {
    let mut queue: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::from([Reverse((0, 0))]);
    let mut visited: HashSet<usize> = HashSet::new();
    while !queue.is_empty() {
        let Reverse((cost, node)) = queue.pop().unwrap();
        if node == size * size - 1 {
            return Some(cost);
        }
        if visited.contains(&node) {
            continue;
        }
        visited.insert(node);
        if corrupted[node] {
            continue;
        }
        if node % size != 0 {
            queue.push(Reverse((cost + 1, node - 1)));
        }
        if node % size != size - 1 {
            queue.push(Reverse((cost + 1, node + 1)));
        }
        if node / size != 0 {
            queue.push(Reverse((cost + 1, node - size)));
        }
        if node / size != size - 1 {
            queue.push(Reverse((cost + 1, node + size)));
        }
    }
    return None;
}

fn main() {
    // let (path, size, n1) = ("inputs/day18/test.txt", 7, 12);
    let (path, size, n1) = ("inputs/day18/input.txt", 71, 1024);

    // Parse
    let raw_data = fs::read_to_string(path).unwrap();
    let idx: Vec<usize> = raw_data
        .split("\n")
        .map(|line| {
            let parts: Vec<&str> = line.split(",").collect();
            return parts[0].parse::<usize>().unwrap() + size * parts[1].parse::<usize>().unwrap();
        })
        .collect();

    // Part one
    let mut corrupted = vec![false; size * size];
    for i in idx[0..n1].iter() {
        corrupted[*i] = true;
    }
    let p1_length = path_length(&corrupted, size).unwrap();
    println!("part one: {p1_length}");

    // Part two
    // (Could do better than this naive brute force, but "good enough" is the mantra this year...)
    for i in idx[n1..idx.len()].iter() {
        corrupted[*i] = true;
        if path_length(&corrupted, size).is_none() {
            println!("part two: {},{}", i % size, i / size);
            break;
        }
    }
}
