use std::cmp::max;

use itertools::Itertools;

fn steps(pos: &(i64, i64)) -> i64 {
    match [pos.0.abs(), pos.1.abs()].iter().minmax() {
        itertools::MinMaxResult::MinMax(min, max) => min + (max - min),
        itertools::MinMaxResult::OneElement(element) => element * 2,
        itertools::MinMaxResult::NoElements => 0
    }
}

pub fn part1(input: String) {
    let mut pos = (0i64, 0i64);
    let mut max_distance = 0;

    for direction in input.split(",") {
        match direction {
            "n" => pos.1 -= 1,
            "s" => pos.1 += 1,
            "ne" => {
                pos.0 += 1;
                pos.1 -= 1;
            },
            "sw" => {
                pos.0 -= 1;
                pos.1 += 1;
            },
            "nw" => pos.0 -= 1,
            "se" => pos.0 += 1,
            _ => ()
        }
        max_distance = max(max_distance, steps(&pos));
    }
    println!("Part 1: {}", steps(&pos));
    println!("Part 2: {}", max_distance);
}
