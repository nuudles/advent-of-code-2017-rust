use itertools::{Itertools, MinMaxResult};

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .map(|l| {
            match l.split("\t").flat_map(|n| n.parse::<u64>().ok()).minmax() {
                MinMaxResult::NoElements => 0,
                MinMaxResult::OneElement(_) => 0,
                MinMaxResult::MinMax(min, max) => max - min,
            }
        })
        .sum::<u64>()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .map(|l| {
            let numbers = l.split("\t").flat_map(|n| n.parse::<u64>().ok());
            for (a, b) in numbers.tuple_combinations() {
                if b % a == 0 {
                    return b / a;
                } else if a % b == 0 {
                    return a / b;
                }
            }
            0
        })
        .sum::<u64>()
        .print();
}
