use regex::Regex;
use std::{fs, mem::swap};

fn main() {
    // let (name, width, height) = ("test", 11, 7);
    let (name, width, height) = ("input", 101, 103);
    let steps = 100;

    let data = fs::read_to_string(format!("inputs/day14/{name}.txt")).unwrap();
    let pattern = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    let pv: Vec<(i32, i32, i32, i32)> = data
        .split("\n")
        .map(|line| {
            let m = pattern.captures(line).unwrap();
            let px: i32 = m.get(1).unwrap().as_str().parse().unwrap();
            let py: i32 = m.get(2).unwrap().as_str().parse().unwrap();
            let vx: i32 = m.get(3).unwrap().as_str().parse().unwrap();
            let vy: i32 = m.get(4).unwrap().as_str().parse().unwrap();
            return (px, py, vx, vy);
        })
        .collect();

    // Part one
    let mut quad_count = [0, 0, 0, 0];
    for (px, py, vx, vy) in &pv {
        let dx = ((px + vx * steps) % width + width) % width;
        let dy = ((py + vy * steps) % height + height) % height;
        if dx != width / 2 && dy != height / 2 {
            let qx = dx / ((width + 1) / 2);
            let qy = dy / ((height + 1) / 2);
            quad_count[(2 * qy + qx) as usize] += 1;
        }
    }
    println!("part one: {}", quad_count.iter().product::<i32>());

    // Part two
    let mut min_score = i32::MAX;
    let mut min_score_step = 0;
    let mut min_score_grid = vec!['.'; (width * height) as usize];
    let mut wip_grid = vec!['.'; (width * height) as usize];
    for step in 0..100000 {
        wip_grid.fill('.');
        // Make a big triangle (turns out to be a wrong guess, but still gets us there)
        let mut score = 0;
        for (px, py, vx, vy) in &pv {
            let dx = ((px + vx * step) % width + width) % width;
            let dy = ((py + vy * step) % height + height) % height;
            wip_grid[(dy * width + dx) as usize] = '*';
            score += ((dx - width / 2).abs() - dy / 2).max(0);
        }
        if score < min_score {
            min_score = score;
            min_score_step = step;
            swap(&mut min_score_grid, &mut wip_grid);
        }
    }
    println!("part two: {min_score_step}");
    for y in 0..height {
        println!(
            "{}",
            (0..width)
                .map(|x| min_score_grid[(y * width + x) as usize])
                .collect::<String>()
        );
    }
}
