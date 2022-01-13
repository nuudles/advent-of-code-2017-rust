use itertools::Itertools;

pub fn part1(input: String) {
    let max = 256;
    let mut list = (0..max).collect_vec();
    let mut current = 0;
    let mut skip_size = 0;
    for length in input.split(",").flat_map(|n| n.parse::<usize>().ok()) {
        for i in 0..length / 2 {
            let lower_index = (current + i) % max;
            let upper_index = (current + length - i - 1) % max;
            let tmp = list[lower_index];
            list[lower_index] = list[upper_index];
            list[upper_index] = tmp;
        }
        current += length + skip_size;
        skip_size += 1;
    }
    println!("{}", list[0] * list[1]);
}

pub fn part2(input: String) {
    let mut lengths = input.bytes().map(|x| x as usize).collect_vec();
    lengths.append(&mut vec![17, 31, 73, 47, 23]);

    let max = 256;
    let mut list = (0..max).collect_vec();
    let mut current = 0;
    let mut skip_size = 0;
    for _ in 0..64 {
        for length in &lengths {
            for i in 0..length / 2 {
                let lower_index = (current + i) % max;
                let upper_index = (current + length - i - 1) % max;
                let tmp = list[lower_index];
                list[lower_index] = list[upper_index];
                list[upper_index] = tmp;
            }
            current += length + skip_size;
            skip_size += 1;
        }
    }
    let dense_hash = list.chunks(16).map(|c| c.iter().fold(0, |r, n| r ^ n));
    let as_hex = dense_hash.fold(String::new(), |string, x| string + format!("{:02x}", x).as_str());
    println!("{}", as_hex);
}
