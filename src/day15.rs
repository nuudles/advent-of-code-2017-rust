pub fn part1(_input: String) {
    let mut a: u64 = 65; // Replace based on input
    let mut b: u64 = 8921; // Replace based on input
    let mut matches = 0;
    for _ in 0..40_000_000 {
        a = a * 16807 % 2147483647;
        b = b * 48271 % 2147483647;
        if (a ^ b) & 0xFFFF == 0 {
            matches += 1;
        }
    }
    println!("{}", matches);
}

pub fn part2(_input: String) {
    let mut a: u64 = 65; // Replace based on input
    let mut b: u64 = 8921; // Replace based on input
    let mut matches = 0;
    for _ in 0..5_000_000 {
        loop {
            a = a * 16807 % 2147483647;
            if a % 4 == 0 {
                break;
            }
        }
        loop {
            b = b * 48271 % 2147483647;
            if b % 8 == 0 {
                break;
            }
        }
        if (a ^ b) & 0xFFFF == 0 {
            matches += 1;
        }
    }
    println!("{}", matches);
}
