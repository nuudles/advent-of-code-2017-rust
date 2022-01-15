use std::collections::HashSet;

pub fn part1(input: String) {
    let mut infections = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row
                .chars()
                .enumerate()
                .flat_map(move |(x, c)|
                    if c == '#' {
                        Some((i64::try_from(x).unwrap_or_default(), i64::try_from(y).unwrap_or_default()))
                    } else {
                        None
                    }
                )
        })
        .collect::<HashSet<_>>();
    let size = i64::try_from(input.lines().count()).unwrap_or_default();
    let directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0)
    ];
    let mut dir_index = 0;
    let mut pos = (size / 2, size / 2);
    let mut count = 0;
    for _ in 0..10000 {
        let is_infected = infections.contains(&pos);
        if is_infected {
            dir_index = (dir_index + 1) % directions.len();
            infections.remove(&pos);
        } else {
            dir_index = dir_index.checked_sub(1).unwrap_or(directions.len() - 1);
            infections.insert(pos);
            count += 1;
        }

        let dir = directions[dir_index];
        pos.0 += dir.0;
        pos.1 += dir.1;
    }
    println!("{}", count);
}

pub fn part2(input: String) {
    let mut infected = input
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row
                .chars()
                .enumerate()
                .flat_map(move |(x, c)|
                    if c == '#' {
                        Some((i64::try_from(x).unwrap_or_default(), i64::try_from(y).unwrap_or_default()))
                    } else {
                        None
                    }
                )
        })
        .collect::<HashSet<_>>();
    let mut weakened = HashSet::<(i64, i64)>::new();
    let mut flagged = HashSet::<(i64, i64)>::new();
    let size = i64::try_from(input.lines().count()).unwrap_or_default();
    let directions = [
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0)
    ];
    let mut dir_index = 0;
    let mut pos = (size / 2, size / 2);
    let mut count = 0;
    for _ in 0..10000000 {
        if infected.remove(&pos) {
            flagged.insert(pos);
            dir_index = (dir_index + 1) % directions.len();
        } else if weakened.remove(&pos) {
            infected.insert(pos);
            count += 1;
        } else if flagged.remove(&pos) {
            dir_index = (dir_index + 2) % directions.len();
        } else {
            weakened.insert(pos);
            dir_index = dir_index.checked_sub(1).unwrap_or(directions.len() - 1);
        }

        let dir = directions[dir_index];
        pos.0 += dir.0;
        pos.1 += dir.1;
    }
    println!("{}", count);
}
