use std::collections::HashMap;

pub fn part1(input: String) {
    let target = input.parse::<i64>().unwrap_or_default();
    let mut pos = (0i64, 0i64);
    let directions = [
        (1, 0),
        (0, -1),
        (-1, 0),
        (0, 1)
    ];
    let mut index = 0;
    let mut min = (0, 0);
    let mut max = (0, 0);
    for _ in 1..target {
        let direction = directions[index];
        pos = (pos.0 + direction.0, pos.1 + direction.1);
        if pos.0 > max.0 {
            max.0 = pos.0;
            index = (index + 1) % 4;
        } else if pos.0 < min.0 {
            min.0 = pos.0;
            index = (index + 1) % 4;
        } else if pos.1 > max.1 {
            max.1 = pos.1;
            index = (index + 1) % 4;
        } else if pos.1 < min.1 {
            min.1 = pos.1;
            index = (index + 1) % 4;
        }
    }
    println!("{}", pos.0.abs() + pos.1.abs());
}

pub fn part2(input: String) {
    let target = input.parse::<u64>().unwrap_or_default();
    let mut pos = (0i64, 0i64);
    let directions = [
        (1, 0),
        (0, -1),
        (-1, 0),
        (0, 1)
    ];
    let neighbors = [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1)
    ];
    let mut index = 0;
    let mut min = (0, 0);
    let mut max = (0, 0);
    let mut map = HashMap::new();
    map.insert((0, 0), 1u64);
    loop {
        let direction = directions[index];
        pos = (pos.0 + direction.0, pos.1 + direction.1);
        let value = neighbors.iter().map(|(dx, dy)| map.get(&(pos.0 + dx, pos.1 + dy)).unwrap_or(&0)).sum();
        if value > target {
            println!("{}", value);
            break;
        }
        map.insert(pos, value);
        if pos.0 > max.0 {
            max.0 = pos.0;
            index = (index + 1) % 4;
        } else if pos.0 < min.0 {
            min.0 = pos.0;
            index = (index + 1) % 4;
        } else if pos.1 > max.1 {
            max.1 = pos.1;
            index = (index + 1) % 4;
        } else if pos.1 < min.1 {
            min.1 = pos.1;
            index = (index + 1) % 4;
        }
    }
}
