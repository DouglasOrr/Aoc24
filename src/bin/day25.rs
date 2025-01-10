use std::fs;

fn parse(path: &str) -> (Vec<u32>, Vec<u32>) {
    let data = fs::read_to_string(path).unwrap();
    let mut locks: Vec<u32> = Vec::new();
    let mut keys: Vec<u32> = Vec::new();
    for block in data.split("\n\n") {
        let is_lock = block.as_bytes()[0] == b'#';
        let code = block
            .replace("\n", "")
            .as_bytes()
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let col = i % 5;
                return ((*v == b'#') as u32) << (4 * col);
            })
            .sum::<u32>()
            - 0x11111;
        if is_lock {
            locks.push(code);
        } else {
            keys.push(code);
        }
    }
    return (locks, keys);
}

fn main() {
    let (locks, keys) = parse("inputs/day25/input.txt");

    // Part one
    let mut p1_count = 0;
    for lock in locks.iter() {
        for key in keys.iter() {
            let s = lock + key;
            let mut fit = true;
            for i in 0..5 {
                if (s >> (4 * i)) & 0xf > 5 {
                    fit = false;
                    break;
                }
            }
            p1_count += fit as u32;
        }
    }
    println!("part one: {p1_count}");

    // Part two was implicit - all done, then!
}
