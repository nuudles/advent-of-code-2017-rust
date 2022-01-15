use std::collections::HashSet;

pub fn part1(_input: String) {
    let mut ones = HashSet::new();
    let mut x = 0i64;
    let mut state = 0;
    for _ in 0..12261543 {
        let is_one = ones.contains(&x);
        match state {
            0 if !is_one => {
                ones.insert(x);
                x += 1;
                state = 1;
            },
            0 => {
                ones.remove(&x);
                x -= 1;
                state = 2;
            },
            1 if !is_one => {
                ones.insert(x);
                x -= 1;
                state = 0;
            },
            1 => {
                x += 1;
                state = 2;
            },
            2 if !is_one => {
                ones.insert(x);
                x += 1;
                state = 0;
            },
            2 => {
                ones.remove(&x);
                x -= 1;
                state = 3;
            },
            3 if !is_one => {
                ones.insert(x);
                x -= 1;
                state = 4;
            },
            3 => {
                x -= 1;
                state = 2;
            },
            4 if !is_one => {
                ones.insert(x);
                x += 1;
                state = 5;
            },
            4 => {
                x += 1;
                state = 0;
            },
            5 if !is_one => {
                ones.insert(x);
                x += 1;
                state = 0;
            },
            5 => {
                x += 1;
                state = 4;
            },
            _ => ()
        }
    }
    println!("{}", ones.len());
}
