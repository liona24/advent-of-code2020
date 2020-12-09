use std::collections::VecDeque;

use crate::util::read_lines;

pub fn solve() {
    println!("Day 9 - Part 1");
    let n = solve_first().expect("Should have found a solution!");
    println!("      - Part 2");
    solve_second(n);
    println!();
}

fn solve_first() -> Option<usize> {
    let input = read_lines("inputs/d09/input.txt").expect("Could not find input for day 9!");

    let mut numbers = input
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<usize>().expect("Input should be number!"));

    let mut preamble: VecDeque<_> = numbers.by_ref().take(25).collect();

    for n in numbers {
        let mut found = false;
        for i in 0..preamble.len() - 1 {
            for j in i + 1..preamble.len() {
                if preamble[i] != preamble[j] && preamble[i] + preamble[j] == n {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }

        if !found {
            println!("The solution is {}", n);
            return Some(n);
        }

        preamble.pop_front();
        preamble.push_back(n);
    }

    None
}

fn solve_second(first_solution: usize) {
    let input = read_lines("inputs/d09/input.txt").expect("Could not find input for day 9!");

    let numbers = input
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<usize>().expect("Input should be number!"));

    let mut sum = 0;
    let mut set = VecDeque::new();

    for n in numbers {
        sum += n;
        set.push_back(n);

        while sum > first_solution {
            if let Some(i) = set.pop_front() {
                sum -= i;
            } else {
                break;
            }
        }

        if set.len() >= 2 && sum == first_solution {
            let ans = set.iter().min().unwrap() + set.iter().max().unwrap();
            println!("The solution is {}", ans);
            break;
        }
    }
}
