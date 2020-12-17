use std::collections::HashSet;

use crate::util::read_lines;

#[derive(PartialEq, Debug, Clone, Copy)]
enum Cell {
    Active,
    Inactive,
}

fn parse_slice3(lines: impl Iterator<Item = String>) -> HashSet<(i64, i64, i64)> {
    let mut set = HashSet::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                set.insert((x as i64, y as i64, 0));
            }
        }
    }

    set
}
fn parse_slice4(lines: impl Iterator<Item = String>) -> HashSet<(i64, i64, i64, i64)> {
    let mut set = HashSet::new();

    for (y, line) in lines.enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                set.insert((x as i64, y as i64, 0, 0));
            }
        }
    }

    set
}

fn loop3d(
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
    z0: i64,
    z1: i64,
    mut body: impl FnMut(i64, i64, i64),
) {
    for x in x0..=x1 {
        for y in y0..=y1 {
            for z in z0..=z1 {
                body(x, y, z);
            }
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn loop4d(
    x0: i64,
    x1: i64,
    y0: i64,
    y1: i64,
    z0: i64,
    z1: i64,
    w0: i64,
    w1: i64,
    mut body: impl FnMut(i64, i64, i64, i64),
) {
    for x in x0..=x1 {
        for y in y0..=y1 {
            for z in z0..=z1 {
                for w in w0..=w1 {
                    body(x, y, z, w);
                }
            }
        }
    }
}

pub fn solve() {
    if cfg!(release) {
        println!("Day 17 - Part 1");
        solve_first();
        println!("      - Part 2");
        solve_second();
    } else {
        println!("Day 17 - Skipped.")
    }
    println!();
}

fn solve_first() {
    let input = read_lines("inputs/d17/input.txt").expect("Could not find input for day 17!");
    let mut current = parse_slice3(input.filter_map(|line| line.ok()));
    let mut next = HashSet::new();
    let mut stack = Vec::new();
    for _ in 0..6 {
        for pos in current.iter() {
            stack.push((Cell::Active, *pos));

            while let Some((v, pos)) = stack.pop() {
                let mut active_neighbors = 0;
                loop3d(
                    pos.0 - 1,
                    pos.0 + 1,
                    pos.1 - 1,
                    pos.1 + 1,
                    pos.2 - 1,
                    pos.2 + 1,
                    |x, y, z| {
                        if x == pos.0 && y == pos.1 && z == pos.2 {
                            return;
                        }
                        if current.contains(&(x, y, z)) {
                            active_neighbors += 1;
                        } else if v == Cell::Active {
                            stack.push((Cell::Inactive, (x, y, z)));
                        }
                    },
                );

                match (v, active_neighbors) {
                    (Cell::Active, 2) | (Cell::Active, 3) | (Cell::Inactive, 3) => next.insert(pos),
                    _ => false,
                };
            }
        }

        std::mem::swap(&mut current, &mut next);
        next.clear();
    }

    let ans = current.len();
    println!("The solution is {}", ans);
}

fn solve_second() {
    let input = read_lines("inputs/d17/input.txt").expect("Could not find input for day 17!");
    let mut current = parse_slice4(input.filter_map(|line| line.ok()));
    let mut next = HashSet::new();
    let mut stack = Vec::new();
    for _ in 0..6 {
        for pos in current.iter() {
            stack.push((Cell::Active, *pos));

            while let Some((v, pos)) = stack.pop() {
                let mut active_neighbors = 0;
                loop4d(
                    pos.0 - 1,
                    pos.0 + 1,
                    pos.1 - 1,
                    pos.1 + 1,
                    pos.2 - 1,
                    pos.2 + 1,
                    pos.3 - 1,
                    pos.3 + 1,
                    |x, y, z, w| {
                        if x == pos.0 && y == pos.1 && z == pos.2 && w == pos.3 {
                            return;
                        }
                        if current.contains(&(x, y, z, w)) {
                            active_neighbors += 1;
                        } else if v == Cell::Active {
                            stack.push((Cell::Inactive, (x, y, z, w)));
                        }
                    },
                );

                match (v, active_neighbors) {
                    (Cell::Active, 2) | (Cell::Active, 3) | (Cell::Inactive, 3) => next.insert(pos),
                    _ => false,
                };
            }
        }

        std::mem::swap(&mut current, &mut next);
        next.clear();
    }

    let ans = current.len();
    println!("The solution is {}", ans);
}
