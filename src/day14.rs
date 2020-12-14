use std::collections::HashMap;

use crate::util::read_lines;

pub fn solve() {
    println!("Day 14 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn update_mask(mask: &str, zeros: &mut u64, ones: &mut u64) {
    *zeros = !0;
    *ones = 0;

    for (i, c) in mask.chars().enumerate() {
        let i = mask.len() - i - 1;
        match c {
            '0' => *zeros &= !(1 << i),
            '1' => *ones |= 1 << i,
            _ => (),
        }
    }
}

fn update_mask2(s: &str, mask: &mut usize, perms: &mut Vec<usize>) {
    *mask = !0;

    let mut base = 0;
    perms.clear();
    let mut bits = Vec::new();

    for (i, c) in s.chars().enumerate() {
        let i = s.len() - i - 1;
        match c {
            '0' => (),
            '1' => base |= 1 << i,
            _ => {
                *mask &= !(1 << i);
                bits.push(i);
            }
        }
    }

    for i in 0..(1 << bits.len()) {
        let mut perm = base;
        for (pos_i, &pos_base) in bits.iter().enumerate() {
            perm |= ((i >> pos_i) & 1) << pos_base
        }
        perms.push(perm);
    }
}
fn parse_mem_access(s: &str) -> Option<(usize, u64)> {
    let mut it = s.splitn(2, '[').nth(1)?.splitn(2, ']');
    let addr = it.next()?.parse::<usize>().ok()?;
    let value = it.next()?.splitn(2, " = ").nth(1)?.parse::<u64>().ok()?;

    Some((addr, value))
}

fn solve_first() {
    let input = read_lines("inputs/d14/input.txt")
        .expect("Could not find input for day 14!")
        .filter_map(|line| line.ok());

    let mut zeros = !0;
    let mut ones = 0;

    let mut memory: HashMap<usize, u64> = HashMap::new();

    for line in input {
        if line.starts_with("mask") {
            update_mask(
                line.split(" = ").nth(1).expect("Invalid input"),
                &mut zeros,
                &mut ones,
            );
        } else if line.starts_with("mem") {
            let (addr, value) = parse_mem_access(&line).expect("Invalid input");
            let value = (value & zeros) | ones;
            memory.insert(addr, value);
        }
    }

    let ans: u64 = memory.values().sum();
    println!("The solution is {}", ans);
}

fn solve_second() {
    let input = read_lines("inputs/d14/input.txt")
        .expect("Could not find input for day 14!")
        .filter_map(|line| line.ok());

    let mut memory: HashMap<usize, u64> = HashMap::new();
    let mut mask = 0;
    let mut perms = Vec::new();

    for line in input {
        if line.starts_with("mask") {
            update_mask2(
                line.split(" = ").nth(1).expect("Invalid input"),
                &mut mask,
                &mut perms,
            );
        } else if line.starts_with("mem") {
            let (addr, value) = parse_mem_access(&line).expect("Invalid input");
            let addr = addr & mask;
            for p in perms.iter() {
                memory.insert(addr | p, value);
            }
        }
    }

    let ans: u64 = memory.values().sum();
    println!("The solution is {}", ans);
}
