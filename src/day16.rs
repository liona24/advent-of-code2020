use crate::util::read_lines;

#[derive(Debug, Clone, PartialEq)]
struct Range {
    low: u32,
    high: u32,
}

impl Range {
    fn from_str(s: &str) -> Option<Self> {
        let mut bounds = s.split('-').map(|n| n.parse::<u32>().ok());
        let low = bounds.next()??;
        let high = bounds.next()??;

        Some(Self { low, high })
    }

    fn contains(&self, v: u32) -> bool {
        self.low <= v && self.high >= v
    }
}

#[derive(Debug)]
struct Input {
    my_ticket: Vec<u32>,
    nearby_tickets: Vec<Vec<u32>>,
    rules: Vec<[Range; 2]>,
}

pub fn solve() {
    println!("Day 16 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn parse_input(mut input: impl Iterator<Item = String>) -> Input {
    let mut rules = Vec::new();
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }

        let ranges = line.splitn(2, ": ").nth(1).expect("Invalid input");
        let mut ranges = ranges
            .splitn(2, " or ")
            .map(|r| Range::from_str(r).expect("Invalid input"));
        rules.push([
            ranges.next().expect("Invalid input"),
            ranges.next().expect("Invalid input"),
        ]);
    }

    let mut my_ticket = Vec::new();
    while let Some(line) = input.next() {
        if line.is_empty() {
            break;
        }
        if !line.starts_with("your") {
            my_ticket = line
                .split(',')
                .map(|n| n.parse::<u32>().expect("Invalid input"))
                .collect();
        }
    }

    let mut nearby_tickets: Vec<Vec<_>> = Vec::new();

    for line in input {
        if line.is_empty() || line.starts_with("nearby") {
            continue;
        }

        nearby_tickets.push(
            line.split(',')
                .map(|n| n.parse::<u32>().expect("Invalid input"))
                .collect(),
        );
    }

    Input {
        my_ticket,
        nearby_tickets,
        rules,
    }
}

fn solve_first() {
    let input = read_lines("inputs/d16/input.txt")
        .expect("Could not find input for day 16!")
        .filter_map(|line| line.ok());

    let input = parse_input(input);
    let mut valid = [false; 1000];
    for rule in input.rules.iter() {
        for range in rule {
            for i in range.low..=range.high {
                valid[i as usize] = true;
            }
        }
    }

    let ans: u32 = input
        .nearby_tickets
        .iter()
        .filter_map(|t| {
            let sum: u32 = t.iter().filter(|i| !valid[**i as usize]).sum();
            if sum == 0 {
                None
            } else {
                Some(sum)
            }
        })
        .sum();
    println!("The solution is {}", ans);
}

fn solve_second() {
    let input = read_lines("inputs/d16/input.txt")
        .expect("Could not find input for day 16!")
        .filter_map(|line| line.ok());

    let input = parse_input(input);
    let n_rules = input.rules.len();

    let mut valid = [false; 1000];
    for rule in input.rules.iter() {
        for range in rule {
            for i in range.low..=range.high {
                valid[i as usize] = true;
            }
        }
    }

    let Input {
        my_ticket,
        rules,
        nearby_tickets,
    } = input;

    let valid_tickets = nearby_tickets
        .into_iter()
        .filter(|t| t.iter().all(|i| valid[*i as usize]))
        .chain(std::iter::once(my_ticket.clone()));

    let mut candidates = vec![true; n_rules * n_rules];

    for t in valid_tickets {
        for (i, value) in t.iter().enumerate() {
            for (j, rule) in rules.iter().enumerate() {
                if !rule.iter().any(|rule| rule.contains(*value)) {
                    candidates[i * n_rules + j] = false;
                }
            }
        }
    }

    let mut mapping = Vec::new();
    while mapping.len() < n_rules {
        for in_i in 0..n_rules {
            let mut possible_rules = 0;
            let mut mapped_i = None;
            for out_i in 0..n_rules {
                if candidates[in_i * n_rules + out_i] {
                    possible_rules += 1;
                    mapped_i = Some(out_i);
                }
            }
            if possible_rules == 1 {
                let mapped_i = mapped_i.unwrap();
                mapping.push((in_i, mapped_i));
                for i in 0..n_rules {
                    candidates[i * n_rules + mapped_i] = false;
                }
            }
        }
    }

    let ans = mapping
        .into_iter()
        .filter(|m| m.1 < 6)
        .map(|m| my_ticket[m.0])
        .fold(1u64, |prod, x| (x as u64) * prod);

    println!("The solution is {}", ans);
}
