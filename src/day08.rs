use std::collections::HashMap;

use regex::Regex;
use lazy_static::lazy_static;

#[derive(Debug)]
struct Instruction<'a> {
    target: &'a str,
    instruction: &'a str,
    amount: i64,
    register: &'a str,
    comparison: &'a str,
    value: i64
}

impl Instruction<'_> {
    fn from(string: &str) -> Option<Instruction> {
        lazy_static! {
            static ref RE: Regex = 
                Regex::new(r"([a-z]+) (inc|dec) (-?\d+) if ([a-z]+) ([^a-z\s]+) (-?\d+)")
                    .expect("Invalid Regex");
        }

        let captures = RE.captures(string)?;
        Some(
            Instruction {
                target: captures.get(1)?.as_str(),
                instruction: captures.get(2)?.as_str(),
                amount: captures.get(3)?.as_str().parse().ok()?,
                register: captures.get(4)?.as_str(),
                comparison: captures.get(5)?.as_str(),
                value: captures.get(6)?.as_str().parse().ok()?,
            }
        )
    }
}

pub fn part1(input: String) {
    let mut registers: HashMap<&str, i64> = HashMap::new();
    let mut highest = 0;
    for instruction in input.lines().flat_map(Instruction::from) {
        let cmp = registers.get(instruction.register).unwrap_or(&0).cmp(&instruction.value);
        match (instruction.comparison, cmp) {
            (">", std::cmp::Ordering::Greater) |
            ("<", std::cmp::Ordering::Less) |
            ("==", std::cmp::Ordering::Equal) |
            ("!=", std::cmp::Ordering::Less) |
            ("!=", std::cmp::Ordering::Greater) |
            ("<=", std::cmp::Ordering::Less) |
            ("<=", std::cmp::Ordering::Equal) |
            (">=", std::cmp::Ordering::Greater) |
            (">=", std::cmp::Ordering::Equal) => {
                let value = registers.get(&instruction.target).unwrap_or(&0) + 
                    instruction.amount * 
                    if instruction.instruction == "inc" { 1 } else { -1 };
                if value > highest {
                    highest = value;
                }
                registers.insert(instruction.target, value);
            },
            _ => ()
        }
    }
    println!("Part 1: {}", registers.values().max().unwrap_or(&0));
    println!("Part 2: {}", highest);
}
