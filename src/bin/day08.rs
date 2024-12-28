use std::{collections::HashMap, fs};

fn get_direction(x: i32, y: i32) -> (i32, i32) {
    // A proper GCD would be a better idea, but oh well!
    let mut xx = x;
    let mut yy = y;
    let mut i = 2;
    while i <= x / 2 && i <= y / 2 {
        if xx % i == 0 && yy % i == 0 {
            xx /= i;
            yy /= i;
        } else {
            i += 1;
        }
    }
    if xx != x || yy != y {
        // Ah, this bit was quite pointless!
        println!("dir {x},{y} -> {xx},{yy}");
    }
    return (xx, yy);
}

fn main() {
    // Parse
    let raw_data = fs::read_to_string("inputs/day08/input.txt").unwrap();
    let lines: Vec<&str> = raw_data.split("\n").collect();
    let size = lines.len();
    let mut antannae: HashMap<u8, Vec<(i32, i32)>> = HashMap::new();
    for y in 0..size {
        for x in 0..size {
            let ch = lines[y].as_bytes()[x];
            if ch != b'.' {
                antannae
                    .entry(ch)
                    .or_insert(vec![])
                    .push((x as i32, y as i32));
            }
        }
    }
    // Part one
    let mut p1_antinodes = vec![false; size * size];
    for loc in antannae.values() {
        for i in 0..loc.len() {
            for j in 0..loc.len() {
                if i != j {
                    let ax = 2 * loc[i].0 - loc[j].0;
                    let ay = 2 * loc[i].1 - loc[j].1;
                    if 0 <= ax && ax < size as i32 && 0 <= ay && ay < size as i32 {
                        p1_antinodes[(ay as usize) * size + (ax as usize)] = true;
                    }
                }
            }
        }
    }
    println!(
        "part one: {}",
        p1_antinodes.iter().map(|x| *x as i32).sum::<i32>()
    );

    // Part two
    let mut p2_antinodes = vec![false; size * size];
    for loc in antannae.values() {
        for i in 0..loc.len() {
            for j in 0..loc.len() {
                if i != j {
                    let (dx, dy) = get_direction(loc[i].0 - loc[j].0, loc[i].1 - loc[j].1);
                    let mut ax = loc[i].0;
                    let mut ay = loc[i].1;
                    while 0 <= ax && ax < size as i32 && 0 <= ay && ay < size as i32 {
                        p2_antinodes[(ay as usize) * size + (ax as usize)] = true;
                        ax += dx;
                        ay += dy;
                    }
                }
            }
        }
    }
    println!(
        "part two: {}",
        p2_antinodes.iter().map(|x| *x as i32).sum::<i32>()
    );
}
