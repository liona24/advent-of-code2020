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

    let mut groups = vec![Vec::new()];

    for line in input.filter_map(|line| line.ok()) {
        if line.is_empty() {
            groups.push(Vec::new());
            continue;
        }

        let chars: HashSet<_> = line.chars().collect();
        groups.last_mut().unwrap().push(chars);
    }

    let mut sum = 0;
    for mut group in groups {
        if let Some(chars) = group.pop() {
            for c in chars {
                if group.iter().all(|set| set.contains(&c)) {
                    sum += 1;
                }
            }
        }
    }

    println!("The solution is {}", sum);
}
