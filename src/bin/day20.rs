use std::fs;

fn parse(path: &str) -> (Vec<bool>, usize, usize, usize) {
    let data = fs::read_to_string(path).unwrap();
    let size = 1 + data
        .as_bytes()
        .iter()
        .map(|c| (*c == b'\n') as usize)
        .sum::<usize>();
    let data_flat = data.replace("\n", "");
    let walls: Vec<bool> = data_flat.as_bytes().iter().map(|c| *c == b'#').collect();
    let start = data_flat
        .as_bytes()
        .iter()
        .enumerate()
        .find(|(_, c)| **c == b'S')
        .unwrap()
        .0;
    let end = data_flat
        .as_bytes()
        .iter()
        .enumerate()
        .find(|(_, c)| **c == b'E')
        .unwrap()
        .0;
    return (walls, size, start, end);
}

fn main() {
    let (walls, size, start, end) = parse("inputs/day20/input.txt");

    // Find "visit time" for each point on the track
    let mut times: Vec<i32> = vec![-1; size * size];
    let mut pos = start;
    times[pos] = 0;
    'outer: while pos != end {
        for next in [pos - 1, pos + 1, pos - size, pos + size] {
            if !walls[next] && times[next] == -1 {
                times[next] = times[pos] + 1;
                pos = next;
                continue 'outer;
            }
        }
        panic!("No route from {pos}");
    }

    // Part one
    let mut p1_count = 0;
    for a in 0..size.pow(2) {
        if times[a] != -1 {
            for (dx, dy) in [(-2, 0), (2, 0), (0, -2), (0, 2)] {
                let bx = (a % size) as i32 + dx;
                let by = (a / size) as i32 + dy;
                if 0 <= bx && bx < (size as i32) && 0 <= by && by < (size as i32) {
                    let dt = times[by as usize * size + bx as usize] - times[a] - 2;
                    p1_count += (dt >= 100) as usize;
                }
            }
        }
    }
    println!("part one: {p1_count}");

    // Part two
    let mut p2_count = 0;
    for a in 0..size.pow(2) {
        if times[a] != -1 {
            for dx in -20_i32..=20 {
                for dy in -20_i32..=20 {
                    let cheat_t = dx.abs() + dy.abs();
                    let bx = (a % size) as i32 + dx;
                    let by = (a / size) as i32 + dy;
                    if cheat_t <= 20
                        && 0 <= bx
                        && bx < (size as i32)
                        && 0 <= by
                        && by < (size as i32)
                    {
                        let dt = times[by as usize * size + bx as usize] - times[a] - cheat_t;
                        p2_count += (dt >= 100) as usize;
                    }
                }
            }
        }
    }
    println!("part two: {p2_count}");
}
