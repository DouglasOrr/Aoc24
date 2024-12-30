use std::fs;
use std::time::Instant;

fn find_root(parent: &Vec<usize>, node: usize) -> usize {
    let mut i = node;
    while parent[i] != i {
        i = parent[i];
    }
    return i;
}

fn main() {
    let data = fs::read_to_string("inputs/day12/input.txt").unwrap();
    let size = data.split("\n").count();
    let labels_data = data.replace("\n", "");
    let labels = labels_data.as_bytes();

    // Part one & two
    let start = Instant::now();
    let mut perimiter: Vec<i32> = (0..size.pow(2))
        .map(|i| {
            return (i % size == 0 || labels[i - 1] != labels[i]) as i32
                + (i % size == (size - 1) || labels[i + 1] != labels[i]) as i32
                + (i / size == 0 || labels[i - size] != labels[i]) as i32
                + (i / size == (size - 1) || labels[i + size] != labels[i]) as i32;
        })
        .collect();
    let mut corners: Vec<i32> = (0..size.pow(2))
        .map(|i| {
            let mut n = 0;
            for (dx, dy) in [
                (-1, -(size as i32)),
                (1, -(size as i32)),
                (-1, size as i32),
                (1, size as i32),
            ] {
                let bx = (i % size) == (if dx < 0 { 0 } else { size - 1 });
                let by = (i / size) == (if dy < 0 { 0 } else { size - 1 });

                let _11 = bx || by || labels[(i as i32 + dx + dy) as usize] != labels[i];
                let _10 = bx || labels[(i as i32 + dx) as usize] != labels[i];
                let _01 = by || labels[(i as i32 + dy) as usize] != labels[i];
                n += ((_10 && _01) || (_11 && !_10 && !_01)) as i32;
            }
            return n;
        })
        .collect();
    let mut area: Vec<i32> = (0..size.pow(2)).map(|_| 1).collect();

    // - union-find (I think)
    let mut parent: Vec<usize> = (0..size.pow(2)).collect();
    for i in 0..size.pow(2) {
        if i % size != 0 && labels[i - 1] == labels[i] {
            assert_eq!(parent[i], i, "shouldn't be grouped");
            parent[i] = parent[i - 1];
            let root = find_root(&parent, i);
            perimiter[root] += perimiter[i];
            perimiter[i] = 0;
            area[root] += area[i];
            area[i] = 0;
            corners[root] += corners[i];
            corners[i] = 0;
        }
        if i / size != 0 && labels[i - size] == labels[i] {
            let old_root = find_root(&parent, i);
            let new_root = find_root(&parent, i - size);
            if old_root != new_root {
                parent[old_root] = new_root;
                perimiter[new_root] += perimiter[old_root];
                perimiter[old_root] = 0;
                area[new_root] += area[old_root];
                area[old_root] = 0;
                corners[new_root] += corners[old_root];
                corners[old_root] = 0;
            }
        }
    }

    let p1_cost: i32 = perimiter.iter().zip(area.iter()).map(|(p, a)| p * a).sum();
    println!("part one: {p1_cost}");
    let p2_cost: i32 = corners.iter().zip(area.iter()).map(|(p, a)| p * a).sum();
    println!("part two: {p2_cost}  (in {:.1?})", start.elapsed());
}
