use rand::{Rng, SeedableRng};
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    fs,
    time::Instant,
};

// (OP, IN0, IN1, OUT)
type Gate<'a> = (&'a str, &'a str, &'a str, &'a str);

fn part_one(input_to_gate: &HashMap<&str, Vec<Gate>>, input_wires: &Vec<(&str, bool)>) {
    fn signal<'a>(
        input_to_gate: &HashMap<&'a str, Vec<Gate<'a>>>,
        wire_state: &mut HashMap<&'a str, bool>,
        wire: &'a str,
        state: bool,
    ) {
        wire_state.insert(wire, state);
        for g in input_to_gate.get(wire).unwrap_or(&Vec::new()) {
            if wire_state.contains_key(g.1) && wire_state.contains_key(g.2) {
                let a = *wire_state.get(g.1).unwrap();
                let b = *wire_state.get(g.2).unwrap();
                let o = match g.0 {
                    "XOR" => a ^ b,
                    "AND" => a && b,
                    "OR" => a || b,
                    _ => panic!("Bad gate {}", g.1),
                };
                signal(input_to_gate, wire_state, g.3, o);
            }
        }
    }

    let mut wire_state: HashMap<&str, bool> = HashMap::new();
    for (wire, state) in input_wires.iter() {
        signal(&input_to_gate, &mut wire_state, wire, *state);
    }

    let decode_pattern = Regex::new(r"([xyz])(\d+)").unwrap();
    let mut decoded: HashMap<&str, u64> = HashMap::from([("x", 0), ("y", 0), ("z", 0)]);
    for (wire, state) in wire_state.iter() {
        if *state {
            if let Some(m) = decode_pattern.captures(wire) {
                let o = m.get(1).unwrap().as_str();
                let d = m.get(2).unwrap().as_str().parse::<u64>().unwrap();
                *decoded.get_mut(o).unwrap() += 1 << d;
            }
        }
    }
    println!("part one: {}", decoded.get("z").unwrap());
}

