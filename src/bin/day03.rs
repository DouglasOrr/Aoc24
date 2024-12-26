use regex::Regex;
use std::fs;

fn main() {
    let program = fs::read_to_string("inputs/day03/input.txt").unwrap();

    // part one
    let result: i64 = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")
        .unwrap()
        .captures_iter(program.as_str())
        .map(|m| -> i64 {
            m.iter()
                .skip(1)
                .map(|x| {
                    x.unwrap()
                        .as_str()
                        .parse::<i64>()
                        .expect(format!("{}", x.unwrap().as_str()).as_str())
                })
                .product()
        })
        .sum();
    println!("part one: {result}");

    // part two
    let mut p2_result: i64 = 0;
    let mut enabled = true;
    for m in Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)")
        .unwrap()
        .captures_iter(program.as_str())
    {
        match m.get(0).unwrap().as_str() {
            "do()" => {
                enabled = true;
            }
            "don't()" => {
                enabled = false;
            }
            _ => {
                p2_result += enabled as i64
                    * m.iter()
                        .skip(1)
                        .map(|x| x.unwrap().as_str().parse::<i64>().unwrap())
                        .product::<i64>();
            }
        }
    }
    println!("part two: {p2_result}");
}
