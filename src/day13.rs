use itertools::Itertools;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .map(|l| {
            let mut components = l.split(": ");
            (
                components.next().unwrap_or_default().parse::<u64>().unwrap_or_default(),
                components.next().unwrap_or_default().parse::<u64>().unwrap_or_default()
            )
        })
        .map(|(d, r)| {
            if d % ((r - 1) * 2) == 0 {
                d * r
            } else {
                0
            }
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    let layers = input
        .lines()
        .map(|l| {
            let mut components = l.split(": ");
            (
                components.next().unwrap_or_default().parse::<u64>().unwrap_or_default(),
                components.next().unwrap_or_default().parse::<u64>().unwrap_or_default()
            )
        })
        .collect_vec();
    let mut delay = 1;
    loop {
        if !layers.iter().any(|(d, r)| (d + delay) % ((r - 1) * 2) == 0) {
            break;
        }
        delay += 1;
    }
    println!("{}", delay);
}
