use regex::Regex;
use std::fs;

#[derive(Debug)]
struct Machine {
    ax: i64,
    ay: i64,
    bx: i64,
    by: i64,
    px: i64,
    py: i64,
}

fn win_cost(machines: &Vec<Machine>) -> i64 {
    return machines
        .iter()
        .map(|m| {
            // Solve: M z = p  ;  z = M^-1 p  ;  M = [[ax bx]; [ay by]]
            let det = m.ax * m.by - m.ay * m.bx;
            assert_ne!(det, 0);
            let a_ = m.by * m.px - m.bx * m.py;
            let b_ = -m.ay * m.px + m.ax * m.py;
            if a_ % det == 0 && b_ % det == 0 && a_ / det >= 0 && b_ / det >= 0 {
                return 3 * (a_ / det) + (b_ / det);
            }
            return 0;
        })
        .sum::<i64>();
}

fn main() {
    let data = fs::read_to_string("inputs/day13/input.txt").unwrap();
    let pattern = Regex::new(
        r"Button A: X\+(\d+), Y\+(\d+)
Button B: X\+(\d+), Y\+(\d+)
Prize: X=(\d+), Y=(\d+)",
    )
    .unwrap();
    let mut machines: Vec<Machine> = pattern
        .captures_iter(data.as_str())
        .map(|m| Machine {
            ax: m.get(1).unwrap().as_str().parse().unwrap(),
            ay: m.get(2).unwrap().as_str().parse().unwrap(),
            bx: m.get(3).unwrap().as_str().parse().unwrap(),
            by: m.get(4).unwrap().as_str().parse().unwrap(),
            px: m.get(5).unwrap().as_str().parse().unwrap(),
            py: m.get(6).unwrap().as_str().parse().unwrap(),
        })
        .collect();
    println!("part one: {}", win_cost(&machines));
    for m in machines.iter_mut() {
        const P2_OFFSET: i64 = 10000000000000;
        m.px += P2_OFFSET;
        m.py += P2_OFFSET;
    }
    println!("part two: {}", win_cost(&machines));
}
