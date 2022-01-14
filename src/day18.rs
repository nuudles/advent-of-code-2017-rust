use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

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
    send_count: u64
}

impl Program<'_> {
    fn new<'a>(instructions: &'a Vec<&'a str>, id: i64) -> Program<'a> {
        let mut registers = HashMap::new();
        registers.insert("p", id);
        Program {
            instructions,
            registers,
            index: 0,
            send_count: 0
        }
    }

    fn run(&mut self, stack: &VecDeque<i64>) -> VecDeque<i64> {
        let mut send_stack = VecDeque::new();
        let mut receive_stack = stack.clone();

        while let Ok(i) = usize::try_from(self.index) {
            if i >= self.instructions.len() {
                break;
            }
            let instruction = self.instructions[i];
            let mut components = instruction.split(" ");

            match components.next().unwrap_or_default() {
                "snd" => {
                    let value = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    send_stack.push_back(value);
                    self.send_count += 1;
                },
                "set" => {
                    let register = components.next().unwrap_or_default();
                    let value = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    self.registers.insert(register, value);
                },
                "add" => {
                    let register = components.next().unwrap_or_default();
                    let value = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    self.registers.insert(register, value + self.registers.get(register).unwrap_or(&0));
                },
                "mul" => {
                    let register = components.next().unwrap_or_default();
                    let value = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    self.registers.insert(register, value * self.registers.get(register).unwrap_or(&0));
                },
                "mod" => {
                    let register = components.next().unwrap_or_default();
                    let value = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    self.registers.insert(register, self.registers.get(register).unwrap_or(&0) % value);
                },
                "rcv" => {
                    if let Some(received) = receive_stack.pop_front() {
                        self.registers.insert(components.next().unwrap_or_default(), received);
                    } else {
                        break;
                    }
                },
                "jgz" => {
                    let x = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    let y = parse_or_get(components.next().unwrap_or_default(), &self.registers);
                    if x > 0 {
                        self.index += y;
                        continue;
                    }
                },
                _ => ()
            }
            self.index += 1;
        }

        send_stack
    }
}

pub fn part1(input: String) {
    let instructions = input.lines().collect_vec();
    let mut program = Program::new(&instructions, 0);
    let mut stack = program.run(&VecDeque::new());
    println!("{}", stack.pop_back().unwrap_or_default());
}

pub fn part2(input: String) {
    let instructions = input.lines().collect_vec();
    let mut which = 0;
    let mut stack = VecDeque::new();
    let mut programs = [
        Program::new(&instructions, 0),
        Program::new(&instructions, 1)
    ];
    loop {
        stack = programs[which].run(&stack);
        which = 1 - which;
        if stack.len() == 0 {
            break;
        }
    }
    println!("{}", programs[1].send_count);
}
