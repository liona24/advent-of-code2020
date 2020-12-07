use std::collections::HashMap;

use crate::util::read_lines;

fn has_digits_only_10(s: &str) -> bool {
    const DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    s.chars().all(|c| DIGITS.contains(&c))
}

fn has_digits_only_16(s: &str) -> bool {
    const DIGITS: [char; 16] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f'];
    s.chars().all(|c| DIGITS.contains(&c))
}

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn parse_many(lines: impl IntoIterator<Item = String>) -> Vec<Self> {
        let mut passports = Vec::new();
        let mut pp = Self {
            fields: HashMap::new(),
        };

        for line in lines.into_iter() {
            if line.is_empty() {
                passports.push(pp);
                pp = Self {
                    fields: HashMap::new(),
                };
                continue;
            }

            for pair in line.split(' ') {
                let mut key_value = pair.split(':');
                let key = key_value.next().expect("Invalid format");
                let value = key_value.next().expect("Invalid format");
                pp.fields.insert(key.into(), value.into());
            }
        }

        if !pp.fields.is_empty() {
            passports.push(pp);
        }

        passports
    }

    fn is_valid(&self) -> bool {
        const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

        REQUIRED_KEYS
            .iter()
            .all(|should| self.fields.keys().any(|k| k == should))
    }

    fn is_valid2(&self) -> bool {
        if !self.is_valid() {
            return false;
        }

        self.fields.keys().all(|k| {
            let v = &self.fields[k];
            match k.as_ref() {
                "byr" => v.parse::<u32>().map_or(false, |v| v <= 2002 && v >= 1920),
                "iyr" => v.parse::<u32>().map_or(false, |v| v <= 2020 && v >= 2010),
                "eyr" => v.parse::<u32>().map_or(false, |v| v <= 2030 && v >= 2020),
                "hgt" => {
                    if !v.ends_with("cm") && !v.ends_with("in") {
                        return false;
                    }

                    let min = if v.ends_with("cm") { 150 } else { 59 };
                    let max = if v.ends_with("cm") { 193 } else { 76 };

                    v[..v.len() - 2]
                        .parse::<u32>()
                        .map_or(false, |v| v <= max && v >= min)
                },
                "hcl" => v.starts_with('#') && v.len() == 7 && has_digits_only_16(&v[1..]),
                "ecl" => {
                    const COLORS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
                    COLORS.contains(&v.as_ref())
                },
                "pid" => v.len() == 9 && has_digits_only_10(&v),
                _ => true,
            }
        })
    }
}

pub fn solve() {
    println!("Day 4 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}

fn solve_first() {
    let input = read_lines("inputs/d04/input.txt").expect("Could not find input for day 4!");
    let passports = Passport::parse_many(input.filter_map(|line| line.ok()));

    let ans = passports.iter().fold(0, |sum, passport| {
        sum + if passport.is_valid() { 1 } else { 0 }
    });
    println!("The solution is {}", ans);
}

fn solve_second() {
    let input = read_lines("inputs/d04/input.txt").expect("Could not find input for day 4!");
    let passports = Passport::parse_many(input.filter_map(|line| line.ok()));

    let ans = passports.iter().fold(0, |sum, passport| {
        sum + if passport.is_valid2() { 1 } else { 0 }
    });
    println!("The solution is {}", ans);
}
