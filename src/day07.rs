use std::collections::{HashSet, HashMap};

use counter::Counter;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

use crate::selfprint::SelfPrint;

#[derive(Debug)]
struct Program<'a> {
    name: &'a str,
    weight: u64,
    children: Vec<&'a str>
}

impl Program<'_> {
    fn from(string: &str) -> Option<Program> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]+) \((\d+)\)(?: -> (.*))?").unwrap();
        }
        let captures = RE.captures(string)?;
        let name = captures.get(1)?.as_str();
        let weight = captures.get(2)?.as_str().parse().ok()?;
        Some(
            Program {
                name,
                weight,
                children: captures
                    .get(3)
                    .map_or(
                        vec![], 
                        |c| c.as_str()
                            .split(", ")
                            .collect_vec()
                    )
            }
        )
    }

    fn total_weight(&self, map: &HashMap<&str, Program>) -> u64 {
        self.weight + self.children.iter().map(|c| map.get(c).map_or(0, |p| p.total_weight(map))).sum::<u64>()
    }
}

pub fn part1(input: String) {
    let programs: HashMap<_, _> = input.lines().flat_map(Program::from).map(|p| (p.name, p)).collect();
    let mut names = programs.keys().collect::<HashSet<_>>();
    for child in programs.values().flat_map(|p| p.children.clone()) {
        names.remove(&child);
    }
    let root = names.iter().nth(0).unwrap_or(&&"");
    println!("Part 1: {}", root);

    // For Part 2, I used this to calculate all the unbalanced nodes and eyeballed it
    // to determine which one was the deepest (e.g. had the lowest weight), then manually 
    // calculated the weight needed to balance it. Probably could write an algorithm to do
    // this, but it worked, so moved on.
    for program in programs.values().filter(|p| !p.children.is_empty()) {
        let weights = program
            .children
            .iter()
            .flat_map(|c| programs.get(c).map(|p| (p, p.total_weight(&programs))))
            .collect_vec();
        let counts = weights.iter().map(|(_, w)| w).collect::<Counter<_>>();
        if counts.len() > 1 {
            counts.print();
            weights.print();
        }
    }
}
