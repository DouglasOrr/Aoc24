use std::fs;
use std::time::Instant;

fn main() {
    // Parse
    let text = fs::read_to_string("inputs/day06/input.txt").unwrap();
    let lines: Vec<&str> = text.split("\n").collect();
    let size = lines.len();
    assert_eq!(size, lines[0].len());
    let stride = size + 2;
    let mut walls = vec![false; stride * stride];
    let mut start = 0;
    for y in 0..size {
        for x in 0..size {
            let ch = lines[y].as_bytes()[x];
            walls[(y + 1) * stride + x + 1] = ch == b'#';
            if ch == b'^' {
                start = (y + 1) * stride + x + 1;
            }
        }
    }

    // Part one
    let mut position = start;
    let mut visited = vec![false; stride * stride];
    let mut direction = 0;
    let offsets = [-(stride as i32), 1, stride as i32, -1];
    loop {
        visited[position] = true;
        let next = ((position as i32) + offsets[direction]) as usize;
        let ny = next / stride;
        let nx = next % stride;
        if ny == 0 || ny == stride - 1 || nx == 0 || nx == stride - 1 {
            break;
        }
        if walls[next] {
            direction = (direction + 1) % 4;
        } else {
            position = next;
        }
    }
    println!(
        "part one: {}",
        visited.iter().map(|x| *x as i32).sum::<i32>()
    );

    // Part two
    let now = Instant::now();
    let mut p2_count = 0;
    for y in 0..size {
        for x in 0..size {
            let block_idx = (y + 1) * stride + x + 1;
            if !walls[block_idx] && block_idx != start {
                walls[block_idx] = true;
                let mut dir_visited = vec![false; 4 * stride * stride];
                let mut position = start;
                let mut direction = 0;
                loop {
                    if dir_visited[4 * position + direction] {
                        p2_count += 1;
                        break;
                    }
                    dir_visited[4 * position + direction] = true;
                    let next = ((position as i32) + offsets[direction]) as usize;
                    let ny = next / stride;
                    let nx = next % stride;
                    if ny == 0 || ny == stride - 1 || nx == 0 || nx == stride - 1 {
                        break;
                    }
                    if walls[next] {
                        direction = (direction + 1) % 4;
                    } else {
                        position = next;
                    }
                }
                walls[block_idx] = false;
            }
        }
    }
    println!("part two: {p2_count}   (in {:.0?})", now.elapsed());
}
