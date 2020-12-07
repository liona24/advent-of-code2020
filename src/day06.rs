use std::collections::HashSet;

use crate::util::read_lines;

pub fn solve() {
    println!("Day 6 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn solve_first() {
    let input = read_lines("inputs/d06/input.txt").expect("Could not find input for day 6!");

    let mut groups = vec![HashSet::new()];

    for line in input.filter_map(|line| line.ok()) {
        if line.is_empty() {
            groups.push(HashSet::new());
            continue;
        }
        for c in line.chars() {
            groups.last_mut().unwrap().insert(c);
        }
    }

    let ans = groups.iter().fold(0, |sum, set| sum + set.len());
    println!("The solution is {}", ans);
}

fn solve_second() {
    let input = read_lines("inputs/d06/input.txt").expect("Could not find input for day 6!");

    let mut groups = Vec::new();
    let mut current_group: Option<HashSet<_>> = None;

    for line in input.filter_map(|line| line.ok()) {
        if line.is_empty() {
            groups.push(current_group.unwrap());
            current_group = None;
            continue;
        }

        let chars: HashSet<_> = line.chars().collect();

        current_group = if let Some(set) = current_group {
            Some(set.intersection(&chars).cloned().collect())
        } else {
            Some(chars)
        }
    }

    if let Some(current_group) = current_group {
        groups.push(current_group);
    }

    let ans = groups.iter().fold(0, |sum, set| sum + set.len());
    println!("The solution is {}", ans);
}
