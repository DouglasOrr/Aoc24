use std::{collections::HashSet, fs};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Box,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell2 {
    Empty,
    BoxL,
    BoxR,
    Wall,
}

type Move = i32;

fn load(path: &str) -> (usize, usize, Vec<Cell>, Vec<Move>) {
    let data = fs::read_to_string(path).unwrap();
    let parts: Vec<&str> = data.split("\n\n").collect();

    let size = parts[0].split("\n").count();
    let flat_grid = parts[0].replace("\n", "");
    let flat_grid_u8 = flat_grid.as_bytes();
    let mut cells = Vec::new();
    let mut start: usize = 0;
    for i in 0..size.pow(2) {
        let c = flat_grid_u8[i];
        cells.push(if c == b'O' {
            Cell::Box
        } else if c == b'#' {
            Cell::Wall
        } else {
            Cell::Empty
        });
        if c == b'@' {
            start = i;
        }
    }

    let moves: Vec<Move> = parts[1]
        .replace("\n", "")
        .as_bytes()
        .iter()
        .map(|c| {
            [b'^', b'>', b'v', b'<']
                .iter()
                .position(|e| e == c)
                .unwrap() as Move
        })
        .collect();

    return (size, start, cells, moves);
}

fn sadd(base: usize, d: i32) -> usize {
    return (base as i32 + d) as usize;
}

#[allow(dead_code)]
fn show2(size: usize, position: usize, cells: &Vec<Cell2>) {
    let width = 2 * size;
    for y in 0..size {
        let i0 = y * width;
        println!(
            "{}",
            cells[i0..(i0 + width)]
                .iter()
                .enumerate()
                .map(|(i, c)| {
                    if i + i0 == position {
                        return '@';
                    }
                    return match c {
                        Cell2::Empty => '.',
                        Cell2::BoxL => '[',
                        Cell2::BoxR => ']',
                        Cell2::Wall => '#',
                    };
                })
                .collect::<String>()
        );
    }
}

fn main() {
    // Parse
    let (size, start, init, moves) = load("inputs/day15/input.txt");

    // Part one
    {
        let mut position = start;
        let mut state = init.clone();
        'outer: for dir in moves.iter() {
            let d = [-(size as i32), 1, size as i32, -1][*dir as usize];
            // Can we move? How far?
            let mut n = 0;
            'inner: loop {
                match state[sadd(position, d * (n + 1))] {
                    Cell::Empty => break 'inner,
                    Cell::Wall => continue 'outer,
                    Cell::Box => {
                        n += 1;
                    }
                }
            }
            // Execute the move
            while n > 0 {
                state[sadd(position, d * (n + 1))] = state[sadd(position, d * n)];
                n -= 1;
            }
            position = sadd(position, d);
            state[position] = Cell::Empty;
        }
        println!(
            "part one: {}",
            state
                .iter()
                .enumerate()
                .map(|(i, cell)| ((*cell == Cell::Box) as usize) * (100 * (i / size) + i % size))
                .sum::<usize>()
        );
    }

    // Part two
    {
        let width = 2 * size;
        let mut position = 2 * start;
        let mut state: Vec<Cell2> = init
            .iter()
            .flat_map(|c| match c {
                Cell::Empty => [Cell2::Empty, Cell2::Empty],
                Cell::Box => [Cell2::BoxL, Cell2::BoxR],
                Cell::Wall => [Cell2::Wall, Cell2::Wall],
            })
            .collect();
        'outer: for dir in moves.iter() {
            let is_vertical = *dir == 0 || *dir == 2;
            let d = [-(width as i32), 1, width as i32, -1][*dir as usize];

            // Can we move? What boxes are bumped?
            let mut bumps = Vec::new();
            let mut cols = HashSet::from([position]);
            while !cols.is_empty() {
                let mut next_cols = HashSet::new();
                for col in cols {
                    bumps.push(col);
                    let next_col = sadd(col, d);
                    match state[next_col] {
                        Cell2::Empty => {}
                        Cell2::Wall => continue 'outer,
                        Cell2::BoxL => {
                            next_cols.insert(next_col);
                            if is_vertical {
                                next_cols.insert(next_col + 1);
                            }
                        }
                        Cell2::BoxR => {
                            next_cols.insert(next_col);
                            if is_vertical {
                                next_cols.insert(next_col - 1);
                            }
                        }
                    }
                }
                cols = next_cols;
            }

            // Execute the move
            for x in bumps.iter().rev() {
                state[sadd(*x, d)] = state[*x];
                state[*x] = Cell2::Empty;
            }
            position = sadd(position, d);

            // println!("Dir: {dir}");
            // show2(size, position, &state);
        }
        println!(
            "part two: {}",
            state
                .iter()
                .enumerate()
                .map(|(i, cell)| ((*cell == Cell2::BoxL) as usize) * (100 * (i / width) + i % width))
                .sum::<usize>()
        );
    }
}
