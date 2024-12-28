use std::fs;
use std::time::Instant;

fn main() {
    // Parse
    let desc = fs::read_to_string("inputs/day09/input.txt").unwrap();
    let sizes: Vec<u8> = desc.bytes().map(|x| x - b'0').collect();

    // Part one
    {
        let start = Instant::now();
        let mut right: usize = sizes.len() - 1;
        let mut right_consumed = 0_usize;
        let mut pos = 0_usize;
        let mut checksum = 0_usize;
        for left in 0..sizes.len() {
            if left > right {
                break;
            } else if left % 2 == 0 {
                // Retain a chunk from left
                let n = sizes[left] as usize - (left == right) as usize * right_consumed;
                for _ in 0..n {
                    checksum += (left / 2) * pos;
                    pos += 1;
                }
            } else {
                // Fill empty space from right
                'outer: for _ in 0..sizes[left] {
                    while right_consumed >= sizes[right] as usize {
                        right -= 2;
                        right_consumed = 0;
                        if right < left {
                            break 'outer;
                        }
                    }
                    checksum += (right / 2) * pos;
                    pos += 1;
                    right_consumed += 1;
                }
            }
        }
        println!("part one: {checksum}   (in {:.0?})", start.elapsed());
    }

    // Part two
    {
        let start = Instant::now();
        let mut new_pos = vec![0; sizes.len()];
        let mut free_count = vec![0; sizes.len() / 2];
        let mut pos = 0_usize;
        for i in 0..sizes.len() {
            if i % 2 == 1 {
                free_count[i / 2] = sizes[i];
            }
            new_pos[i] = pos;
            pos += sizes[i] as usize;
        }
        let mut checksum = 0;
        'outer: for right in (0..sizes.len()).step_by(2).rev() {
            let rsize = sizes[right] as usize;
            for i in 0..right / 2 {
                if free_count[i] >= rsize as u8 {
                    checksum +=
                        (right / 2) * (new_pos[2 * i + 1] * rsize + (rsize * (rsize - 1)) / 2);
                    new_pos[2 * i + 1] += rsize;
                    free_count[i] -= rsize as u8;
                    continue 'outer;
                }
            }
            checksum += (right / 2) * (new_pos[right] * rsize + (rsize * (rsize - 1)) / 2);
        }
        println!("part two: {checksum}   (in {:.0?})", start.elapsed());
    }
}
