use itertools::Itertools;

fn score(chars: &Vec<char>, index: &mut usize, garbage_count: &mut u64, level: u64) -> u64 {
    let mut group_score = 0;
    match chars[*index] {
        '{' => {
            *index += 1;
            while chars[*index] != '}' {
                group_score += score(chars, index, garbage_count, level + 1);
                if chars[*index] == ',' {
                    *index += 1;
                }
            }
            *index += 1;
            group_score += level;
        },
        '<' => {
            while chars[*index] != '>' {
                if chars[*index] == '!' {
                    *index += 2;
                } else {
                    *index += 1;
                    *garbage_count += 1;
                }
            }
            *index += 1;
            // Remove '>' from garbage_count
            *garbage_count -= 1;
        },
        _ => ()
    }
    group_score
}

pub fn part1(input: String) {
    let mut index = 0;
    let mut garbage_count = 0;
    let chars = input.chars().collect_vec();
    println!("Part 1: {}", score(&chars, &mut index, &mut garbage_count, 1));
    println!("Part 2: {}", garbage_count);
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::score;

    #[test]
    fn test_score() {
        let mut index = 0;
        let mut garbage_count = 0;
        assert_eq!(score(&"{}".chars().collect_vec(), &mut index, &mut garbage_count, 1), 1);
        index = 0;
        assert_eq!(score(&"{{{}}}".chars().collect_vec(), &mut index, &mut garbage_count, 1), 6);
        index = 0;
        assert_eq!(score(&"{{},{}}".chars().collect_vec(), &mut index, &mut garbage_count, 1), 5);
        index = 0;
        assert_eq!(score(&"{{{},{},{{}}}}".chars().collect_vec(), &mut index, &mut garbage_count, 1), 16);
        index = 0;
        assert_eq!(score(&"{<a>,<a>,<a>,<a>}".chars().collect_vec(), &mut index, &mut garbage_count, 1), 1);
        index = 0;
        assert_eq!(score(&"{{<ab>},{<ab>},{<ab>},{<ab>}}".chars().collect_vec(), &mut index, &mut garbage_count, 1), 9);
        index = 0;
        assert_eq!(score(&"{{<!!>},{<!!>},{<!!>},{<!!>}}".chars().collect_vec(), &mut index, &mut garbage_count, 1), 9);
        index = 0;
        assert_eq!(score(&"{{<a!>},{<a!>},{<a!>},{<ab>}}".chars().collect_vec(), &mut index, &mut garbage_count, 1), 3);
    }
}
