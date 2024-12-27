use std::fs;
use std::time::Instant;

fn can_produce(result: i64, operands: &Vec<i64>, concat: bool, index: usize) -> bool {
    let value = operands[index];
    if index == 0 {
        return result == value;
    }
    if result % value == 0 && can_produce(result / value, operands, concat, index - 1) {
        return true;
    }
    if result > value && can_produce(result - value, operands, concat, index - 1) {
        return true;
    }
    if concat {
        let mask = 10_i64.pow(value.ilog10() + 1);
        if result % mask == value && can_produce(result / mask, operands, concat, index - 1) {
            return true;
        }
    }
    return false;
}

fn main() {
    // Parse
    let data = fs::read_to_string("inputs/day07/input.txt").unwrap();
    let mut results: Vec<i64> = Vec::new();
    let mut operands: Vec<Vec<i64>> = Vec::new();
    for line in data.split("\n") {
        let mut i = line.split(": ");
        results.push(i.next().unwrap().parse().unwrap());
        operands.push(
            i.next()
                .unwrap()
                .trim_start()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect(),
        );
    }
    // Parts one and two
    for (part, concat) in [("one", false), ("two", true)] {
        let now = Instant::now();
        let mut result = 0;
        for i in 0..results.len() {
            if can_produce(results[i], &operands[i], concat, operands[i].len() - 1) {
                result += results[i];
            }
        }
        println!("part {part}: {result}   (in {:.0?})", now.elapsed());
    }
}
