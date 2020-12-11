use crate::util::read_lines;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Cell {
    Empty,
    Floor,
    Dude,
}

#[derive(Debug, Clone, PartialEq)]
struct Map {
    values: Vec<Cell>,
    width: usize,
    height: usize,
}

impl Cell {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Floor),
            '#' => Some(Self::Dude),
            'L' => Some(Self::Empty),
            _ => None,
        }
    }
}

impl Map {
    fn new(repr: impl IntoIterator<Item = String>) -> Self {
        let mut values = Vec::new();
        let mut it = repr.into_iter();
        let first_line = it.next().expect("At least one line required!");
        let width = first_line.len();

        for c in first_line.chars() {
            values.push(Cell::from_char(c).expect("Invalid character!"));
        }

        for line in it {
            for c in line.chars() {
                values.push(Cell::from_char(c).expect("Invalid character!"));
            }
        }

        let height = values.len() / width;
        Self {
            values,
            width,
            height,
        }
    }

    fn at(&self, x: usize, y: usize) -> Cell {
        if x == 0 || y == 0 || x > self.width || y > self.height {
            Cell::Empty
        } else {
            self.values[(y - 1) * self.width + x - 1]
        }
    }
    fn set(&mut self, x: usize, y: usize, c: Cell) {
        self.values[(y - 1) * self.width + x - 1] = c;
    }
}

fn simulate(from: &Map, into: &mut Map) {
    for x in 1..=from.width {
        for y in 1..=from.height {
            let mut n_dudes = 0;
            for xn in x - 1..=x + 1 {
                for yn in y - 1..=y + 1 {
                    if xn == x && yn == y {
                        continue;
                    }
                    if from.at(xn, yn) == Cell::Dude {
                        n_dudes += 1;
                    }
                }
            }

            into.set(
                x,
                y,
                match from.at(x, y) {
                    Cell::Floor => Cell::Floor,
                    Cell::Dude => {
                        if n_dudes >= 4 {
                            Cell::Empty
                        } else {
                            Cell::Dude
                        }
                    }
                    Cell::Empty => {
                        if n_dudes == 0 {
                            Cell::Dude
                        } else {
                            Cell::Empty
                        }
                    }
                },
            )
        }
    }
}

fn simulate2(from: &Map, into: &mut Map) {
    for x in 1..=from.width {
        for y in 1..=from.height {
            let mut n_dudes = 0;
            let directions = [
                (-1, 0),
                (1, 0),
                (0, -1),
                (0, 1),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ];
            for d in directions.iter() {
                let mut xn = x as i64 + d.0;
                let mut yn = y as i64 + d.1;
                while xn >= 0 && yn >= 0 && xn <= from.width as i64 && yn <= from.height as i64 {
                    match from.at(xn as usize, yn as usize) {
                        Cell::Dude => {
                            n_dudes += 1;
                            break;
                        }
                        Cell::Empty => break,
                        Cell::Floor => (),
                    };

                    xn += d.0;
                    yn += d.1;
                }
            }

            into.set(
                x,
                y,
                match from.at(x, y) {
                    Cell::Floor => Cell::Floor,
                    Cell::Dude => {
                        if n_dudes >= 5 {
                            Cell::Empty
                        } else {
                            Cell::Dude
                        }
                    }
                    Cell::Empty => {
                        if n_dudes == 0 {
                            Cell::Dude
                        } else {
                            Cell::Empty
                        }
                    }
                },
            )
        }
    }
}

pub fn solve() {
    println!("Day 11 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn solve_first() {
    let input = read_lines("inputs/d11/input.txt").expect("Could not find input for day 11!");

    let mut gen = Map::new(input.filter_map(|line| line.ok()));
    let mut next_gen = gen.clone();
    simulate(&gen, &mut next_gen);

    while next_gen != gen {
        std::mem::swap(&mut gen, &mut next_gen);
        simulate(&gen, &mut next_gen);
    }

    let ans = gen
        .values
        .into_iter()
        .filter(|cell| cell == &Cell::Dude)
        .count();
    println!("The solution is {}", ans);
}

fn solve_second() {
    let input = read_lines("inputs/d11/input.txt").expect("Could not find input for day 11!");

    let mut gen = Map::new(input.filter_map(|line| line.ok()));
    let mut next_gen = gen.clone();
    simulate2(&gen, &mut next_gen);

    while next_gen != gen {
        std::mem::swap(&mut gen, &mut next_gen);
        simulate2(&gen, &mut next_gen);
    }

    let ans = gen
        .values
        .into_iter()
        .filter(|cell| cell == &Cell::Dude)
        .count();
    println!("The solution is {}", ans);
}
