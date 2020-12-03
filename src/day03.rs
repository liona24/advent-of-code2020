use crate::util::read_lines;

#[derive(PartialEq, Debug)]
enum Tile {
    Empty,
    Tree,
}

struct Map {
    values: Vec<Tile>,
    width: usize,
    height: usize,
}

impl Tile {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '.' => Some(Self::Empty),
            '#' => Some(Self::Tree),
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
            values.push(Tile::from_char(c).expect("Invalid character!"));
        }

        for line in it {
            for c in line.chars() {
                values.push(Tile::from_char(c).expect("Invalid character!"));
            }
        }

        let height = values.len() / width;
        Self {
            values,
            width,
            height,
        }
    }
}

impl std::ops::Index<(usize, usize)> for Map {
    type Output = Tile;

    fn index(&self, xy : (usize, usize)) -> &Self::Output {
        &self.values[(xy.0 % self.width) + (xy.1 * self.width)]
    }
}

pub fn solve() {
    println!("Day 3 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn count_on_slope(map : &Map, dx : usize, dy: usize) -> usize {
    let mut tree_count = 0;
    let mut x = 0;
    for y in (0..map.height).step_by(dy) {
        if map[(x, y)] == Tile::Tree {
            tree_count += 1;
        }
        x += dx;
    }

    tree_count
}

fn solve_first() {
    let input = read_lines("inputs/d03/input.txt").expect("Could not find input for day 3!");
    let map = Map::new(input.filter_map(|line| line.ok()));

    let tree_count = count_on_slope(&map, 3, 1);

    println!("The solution is {}", tree_count);
}

fn solve_second() {
    let input = read_lines("inputs/d03/input.txt").expect("Could not find input for day 3!");
    let map = Map::new(input.filter_map(|line| line.ok()));

    let tree_counts = [
        count_on_slope(&map, 1, 1),
        count_on_slope(&map, 3, 1),
        count_on_slope(&map, 5, 1),
        count_on_slope(&map, 7, 1),
        count_on_slope(&map, 1, 2),
    ];

    let product = tree_counts.iter().fold(1, |prod, x| x * prod);

    println!("The solution is {}", product);
}
