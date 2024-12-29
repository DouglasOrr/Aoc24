use std::collections::HashMap;
use std::fs;
use std::time::Instant;

fn main() {
    let text = fs::read_to_string("inputs/day11/input.txt").unwrap();
    let init: Vec<u64> = text.split(" ").map(|x| x.parse::<u64>().unwrap()).collect();

    // Part one
    {
        let start = Instant::now();
        let mut queue = init.clone();
        let mut queue_ttl: Vec<i32> = (0..init.len()).map(|_| 25).collect();
        let mut p1_count = 0;
        let mut idx = 0;
        while idx < queue.len() {
            let value = queue[idx];
            let ttl = queue_ttl[idx];
            if ttl == 0 {
                p1_count += 1;
            } else if value == 0 {
                queue.push(1);
                queue_ttl.push(ttl - 1);
            } else if (1 + value.ilog10()) % 2 == 0 {
                let mask = 10_u64.pow((1 + value.ilog10()) / 2);
                queue.push(value / mask);
                queue_ttl.push(ttl - 1);
                queue.push(value % mask);
                queue_ttl.push(ttl - 1);
            } else {
                queue.push(2024 * value);
                queue_ttl.push(ttl - 1);
            }
            idx += 1;
        }
        println!("part one: {p1_count}  (in {:.0?})", start.elapsed());
    }

    // Part two
    {
        let start = Instant::now();
        let mut histogram = HashMap::new();
        for i in init {
            *histogram.entry(i).or_insert(0) += 1;
        }
        for _ in 0..75 {
            let mut next_histogram = HashMap::new();
            for (value, count) in histogram {
                if value == 0 {
                    *next_histogram.entry(1).or_insert(0) += count;
                } else if (1 + value.ilog10()) % 2 == 0 {
                    let mask = 10_u64.pow((1 + value.ilog10()) / 2);
                    *next_histogram.entry(value / mask).or_insert(0) += count;
                    *next_histogram.entry(value % mask).or_insert(0) += count;
                } else {
                    *next_histogram.entry(value * 2024).or_insert(0) += count;
                }
            }
            histogram = next_histogram;
        }
        println!(
            "part two: {}  (in {:.0?})",
            histogram.values().sum::<u64>(),
            start.elapsed()
        );
    }
}
