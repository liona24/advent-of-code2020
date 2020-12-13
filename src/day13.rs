use crate::util::{read_lines, lcm};

pub fn solve() {
    println!("Day 13 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn solve_first() {
    let mut input = read_lines("inputs/d13/input.txt").expect("Could not find input for day 13!");
    let ready_for_departure = input
        .next()
        .expect("Should have line")
        .map(|s| s.parse::<usize>().expect("Could not parse departue time"))
        .expect("Invalid input");

    let busses: Vec<_> = input
        .next()
        .expect("Should have line")
        .map(|line| {
            line.split(',')
                .filter_map(|id| id.parse::<usize>().ok())
                .collect()
        })
        .expect("Invaild input");

    let ans = busses
        .into_iter()
        .map(|id| {
            if id == 0 {
                return (0, 0);
            }
            ((id - (ready_for_departure % id)) % id, id)
        })
        .min()
        .expect("No busses. No solution.");

    println!("The solution is {}", ans.0 * ans.1);
}

fn solve_second() {
    let mut input = read_lines("inputs/d13/input.txt").expect("Could not find input for day 13!");
    let _ = input.next();

    let mut busses: Vec<_> = input
        .next()
        .expect("Should have line")
        .map(|line| {
            line.split(',')
                .enumerate()
                .filter_map(|(i, id)| {
                    if let Ok(id) = id.parse::<usize>() {
                        Some((id, i))
                    } else {
                        None
                    }
                })
                .collect()
        })
        .expect("Invaild input");

    let (b0, _) = busses.remove(0);

    let mut step = 1;
    let mut i = 0;

    while let Some((bn, x)) = busses.pop() {
        loop {
            if (x + b0 * i) % bn == 0 {
                step = lcm(step, bn);
                break;
            }

            i += step;
        }
    }

    println!("The solution is {}", i * b0);
}
