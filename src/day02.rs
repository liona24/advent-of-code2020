use crate::util::read_lines;

pub fn solve() {
    println!("Day 2 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

struct Match {
    char_to_search : char,
    min_reps : usize,
    max_reps : usize,
    password : String
}

impl Match {
    fn parse(mut line : String) -> Self {
        // this is pretty ugly but we kinda follow the goal of std only

        let password = line.find(&": ").expect("Invalid input!");
        let password = line.split_off(password + 2);

        line.pop(); // pop ' '
        line.pop(); // pop ':'

        let char_to_search = line.pop().expect("Invalid input!");

        line.pop(); // pop ' '

        let sep = line.find('-').expect("Invalid input!");
        let max_reps = line.split_off(sep + 1).parse().expect("Invalid input!");

        line.pop(); // pop ':'
        let min_reps = line.parse().expect("Invalid input!");

        Self {
            char_to_search,
            min_reps,
            max_reps,
            password
        }
    }

    fn check(&self) -> bool {
        let count = self.password.chars().filter(|&c| c == self.char_to_search).count();
        count >= self.min_reps && count <= self.max_reps
    }

    fn check2(&self) -> bool {
        let i0 = self.min_reps - 1;
        let i1 = self.max_reps - 1;

        let bytes = self.password.as_bytes();

        let mut cvalue = [0; 1];
        self.char_to_search.encode_utf8(&mut cvalue);
        let cvalue = cvalue[0];

        let first_fits = bytes[i0] == cvalue;
        let secnd_fits = bytes[i1] == cvalue;

        first_fits != secnd_fits
    }
}

fn solve_first() {
    let input = read_lines("inputs/d02/input.txt").expect("Could not find input for day 2!");
    let mut count = 0;
    for line in input.filter_map(|line| line.ok().map(Match::parse)) {
        if line.check() {
            count += 1;
        }
    }
    println!("The solution is {}", count);
}

fn solve_second() {
    let input = read_lines("inputs/d02/input.txt").expect("Could not find input for day 2!");
    let mut count = 0;
    for line in input.filter_map(|line| line.ok().map(Match::parse)) {
        if line.check2() {
            count += 1;
        }
    }
    println!("The solution is {}", count);
}
