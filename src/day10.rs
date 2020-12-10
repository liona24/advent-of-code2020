use crate::util::read_lines;

pub fn solve() {
    println!("Day 10 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn solve_first() {
    let input = read_lines("inputs/d10/input.txt").expect("Could not find input for day 10!");

    let mut numbers: Vec<_> = input
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<usize>().expect("Input should be number!"))
        .collect();

    numbers.sort_unstable();

    let mut diff_1 = 0;
    let mut diff_3 = 1; // diff to our adapter
    let mut prev = 0;
    for n in numbers {
        match n - prev {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => (),
        };

        prev = n;
    }

    println!("The solution is {}", diff_1 * diff_3);
}

fn count_sub(diffs: Vec<usize>) -> usize {
    let len = diffs.len();

    // The problem statement is not so clear about this
    // Apparently it is true but I would suggest that any element
    // could be equal to 2 aswell
    assert!(diffs.into_iter().all(|i| i == 1));

    // Other than that, this simple solution only works because of the
    // following assertion.
    // Poorly designed task I guess :(
    assert!(len <= 4);

    if len == 0 {
        return 1;
    }

    1 + (len * (len - 1)) / 2
}

fn solve_second() {
    let input = read_lines("inputs/d10/input.txt").expect("Could not find input for day 10!");

    let mut numbers: Vec<_> = input
        .filter_map(|line| line.ok())
        .map(|line| line.parse::<usize>().expect("Input should be number!"))
        .collect();

    numbers.sort_unstable();
    numbers.push(numbers.last().unwrap_or(&0) + 3);

    let mut subs = vec![Vec::new()];

    let mut prev = 0;
    for n in numbers {
        if n - prev >= 3 {
            subs.push(Vec::new());
        } else {
            subs.last_mut().unwrap().push(n - prev);
        }
        prev = n;
    }

    let solution = subs.into_iter().map(count_sub).fold(1, |prod, x| x * prod);

    println!("The solution is {}", solution);
}