fn part_two(input_to_gate: &HashMap<&str, Vec<Gate>>, output_to_gate: &HashMap<&str, Gate>) {
    // Input and output wire names
    let wire_x: Vec<String> = (0..45).map(|d| format!("x{d:02}")).collect();
    let wire_y: Vec<String> = (0..45).map(|d| format!("y{d:02}")).collect();
    let wire_z: Vec<String> = (0..46).map(|d| format!("z{d:02}")).collect();

    // Build 64 random test cases of 45-bit addition
    let mut rng = rand::rngs::StdRng::from_seed(*b"0123456789abcdef0123456789abcdef");
    let mut x = [0_u64; 45];
    let mut y = [0_u64; 45];
    let mut z = [0_u64; 46];
    for i in 0..64 {
        let dx: u64 = rng.gen::<u64>() % (1_u64 << 45);
        let dy: u64 = rng.gen::<u64>() % (1_u64 << 45);
        let dz = dx + dy;
        for d in 0..46 {
            if d < 45 {
                x[d] |= if dx & (1 << d) != 0 { 1 << i } else { 0 };
                y[d] |= if dy & (1 << d) != 0 { 1 << i } else { 0 };
            }
            z[d] |= if dz & (1 << d) != 0 { 1 << i } else { 0 };
        }
    }

    fn signal<'a>(
        input_to_gate: &HashMap<&'a str, Vec<Gate<'a>>>,
        wire_state: &mut HashMap<&'a str, u64>,
        wire: &'a str,
        state: u64,
    ) {
        wire_state.insert(wire, state);
        for g in input_to_gate.get(wire).unwrap_or(&Vec::new()) {
            if wire_state.contains_key(g.1) && wire_state.contains_key(g.2) {
                let a = *wire_state.get(g.1).unwrap();
                let b = *wire_state.get(g.2).unwrap();
                let o = match g.0 {
                    "XOR" => a ^ b,
                    "AND" => a & b,
                    "OR" => a | b,
                    _ => panic!("Bad gate {}", g.1),
                };
                signal(input_to_gate, wire_state, g.3, o);
            }
        }
    }

    // All inputs to a good wire are assumed to be good wires
    fn mark_good<'a>(
        output_to_gate: &HashMap<&'a str, Gate<'a>>,
        good_wires: &mut HashSet<&'a str>,
        wire: &'a str,
    ) {
        good_wires.insert(wire);
        if let Some(gate) = output_to_gate.get(wire) {
            mark_good(output_to_gate, good_wires, gate.1);
            mark_good(output_to_gate, good_wires, gate.2);
        }
    }

    let mut swapped: Vec<&str> = Vec::new();
    let mut wire_state: HashMap<&str, u64> = HashMap::new();
    let mut good_wires: HashSet<&str> = HashSet::new();
    for i in 0..46 {
        if i < 45 {
            good_wires.insert(&wire_x[i]);
            good_wires.insert(&wire_y[i]);
            signal(input_to_gate, &mut wire_state, &wire_x[i], x[i]);
            signal(input_to_gate, &mut wire_state, &wire_y[i], y[i]);
        }

        if *wire_state.get(&*wire_z[i]).unwrap() != z[i] {
            // Candidates to swap are wires that are already 'live' at this point, but
            // weren't marked 'good' from a previous output bit
            let live_wires: HashSet<&str> = wire_state.keys().map(|k| *k).collect();
            let candidates = live_wires.difference(&good_wires).collect::<Vec<&&str>>();
            // Consider all pairs of candidates
            for ab in 0..candidates.len().pow(2) {
                let (a, b) = (
                    candidates[ab % candidates.len()],
                    candidates[ab / candidates.len()],
                );
                if a != b {
                    let signal_a = *wire_state.get(a).unwrap();
                    let signal_b = *wire_state.get(b).unwrap();
                    signal(input_to_gate, &mut wire_state, a, signal_b);
                    signal(input_to_gate, &mut wire_state, b, signal_a);
                    if *wire_state.get(&*wire_z[i]).unwrap() == z[i] {
                        swapped.push(a);
                        swapped.push(b);
                        break;
                    } else {
                        // Undo the swap
                        signal(input_to_gate, &mut wire_state, a, signal_a);
                        signal(input_to_gate, &mut wire_state, b, signal_b);
                    }
                }
            }
            if *wire_state.get(&*wire_z[i]).unwrap() != z[i] {
                println!("Error - cannot match {}", wire_z[i]);
                break;
            }
        }
        mark_good(output_to_gate, &mut good_wires, &wire_z[i]);
    }
    swapped.sort();
    println!("part two: {}", swapped.join(","));
}

fn main() {
    // Parse
    let data = fs::read_to_string("inputs/day24/input.txt").unwrap();
    let wire_pattern = Regex::new(r"([\w\d]+): ([01])").unwrap();
    let input_wires: Vec<(&str, bool)> = wire_pattern
        .captures_iter(&data)
        .map(|c| {
            (
                c.get(1).unwrap().as_str(),
                c.get(2).unwrap().as_str() == "1",
            )
        })
        .collect();
    let gate_pattern = Regex::new(r"([\w\d]+) (XOR|AND|OR) ([\w\d]+) -> ([\w\d]+)").unwrap();
    let gates: Vec<Gate> = gate_pattern
        .captures_iter(&data)
        .map(|c| {
            (
                c.get(2).unwrap().as_str(),
                c.get(1).unwrap().as_str(),
                c.get(3).unwrap().as_str(),
                c.get(4).unwrap().as_str(),
            )
        })
        .collect();

    let start = Instant::now();

    // Prep
    let mut input_to_gate: HashMap<&str, Vec<Gate>> = HashMap::new();
    let mut output_to_gate: HashMap<&str, Gate> = HashMap::new();
    for gate in gates.iter() {
        input_to_gate
            .entry(gate.1)
            .or_insert(Vec::new())
            .push(*gate);
        input_to_gate
            .entry(gate.2)
            .or_insert(Vec::new())
            .push(*gate);
        output_to_gate.insert(gate.3, *gate);
    }

    part_one(&input_to_gate, &input_wires);
    part_two(&input_to_gate, &output_to_gate);
    println!("..in {:.1?}", start.elapsed());
}
