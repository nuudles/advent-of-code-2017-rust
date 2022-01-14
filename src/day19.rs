use itertools::Itertools;

pub fn part1(input: String) {
    let diagram = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut pos = diagram[0].iter().position(|c| c == &'|').map_or((0, 0), |x| (x as i64, 0i64));
    let mut direction = (0, 1);
    let mut letters = Vec::new();
    let mut steps = 0;
    loop {
        let (x, y) = (pos.0 as usize, pos.1 as usize);
        match diagram[y][x] {
            '+' => {
                if direction.0 == 0 {
                    if diagram[y][x + 1] != ' ' {
                        direction = (1, 0);
                    } else {
                        direction = (-1, 0);
                    }
                } else {
                    if diagram[y + 1][x] != ' ' {
                        direction = (0, 1);
                    } else {
                        direction = (0, -1);
                    }
                }
            },
            '|' | '-' => (),
            ' ' => break,
            _ => {
                letters.push(diagram[y][x]);
            }
        }
        pos.0 += direction.0;
        pos.1 += direction.1;
        steps += 1;
    }
    println!("Part 1: {}", letters.iter().collect::<String>());
    println!("Part 2: {}", steps);
}
