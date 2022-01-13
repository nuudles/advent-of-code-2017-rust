pub fn part1(input: String) {
    let spins: usize = input.parse().unwrap_or_default();
    let mut spinlock = vec![0];
    let mut current = 0;
    for i in 1..=2017 {
        current = (current + i + spins) % spinlock.len() + 1;
        spinlock.insert(current, i);
    }
    println!("{}", spinlock[(current + 1) % spinlock.len()]);
}

pub fn part2(input: String) {
    let spins: usize = input.parse().unwrap_or_default();
    let mut spinlock = 1;
    let mut current = 0;
    let mut value = 0;
    for i in 1..=50_000_000 {
        current = (current + i + spins) % spinlock + 1;
        if current == 1 {
            value = i;
        }
        spinlock += 1;
    }
    println!("{}", value);
}
