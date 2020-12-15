use std::collections::HashMap;

pub fn solve() {
    const INPUT: [usize; 7] = [9, 19, 1, 6, 0, 5, 4];

    println!("Day 15 - Part 1");
    solve_chall(&INPUT, 2020);
    println!("      - Part 2");
    if cfg!(release) {
        solve_chall(&INPUT, 30000000);
    } else {
        println!("Skipped.");
    }
    println!();
}

fn solve_chall(input: &[usize], n: usize) {
    let mut age = 0;
    let mut map = HashMap::new();

    let mut input = input.iter();
    let mut last_spoken = *input.next().expect("at least one number required!");

    for i in input {
        map.insert(last_spoken, age);
        last_spoken = *i;
        age += 1;
    }

    while age < n - 1 {
        let next = if let Some(t_last) = map.get(&last_spoken) {
            age - t_last
        } else {
            0
        };
        map.insert(last_spoken, age);
        last_spoken = next;

        age += 1;
    }

    println!("The solution is {}", last_spoken);
}
