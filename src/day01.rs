use crate::util::read_lines;

use std::collections::HashSet;

pub fn solve() {
    println!("Day 1 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn solve_first() {
    let input = read_lines("inputs/d01/input.txt").expect("Could not find input for day 1!");
    let numbers: HashSet<_> = input
        .filter_map(|line| {
            line.ok()
                .map(|line| line.parse::<u32>().expect("Numerical input required!"))
        })
        .collect();

    for &n in numbers.iter() {
        let delta = 2020 - n;
        if numbers.contains(&delta) {
            let solution = n * delta;
            println!("The solution is {}", solution);
            break;
        }
    }
}

fn solve_second() {
    let input = read_lines("inputs/d01/input.txt").expect("Could not find input for day 1!");
    let numbers: HashSet<_> = input
        .filter_map(|line| {
            line.ok()
                .map(|line| line.parse::<u32>().expect("Numerical input required!"))
        })
        .collect();

    // is there a better solution than O(n^2)?

    for &i in numbers.iter() {
        let delta1 = 2020 - i;
        for &j in numbers.iter() {
            if j == i || j > delta1 {
                continue;
            }

            let delta2 = delta1 - j;
            if numbers.contains(&delta2) {
                let solution = i * j * delta2;
                println!("The solution is {}", solution);
                return;
            }
        }
    }
}
