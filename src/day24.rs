use std::collections::HashSet;

use itertools::Itertools;

fn scores(start: u64, count: usize, score: u64, pipes: HashSet<(u64, u64)>) -> HashSet<(usize, u64)> {
    let scores = pipes
        .iter()
        .filter(|p| p.0 == start || p.1 == start)
        .flat_map(|p| {
            let mut new_pipes = pipes.clone();
            new_pipes.remove(&p);
            scores(if p.0 == start { p.1 } else { p.0 }, count + 1, score + p.0 + p.1, new_pipes)
                .iter()
                .map(|t| *t)
                .collect_vec()
        })
        .collect::<HashSet<_>>();
    if scores.len() > 0 {
        scores
    } else {
        let mut single = HashSet::new();
        single.insert((count + 1, score));
        single
    }
}

pub fn part1(input: String) {
    let pipes = input
        .lines()
        .map(|l| {
            let mut components = l.split("/");
            (
                components.next().unwrap_or_default().parse().unwrap_or_default(),
                components.next().unwrap_or_default().parse().unwrap_or_default()
            )
        })
        .collect();
    let all_scores = scores(0, 0, 0, pipes);
    println!("Part 1: {}", all_scores.iter().map(|(_, s)| s).max().unwrap_or(&0));
    println!("Part 2: {:?}", all_scores.iter().max_by(|a, b| a.0.cmp(&&b.0).then(a.1.cmp(&&b.1))).map_or(0, |t| t.1));
}
