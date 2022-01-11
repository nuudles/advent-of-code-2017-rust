use std::collections::HashSet;

use itertools::Itertools;

pub fn part1(input: String) {
    let mut seen = HashSet::new();
    let mut banks = input.split("\t").flat_map(|n| n.parse::<usize>().ok()).collect_vec();
    let len = banks.len();
    seen.insert(banks.clone());
    let mut target: Option<Vec<usize>> = None;
    let mut cycles = 1;
    loop {
        let (index, count) = banks
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.cmp(b.1).then(b.0.cmp(&a.0)))
            .unwrap();
        let mut new_banks = banks.clone();
        new_banks[index] = 0;
        for i in 1..=*count {
            new_banks[(index + i) % len] += 1;
        }
        banks = new_banks;
        if target == None {
            if seen.contains(&banks) {
                target = Some(banks.clone());
                println!("Part 1: {}", seen.len());
            }
            seen.insert(banks.clone());
        } else if target == Some(banks.clone()) {
            break;
        } else {
            cycles += 1;
        }
    }
    println!("Part 2: {}", cycles);
}
