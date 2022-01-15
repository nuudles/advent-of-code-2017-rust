use std::collections::HashMap;

use itertools::Itertools;

use crate::selfprint::SelfPrint;

fn parse_or_get(string: &str, map: &HashMap<&str, i64>) -> i64 {
    if let Ok(value) = string.parse() {
        value
    } else {
        *map.get(string).unwrap_or(&0)
    }
}

struct Program<'a> {
    instructions: &'a Vec<&'a str>,
    registers: HashMap<&'a str, i64>,
    index: i64,
    mul_count: u64
}

impl Program<'_> {
    fn new<'a>(instructions: &'a Vec<&'a str>, a: i64) -> Program<'a> {
        let mut registers = HashMap::new();
        registers.insert("a", a);
        Program {
            instructions,
            registers,
            index: 0,
            mul_count: 0
        }
    }

    fn run(&mut self) {
        while let Ok(i) = usize::try_from(self.index) {
            if i >= self.instructions.len() {
                break;
            }
            let instruction = self.instructions[i];
            let mut components = instruction.split(" ");

            match components.next().unwrap_or_default() {
                "set" => {
                    let register = components.next().unwrap_or_default();
                    let value = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    self.registers.insert(register, value);
                },
                "sub" => {
                    let register = components.next().unwrap_or_default();
                    let value = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    self.registers.insert(register, self.registers.get(register).unwrap_or(&0) - value);
                },
                "mul" => {
                    let register = components.next().unwrap_or_default();
                    let value = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    self.registers.insert(register, self.registers.get(register).unwrap_or(&0) * value);
                    self.mul_count += 1;
                },
                "jnz" => {
                    let x = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    let y = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    if x != 0 {
                        // println!("{:?} Jumping {:?}", instruction, self.registers);
                        self.index += y - 1;
                    } else {
                        // println!("{:?} Not jumping {:?}", instruction, self.registers);
                    }
                },
                _ => ()
            }
            self.index += 1;
        }
    }
}

pub fn part1(input: String) {
    let instructions = input.lines().collect_vec();
    let mut program = Program::new(&instructions, 0);
    program.run();
    println!("{}", program.mul_count);
}

fn is_prime(n: u64) -> bool {
    for i in 2..n / 2 {
        if n % i == 0 {
            return false;
        }
    }
    true
}

pub fn part2(input: String) {
    /*
    Analyzed the program by hand and it seems to just count the
    number of non-prime numbers between b and c.
    */
    let mut b = input
        .lines()
        .next()
        .unwrap_or_default()
        .split(" ")
        .nth(2)
        .unwrap_or_default()
        .parse::<u64>()
        .unwrap_or_default();
    b = b * 100 + 100000;
    let c = b + 17000;
    (b..=c)
        .step_by(17)
        .filter(|n| !is_prime(*n))
        .count()
        .print();
}
