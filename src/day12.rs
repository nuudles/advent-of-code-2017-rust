use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: String) {
    let mut pipes = HashSet::new();
    let mut programs = Vec::new();
    for line in input.lines() {
        let mut components = line.split(" <-> ");
        let left: u64 = components.next().and_then(|x| x.parse().ok()).unwrap_or_default();
        programs.push(left);
        for right in components.next().unwrap_or_default().split(", ").flat_map(|x| x.parse::<u64>().ok()) {
            pipes.insert((left, right));
            pipes.insert((right, left));
            programs.push(right);
        }
    }

    let mut groups = 0;
    while let Some(value) = programs.get(0) {
        let mut seen = HashSet::new();
        let mut to_check = vec![value];
        while !to_check.is_empty() {
            let mut next = vec![];
            for i in to_check.clone() {
                if seen.contains(&i) {
                    continue;
                }
                seen.insert(i);
                for connection in pipes.iter().filter(|t| t.0 == *i) {
                    next.push(&connection.1);
                }
            }
            to_check = next;
        }
        if seen.contains(&0) {
            println!("Part 1: {}", seen.len());
        }

        programs = programs.iter().filter(|x| !seen.contains(x)).map(|x| *x).collect_vec();
        groups += 1;
    }

    println!("Part 2: {}", groups);
}
