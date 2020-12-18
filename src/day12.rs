use crate::util::read_lines;

#[derive(Debug)]
enum Instruction {
    North,
    South,
    West,
    East,
    RotLeft,
    RotRight,
    Forward,
}

#[derive(Debug)]
struct Position {
    x: i64,
    y: i64,
    angle: i64,
}

fn sign(v: i64) -> i64 {
    match v {
        0 => 0,
        x if x > 0 => 1,
        _ => -1,
    }
}

impl Instruction {
    fn from_char(c: char) -> Option<Self> {
        match c {
            'N' => Some(Self::North),
            'S' => Some(Self::South),
            'W' => Some(Self::West),
            'E' => Some(Self::East),
            'L' => Some(Self::RotLeft),
            'R' => Some(Self::RotRight),
            'F' => Some(Self::Forward),
            _ => None,
        }
    }

    fn execute(&self, arg: i64, pos: &mut Position) {
        match &self {
            Self::North => pos.y -= arg,
            Self::South => pos.y += arg,
            Self::West => pos.x -= arg,
            Self::East => pos.x += arg,
            Self::RotLeft => pos.angle = (360 + pos.angle + arg) % 360,
            Self::RotRight => pos.angle = (360 + pos.angle - arg) % 360,
            Self::Forward => match pos.angle {
                0 => pos.x += arg,
                90 => pos.y -= arg,
                180 => pos.x -= arg,
                270 => pos.y += arg,
                n => panic!("Apparently we have to do more mork :/ {}", n),
            },
        }
    }

    fn execute2(&self, arg: i64, ship: &mut Position, waypoint: &mut Position) {
        match &self {
            Self::North => waypoint.y -= arg,
            Self::South => waypoint.y += arg,
            Self::West => waypoint.x -= arg,
            Self::East => waypoint.x += arg,

            Self::RotLeft => {
                let x = waypoint.x;
                let y = waypoint.y;
                match arg % 360 {
                    90 | -90 => {
                        waypoint.x = y;
                        waypoint.y = -x * sign(arg);
                    }
                    180 | -180 => {
                        waypoint.x = -x;
                        waypoint.y = -y;
                    }
                    270 | -270 => {
                        waypoint.x = -y;
                        waypoint.y = x * sign(arg);
                    }
                    _ => panic!("Pls."),
                }
            }
            Self::RotRight => {
                let x = waypoint.x;
                let y = waypoint.y;
                match arg % 360 {
                    90 | -90 => {
                        waypoint.x = -y;
                        waypoint.y = x * sign(arg);
                    }
                    180 | -180 => {
                        waypoint.x = -x;
                        waypoint.y = -y;
                    }
                    270 | -270 => {
                        waypoint.x = y;
                        waypoint.y = -x * sign(arg);
                    }
                    _ => panic!("Pls."),
                }
            }
            Self::Forward => {
                ship.x += waypoint.x * arg;
                ship.y += waypoint.y * arg;
            }
        }
    }
}

fn parse(lines: impl Iterator<Item = String>) -> impl Iterator<Item = (Instruction, i64)> {
    lines.map(|line| {
        let t = line.chars().next().expect("Empty line?");
        let instr = Instruction::from_char(t).expect("Invalid instruction");
        let arg = line[1..].parse::<i64>().expect("Invalid argument");

        (instr, arg)
    })
}

pub fn solve() {
    println!("Day 12 - Part 1");
    solve_first();
    println!("       - Part 2");
    solve_second();
    println!();
}

fn solve_first() {
    let input = read_lines("inputs/d12/input.txt").expect("Could not find input for day 12!");
    let input = parse(input.filter_map(|line| line.ok()));

    let mut pos = Position {
        x: 0,
        y: 0,
        angle: 0,
    };
    for (instr, arg) in input {
        instr.execute(arg, &mut pos);
    }

    let dist = pos.x.abs() + pos.y.abs();
    println!("The solution is {}", dist);
}

fn solve_second() {
    let input = read_lines("inputs/d12/input.txt").expect("Could not find input for day 12!");
    let input = parse(input.filter_map(|line| line.ok()));

    let mut ship = Position {
        x: 0,
        y: 0,
        angle: 0,
    };
    let mut waypoint = Position {
        x: 10,
        y: -1,
        angle: 0,
    };
    for (instr, arg) in input {
        instr.execute2(arg, &mut ship, &mut waypoint);
    }

    let dist = ship.x.abs() + ship.y.abs();
    println!("The solution is {}", dist);
}
