use std::collections::HashMap;

use crate::util::read_lines;

pub fn solve() {
    println!("Day 7 - Part 1");
    solve_first();
    println!("      - Part 2");
    solve_second();
    println!();
}


type Rules = HashMap<String, Vec<(usize, String)>>;


fn parse_rules(lines : impl Iterator<Item = String>) -> Option<Rules> {
    let mut rules = Rules::new();

    for line in lines {
        let mut it = line.split(" bags contain ");
        let name = it.next()?.to_string();
        let tail = it.next()?;

        let mut content = Vec::new();
        if !tail.ends_with("no other bags.") {
            for spec in tail.split(", ") {
                let mut it = spec.splitn(2, ' ');
                let count = it.next()?.parse::<usize>().expect("Number required!");

                let contained_bag = it.next()?.rsplitn(2, ' ').nth(1)?;
                content.push((count, contained_bag.to_string()));
            }
        }

        rules.insert(name, content);
    }

    Some(rules)
}

fn solve_first() {
    let input = read_lines("inputs/d07/input.txt").expect("Could not find input for day 7!");

    let rules = parse_rules(input.filter_map(|line| line.ok())).expect("Invalid format!");
    let mut tree : HashMap<String, Vec<String>> = HashMap::new();

    for (name, contained) in rules {
        if !tree.contains_key(&name) {
            tree.insert(name.clone(), Vec::new());
        }

        for (_count, contained_name) in contained {
            if let Some(refs) = tree.get_mut(&contained_name) {
                refs.push(name.clone());
            } else {
                tree.insert(contained_name, vec![name.clone()]);
            }
        }
    }

    let mut stack = Vec::new();

    let mut ans = 0;
    if let Some(root) = tree.remove("shiny gold") {
        stack.extend(root);

        while let Some(top) = stack.pop() {
            if let Some(refs) = tree.remove(&top) {
                ans += 1;
                stack.extend(refs);
            }
        }
    }

    println!("The solution is {}", ans);
}

fn count_bags(rules : &Rules, name: &str) -> usize {
    let refs = rules.get(name).expect("No spec for bag found!");
    let mut ans = 0;

    for (count, name) in refs {
        ans += count * (1 + count_bags(rules, name));
    }

    ans
}

fn solve_second() {
    let input = read_lines("inputs/d07/input.txt").expect("Could not find input for day 7!");

    let rules = parse_rules(input.filter_map(|line| line.ok())).expect("Invalid format!");

    let ans = count_bags(&rules, "shiny gold");

    println!("The solution is {}", ans);
}
