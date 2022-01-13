use itertools::Itertools;
use regex::Regex;

pub fn part1(input: String) {
    let mut programs = "abcdefghijklmnop".chars().collect_vec();
    let len = programs.len();

    let re = Regex::new(r"s(\d+)|x(\d+)/(\d+)|p([a-z])/([a-z])").expect("Invalid Regex");
    let mut seen = vec![];
    for i in 0..1_000_000_000 {
        for capture in re.captures_iter(&input) {
            if let Some(count) = capture.get(1).and_then(|n| n.as_str().parse::<usize>().ok()) {
                // Spin
                programs.rotate_right(count % len);
            } else if let (Some(a), Some(b)) = (
                capture.get(2).and_then(|n| n.as_str().parse::<usize>().ok()),
                capture.get(3).and_then(|n| n.as_str().parse::<usize>().ok())
            ) {
                // Exchange
                let tmp = programs[a];
                programs[a] = programs[b];
                programs[b] = tmp;
            } else if let (Some(a), Some(b)) = (
                capture.get(4).and_then(|n| n.as_str().chars().next()),
                capture.get(5).and_then(|n| n.as_str().chars().next())
            ) {
                // Partner
                let a_pos = programs.iter().position(|c| c == &a).unwrap_or_default();
                let b_pos = programs.iter().position(|c| c == &b).unwrap_or_default();

                let tmp = programs[a_pos];
                programs[a_pos] = programs[b_pos];
                programs[b_pos] = tmp;
            }
        }
        let string = programs.iter().collect::<String>();
        if i == 0 {
            println!("Part 1: {}", string);
        }
        if seen.contains(&string) {
            println!("Part 2: {}", seen[(1_000_000_000 % i) - 1]);
            break;
        }
        seen.push(string);
    }
}
