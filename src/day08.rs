use std::convert::{TryFrom, TryInto};

use crate::util::read_lines;

#[derive(Debug, Clone)]
enum Instruction {
    Nop { arg: i64 },
    Acc { inc: i64 },
    Jmp { offset: i64 },
}

impl Instruction {
    fn execute(&self, ip: usize, acc: i64) -> (usize, i64) {
        match self {
            Instruction::Nop { .. } => (ip + 1, acc),
            Instruction::Acc { inc } => (ip + 1, acc + inc),
            Instruction::Jmp { offset } => (((ip as i64) + offset) as usize, acc),
        }
    }
}

impl TryFrom<&str> for Instruction {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut it = value.split(' ');
        let instr = it.next().ok_or("value is empty?")?;
        let arg = it
            .next()
            .ok_or("argument is missing?")?
            .parse::<i64>()
            .or(Err("argument is no number?"))?;
        match instr {
            "nop" => Ok(Instruction::Nop { arg }),
            "acc" => Ok(Instruction::Acc { inc: arg }),
            "jmp" => Ok(Instruction::Jmp { offset: arg }),
            _ => Err("invalid instruction?"),
        }
    }
}

pub fn solve() {
    println!("Day 8 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn solve_first() {
    let input = read_lines("inputs/d08/input.txt").expect("Could not find input for day 8!");

    let instructions: Vec<Instruction> = input
        .filter_map(|line| line.ok())
        .map(|line| line[..].try_into().expect("Invalid input!"))
        .collect();
    let mut was_executed = vec![false; instructions.len()];

    let mut acc = 0i64;
    let mut ip = 0;
    while ip < was_executed.len() && !was_executed[ip] {
        was_executed[ip] = true;
        let (new_ip, new_acc) = instructions[ip].execute(ip, acc);
        ip = new_ip;
        acc = new_acc;
    }

    println!("The solution is {}", acc);
}

fn switch_instruction(instr: &Instruction) -> Instruction {
    match instr {
        Instruction::Nop { arg } => Instruction::Jmp { offset: *arg },
        Instruction::Jmp { offset } => Instruction::Nop { arg: *offset },
        _ => unreachable!(),
    }
}

fn solve_second() {
    let input = read_lines("inputs/d08/input.txt").expect("Could not find input for day 8!");

    let mut instructions: Vec<Instruction> = input
        .filter_map(|line| line.ok())
        .map(|line| line[..].try_into().expect("Invalid input!"))
        .collect();

    let mut i = 0;
    let mut acc = 0i64;

    while let Some(to_switch) = instructions.iter().skip(i).position(|instr| {
        std::mem::discriminant(instr) != std::mem::discriminant(&Instruction::Acc { inc: 0 })
    }) {
        i += to_switch;

        instructions[i] = switch_instruction(&instructions[i]);

        let mut was_executed = vec![false; instructions.len()];

        let mut finished = false;

        acc = 0;
        let mut ip = 0;
        while !was_executed[ip] {
            was_executed[ip] = true;
            let (new_ip, new_acc) = instructions[ip].execute(ip, acc);
            ip = new_ip;
            acc = new_acc;

            if ip >= was_executed.len() {
                finished = true;
                break;
            }
        }

        if finished {
            break;
        }

        // change back
        instructions[i] = switch_instruction(&instructions[i]);

        i += 1;
    }

    println!("The solution is {}", acc);
}
