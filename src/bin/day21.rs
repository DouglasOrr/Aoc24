use std::{collections::HashMap, fs, str};

fn shortest_path_length(
    pad0: &HashMap<u8, (i32, i32)>,
    pad: &HashMap<u8, (i32, i32)>,
    level: usize,
    seq: String,
    cache: &mut HashMap<(usize, String), usize>,
) -> usize {
    if level == 0 {
        return seq.len();
    }
    let cached_value = cache.get(&(level, seq.clone()));
    if cached_value.is_some() {
        return *cached_value.unwrap();
    }

    let mut pos = pad0.get(&b'A').unwrap();
    let mut length = 0;
    for ch in seq.as_bytes() {
        let next: &(i32, i32) = pad0.get(ch).unwrap();
        let dx = next.0 - pos.0;
        let dy = next.1 - pos.1;

        // There's never any need to interleave up-down and left-right
        let mut h_then_v = Vec::new();
        for _ in 0..dx.abs() {
            h_then_v.push(if dx < 0 { b'<' } else { b'>' });
        }
        for _ in 0..dy.abs() {
            h_then_v.push(if dy < 0 { b'^' } else { b'v' });
        }

        // Avoid the disallowed (0, 0), and only consider the v_then_h path
        // when moving both h and v.
        let mut min_length = usize::MAX;
        if dx != 0 && dy != 0 && !(pos.0 == 0 && next.1 == 0) {
            let mut v_then_h = h_then_v.clone();
            v_then_h.reverse();
            v_then_h.push(b'A');
            min_length = shortest_path_length(
                pad,
                pad,
                level - 1,
                String::from_utf8(v_then_h).unwrap(),
                cache,
            );
        }
        if !(pos.1 == 0 && next.0 == 0) {
            h_then_v.push(b'A');
            min_length = min_length.min(shortest_path_length(
                pad,
                pad,
                level - 1,
                String::from_utf8(h_then_v).unwrap(),
                cache,
            ));
        }
        length += min_length;
        pos = next;
    }
    cache.insert((level, seq), length);
    return length;
}

fn main() {
    let data = fs::read_to_string("inputs/day21/input.txt").unwrap();
    let codes: Vec<&str> = data.split("\n").collect();

    let pad0: HashMap<u8, (i32, i32)> = HashMap::from([
        (b'7', (0, -3)),
        (b'8', (1, -3)),
        (b'9', (2, -3)),
        (b'4', (0, -2)),
        (b'5', (1, -2)),
        (b'6', (2, -2)),
        (b'1', (0, -1)),
        (b'2', (1, -1)),
        (b'3', (2, -1)),
        (b'0', (1, 0)),
        (b'A', (2, 0)),
    ]);
    let pad: HashMap<u8, (i32, i32)> = HashMap::from([
        (b'^', (1, 0)),
        (b'A', (2, 0)),
        (b'<', (0, 1)),
        (b'v', (1, 1)),
        (b'>', (2, 1)),
    ]);
    let mut p1_code = 0;
    let mut p2_code = 0;
    let mut cache: HashMap<(usize, String), usize> = HashMap::new();
    for code in codes.iter() {
        let n: usize = code.strip_suffix("A").unwrap().parse().unwrap();
        p1_code += n * shortest_path_length(&pad0, &pad, 3, code.to_string(), &mut cache);
        p2_code += n * shortest_path_length(&pad0, &pad, 26, code.to_string(), &mut cache);
    }
    println!("part one: {p1_code}");
    println!("part two: {p2_code}");
}
