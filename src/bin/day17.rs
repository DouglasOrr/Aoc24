use regex::Regex;
use std::fs;

struct Interpreter<'a> {
    program: &'a Vec<u8>,
    reg: [u64; 3],
    pc: usize,
    out: Vec<u8>,
}

fn combo(arg: u8, reg: &[u64; 3]) -> u64 {
    if arg < 4 {
        arg as u64
    } else if arg < 7 {
        reg[(arg - 4) as usize]
    } else {
        panic!("Bad operand {arg}");
    }
}

fn literal(arg: u8) -> u64 {
    arg as u64
}

fn reset(i: &mut Interpreter, reg: &[u64; 3]) {
    for j in 0..3 {
        i.reg[j] = reg[j];
    }
    i.pc = 0;
    i.out.clear();
}

fn div_pow(x: u64, p: u64) -> u64 {
    if p >= 64 {
        return 0;
    }
    return x / (1_u64 << p);
}

fn step(i: &mut Interpreter) {
    let arg = i.program[i.pc + 1];
    match i.program[i.pc] {
        0 => {
            // adv
            i.reg[0] = div_pow(i.reg[0], combo(arg, &i.reg));
            i.pc += 2;
        }
        1 => {
            // bxl
            i.reg[1] ^= literal(arg);
            i.pc += 2;
        }
        2 => {
            // bst
            i.reg[1] = combo(arg, &i.reg) % 8;
            i.pc += 2;
        }
        3 => {
            // jnz
            if i.reg[0] == 0 {
                i.pc += 2;
            } else {
                i.pc = literal(arg) as usize;
            }
        }
        4 => {
            // bxc
            i.reg[1] ^= i.reg[2];
            i.pc += 2;
        }
        5 => {
            // out
            i.out.push((combo(arg, &i.reg) % 8) as u8);
            i.pc += 2;
        }
        6 => {
            // bdv
            i.reg[1] = div_pow(i.reg[0], combo(arg, &i.reg));
            i.pc += 2;
        }
        7 => {
            // cdv
            i.reg[2] = div_pow(i.reg[0], combo(arg, &i.reg));
            i.pc += 2;
        }
        opcode => panic!("Bad instruction {opcode}"),
    }
}

fn main() {
    // Parse
    let raw_data = fs::read_to_string("inputs/day17/input.txt").unwrap();
    let pattern = Regex::new(
        r"Register A: (\d+)
Register B: (\d+)
Register C: (\d+)

Program: ([\d,]+)",
    )
    .unwrap();
    let m = pattern.captures(&raw_data).unwrap();
    let a_init: u64 = m.get(1).unwrap().as_str().parse().unwrap();
    let b_init: u64 = m.get(2).unwrap().as_str().parse().unwrap();
    let c_init: u64 = m.get(3).unwrap().as_str().parse().unwrap();
    let program: Vec<u8> = m
        .get(4)
        .unwrap()
        .as_str()
        .split(",")
        .map(|x| x.parse().unwrap())
        .collect();

    // Part one
    {
        let mut i = Interpreter {
            program: &program,
            reg: [a_init, b_init, c_init],
            pc: 0,
            out: Vec::new(),
        };
        while i.pc < i.program.len() {
            step(&mut i);
        }
        println!(
            "part one: {}",
            i.out
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join(",")
        );
    }

    // Part two
    {
        let mut i = Interpreter {
            program: &program,
            reg: [a_init, b_init, c_init],
            pc: 0,
            out: Vec::new(),
        };

        // let (a0, a_inc) = (0, 1);
        // Found by looking for matches of 8 digits manually
        // (could automate this, but seems fiddly)
        let (a0, a_inc) = (23948989, 2_u64.pow(26));

        let mut a_init_attempt: u64 = a0;
        'outer: loop {
            reset(&mut i, &[a_init_attempt, b_init, c_init]);
            while i.pc < i.program.len() && i.out.len() < i.program.len() {
                if !i.out.is_empty() && i.out[i.out.len() - 1] != i.program[i.out.len() - 1] {
                    // if i.out.len() > 8 {
                    //     println!(
                    //         "{a_init_attempt} {} | rem {}",
                    //         i.out.len() - 1,
                    //         a_init_attempt % (2_u64.pow(26))
                    //     );
                    // }
                    a_init_attempt += a_inc;
                    continue 'outer;
                }
                step(&mut i);
            }
            if i.out == *i.program {
                println!("\npart two: {a_init_attempt}");
                break;
            }
            a_init_attempt += a_inc;
        }
    }
}
