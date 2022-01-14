use std::collections::{HashSet, HashMap};

use itertools::{iproduct, Itertools};

fn parse_lights(string: &str) -> (usize, HashSet<(usize, usize)>) {
    let mut count = 0;
    let lights = string
        .split('/')
        .enumerate()
        .fold(HashSet::new(), |mut set, (y, row)| {
            count += 1;
            for (x, c) in row.chars().enumerate() {
                if c == '#' {
                    set.insert((x, y));
                }
            }
            set
        });
    (count, lights)
}

fn all_orientations(mut lights: HashSet<(usize, usize)>, size: usize) -> HashSet<Vec<(usize, usize)>> {
    let mut orientations = HashSet::new();
    orientations.insert(lights.iter().sorted().map(|p| *p).collect());
    orientations.insert(lights.iter().map(|(x, y)| (size - 1 - x, *y)).sorted().collect());
    orientations.insert(lights.iter().map(|(x, y)| (*x, size - 1 - y)).sorted().collect());
    for _ in 0..4 {
        lights = lights.iter().map(|(x, y)| (size - 1 - y, *x)).collect();
        orientations.insert(lights.iter().sorted().map(|p| *p).collect());
        orientations.insert(lights.iter().map(|(x, y)| (size - 1 - x, *y)).sorted().collect());
        orientations.insert(lights.iter().map(|(x, y)| (*x, size - 1 - y)).sorted().collect());
    }
    orientations
}

pub fn part1(input: String) {
    let rules: HashMap<(usize, Vec<(usize, usize)>), HashSet<(usize, usize)>> = input
        .lines()
        .fold(HashMap::new(), |mut map, l| {
            let mut components = l.split(" => ");
            let (size, left) = parse_lights(components.next().unwrap_or_default());
            let (_, right) = parse_lights(components.next().unwrap_or_default());
            for orientation in all_orientations(left, size) {
                map.insert((size, orientation), right.clone());
            }
            map
        });

    let mut size = 3;
    let mut lights = HashSet::<(usize, usize)>::new();
    lights.insert((1, 0));
    lights.insert((2, 1));
    lights.insert((0, 2));
    lights.insert((1, 2));
    lights.insert((2, 2));
    for i in 0..18 {
        let mut new_lights = HashSet::new();
        let step_size = if size % 2 == 0 { 2 } else { 3 };
        for (y_offset, x_offset) in iproduct!(
            0..size / step_size,
            0..size / step_size
        ) {
            let filtered = iproduct!(0..step_size, 0..step_size)
                .filter(|(x, y)| lights.contains(&(x + x_offset * step_size, y + y_offset * step_size)))
                .sorted()
                .collect_vec();

            let new_step_size = step_size + 1;
            for light in rules.get(&(step_size, filtered)).expect("No rule found") {
                new_lights.insert((light.0 + x_offset * new_step_size, light.1 + y_offset * new_step_size));
            }
        }
        size = if size % 2 == 0 { size / 2 * 3 } else { size / 3 * 4 };
        lights = new_lights;

        if i == 4 {
            println!("Part 1: {}", lights.len());
        }
    }
    println!("Part 2: {}", lights.len());
}
