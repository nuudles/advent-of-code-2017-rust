use std::collections::HashSet;

use crate::{day10::dense_hash, selfprint::SelfPrint};

pub fn part1(input: String) {
    (0..128)
        .map(|row| {
            let mut string = input.clone();
            string.push_str(format!("-{}", row).as_str());
            dense_hash(string)
                .iter()
                .map(|n| {
                    let mut bit_count = 0;
                    let mut number = *n;
                    while number != 0 {
                        bit_count += number & 1;
                        number = number >> 1;
                    }
                    bit_count
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        .print();
}

pub fn part2(input: String) {
    let mut used = HashSet::<(usize, usize)>::new();
    for row in 0..128 {
        let mut string = input.clone();
        string.push_str(format!("-{}", row).as_str());
        for (i, n) in dense_hash(string).iter().enumerate() {
            let mut offset = 0;
            let mut number = *n;
            while number != 0 {
                if number & 1 == 1 {
                    used.insert((i * 8 + (7 - offset), row));
                }
                number = number >> 1;
                offset += 1;
            }
        }
    }

    let mut groups = 0;
    while let Some(&pos) = used.iter().next() {
        let mut seen = HashSet::new();
        let mut to_check = HashSet::new();
        to_check.insert(pos);
        while !to_check.is_empty() {
            let mut next = HashSet::new();
            for i in to_check.clone() {
                if seen.contains(&i) {
                    continue;
                }
                seen.insert(i);
                for neighbor in [
                    (i.0.saturating_sub(1), i.1),
                    (i.0 + 1, i.1),
                    (i.0, i.1.saturating_sub(1)),
                    (i.0, i.1 + 1)
                ] {
                    if used.contains(&neighbor) {
                        next.insert(neighbor);
                    }
                }
            }
            to_check = next;
        }

        used = used.iter().filter(|x| !seen.contains(x)).map(|x| *x).collect();
        groups += 1;
    }
    println!("{}", groups);
}
