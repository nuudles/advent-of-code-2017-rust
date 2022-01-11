use std::collections::HashSet;

use crate::selfprint::SelfPrint;

pub fn part1(input: String) {
    input
        .lines()
        .filter(|l| {
            let mut words = HashSet::new();
            for word in l.split(" ") {
                if words.contains(word) {
                    return false;
                }
                words.insert(word);
            }
            true
        })
        .count()
        .print();
}

pub fn part2(input: String) {
    input
        .lines()
        .filter(|l| {
            let mut words = HashSet::<&str>::new();
            for word in l.split(" ") {
                let chars = word.chars().collect::<HashSet<_>>();
                if words.iter().any(|w| w.chars().collect::<HashSet<_>>() == chars) {
                    return false;
                }
                words.insert(word);
            }
            true
        })
        .count()
        .print();
}
