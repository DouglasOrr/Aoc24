use std::{collections::HashMap, fs};

fn main() {
    let init: Vec<u32> = fs::read_to_string("inputs/day22/input.txt")
        .unwrap()
        .split("\n")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

    const MOD: u32 = 16777216;
    let mut p1_sum: u64 = 0;
    let mut code_to_profit: HashMap<u32, (usize, u32)> = HashMap::new();
    for (buyer, x) in init.iter().enumerate() {
        let mut i = *x;
        let mut code_id: u32 = 0;
        for n in 0..2000 {
            let prev_price = i % 10;
            i = (i ^ (i * 64)) % MOD;
            i = (i ^ (i / 32)) % MOD;
            i = (i ^ (i * 2048)) % MOD;
            let price = i % 10;
            let change_id = price + (9 - prev_price);
            code_id = ((code_id << 5) | change_id) & 0xfffff;
            // Don't confuse the first 3 steps with leading zero price changes
            if n >= 3 {
                match code_to_profit.get_mut(&code_id) {
                    Some(c) => {
                        if c.0 < buyer {
                            c.0 = buyer;
                            c.1 += price;
                        }
                    }
                    None => {
                        code_to_profit.insert(code_id, (buyer, price));
                    }
                }
            }
        }
        p1_sum += i as u64;
    }
    println!("part one: {p1_sum}");
    println!(
        "part two: {}",
        code_to_profit.values().map(|(_, p)| p).max().unwrap()
    );
}
