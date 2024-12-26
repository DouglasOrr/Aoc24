use std::fs;

fn main() {
    let raw_data = fs::read_to_string("inputs/day04/input.txt").unwrap();
    let cols = raw_data.find('\n').unwrap() as i32;
    let rows = (raw_data.chars().filter(|c| *c == '\n').count() + 1) as i32;
    let flat_data = raw_data.replace("\n", "");
    let chars = flat_data.as_bytes();

    // Part one
    let pat = "XMAS";
    let pat_chars = pat.as_bytes();
    let pat_len = pat.len() as i32;

    let mut p1_count = 0;
    for x in 0..cols {
        for y in 0..rows {
            for dx in -1..=1 {
                for dy in -1..=1 {
                    let x_end = x + dx * (pat_len - 1);
                    let y_end = y + dy * (pat_len - 1);
                    if 0 <= x_end && x_end < cols && 0 <= y_end && y_end < rows {
                        let mut match_ = true;
                        for i in 0..pat_len {
                            if pat_chars[i as usize]
                                != chars[(cols * (y + i * dy) + (x + i * dx)) as usize]
                            {
                                match_ = false;
                            }
                        }
                        p1_count += match_ as i32;
                    }
                }
            }
        }
    }
    println!("part one: {p1_count}");

    // Part two
    let mut p2_count = 0;
    for x in 1..(cols - 1) {
        for y in 1..(rows - 1) {
            let idx = y * cols + x;
            if chars[idx as usize] == b'A' {
                let c00 = chars[(idx - 1 - cols) as usize];
                let c11 = chars[(idx + 1 + cols) as usize];
                let c01 = chars[(idx - 1 + cols) as usize];
                let c10 = chars[(idx + 1 - cols) as usize];
                p2_count += (((c00 == b'M' && c11 == b'S') || (c00 == b'S' && c11 == b'M'))
                    && ((c01 == b'M' && c10 == b'S') || (c01 == b'S' && c10 == b'M')))
                    as i32;
            }
        }
    }
    println!("part two: {p2_count}");
}
