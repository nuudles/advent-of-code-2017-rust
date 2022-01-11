use itertools::Itertools;

pub fn part1(input: String) {
    let mut count = input
        .bytes()
        .tuple_windows()
        .map(|(a, b)| if a == b { (a - b'0') as u64 } else { 0 })
        .sum::<u64>();
    if input.bytes().nth(0) == input.bytes().last() {
        count += (input.bytes().nth(0).unwrap_or_default() - b'0') as u64;
    }
    println!("{}", count);
}

pub fn part2(input: String) {
    let bytes = input.bytes().collect_vec();
    let len = bytes.len();
    let mut count = 0;
    for i in 0..len {
        if bytes[i] == bytes[(i + len / 2) % len] {
            count += (bytes[i] - b'0') as u64;
        }
    }
    println!("{}", count);
}
