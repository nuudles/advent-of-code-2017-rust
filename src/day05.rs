use itertools::Itertools;

pub fn part1(input: String) {
    let mut jumps = input.lines().flat_map(|n| n.parse::<i64>().ok()).collect_vec();
    let mut index = 0i64;
    let mut steps = 0;
    while let Ok(i) = usize::try_from(index) {
        if i >= jumps.len() {
            break;
        }
        let jump = jumps[i];
        jumps[i] += 1;
        index += jump;
        steps += 1;
    }
    println!("{}", steps);
}

pub fn part2(input: String) {
    let mut jumps = input.lines().flat_map(|n| n.parse::<i64>().ok()).collect_vec();
    let mut index = 0i64;
    let mut steps = 0;
    while let Ok(i) = usize::try_from(index) {
        if i >= jumps.len() {
            break;
        }
        let jump = jumps[i];
        if jump >= 3 {
            jumps[i] -= 1;
        } else {
            jumps[i] += 1;
        }
        index += jump;
        steps += 1;
    }
    println!("{}", steps);
}
