use crate::util::read_lines;

pub fn solve() {
    println!("Day 5 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn parse_input(seat_str: impl IntoIterator<Item = String>) -> impl Iterator<Item = (u32, u32)> {
    seat_str.into_iter().map(|line| {
        let row = &line[..7]
            .chars()
            .map(|c| match c {
                'F' => '0',
                'B' => '1',
                _ => panic!("Invalid input!"),
            })
            .collect::<String>();
        let col = &line[7..]
            .chars()
            .map(|c| match c {
                'L' => '0',
                'R' => '1',
                _ => panic!("Invalid input!"),
            })
            .collect::<String>();

        let row = u32::from_str_radix(&row, 2).expect("Should be valid binary number");
        let col = u32::from_str_radix(&col, 2).expect("Should be valid binary number");

        (row, col)
    })
}

fn solve_first() {
    let input = read_lines("inputs/d05/input.txt").expect("Could not find input for day 5!");

    let max_id = parse_input(input.filter_map(|line| line.ok())).fold(0, |mx, (row, col)| {
        let id = row * 8 + col;
        if id > mx {
            id
        } else {
            mx
        }
    });

    println!("The solution is {}", max_id);
}

fn solve_second() {
    let input = read_lines("inputs/d05/input.txt").expect("Could not find input for day 5!");

    let mut seats: [bool; 128 * 8] = [false; 128 * 8];

    for (row, col) in parse_input(input.filter_map(|line| line.ok())) {
        seats[(row * 8 + col) as usize] = true;
    }

    let mut my_seat = None;
    for i in 1..seats.len() - 1 {
        if seats[i - 1] && !seats[i] && seats[i + 1] {
            my_seat = Some(i);
            break;
        }
    }

    println!("The solution is {}", my_seat.expect("No free seat found!"));
}
