use itertools::Itertools;

fn same_sign(x: i32, y: i32) -> bool {
    (x >= 0) == (y >= 0)
}

fn is_safe(line: &[i32]) -> bool {
    return line
        .iter()
        .tuple_windows()
        .map(|(num1, num2)| num1 - num2)
        .tuple_windows()
        .all(|(delta1, delta2)| {
            (1..=3).contains(&delta1.abs())
                && (1..=3).contains(&delta2.abs())
                && same_sign(delta1, delta2)
        });
}

pub fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|nums| is_safe(nums))
        .count() as i32
}

fn is_safe_with_buffer(line: &[i32]) -> bool {
    if is_safe(line) {
        return true;
    }
    return (0..line.len())
        .map(|i| {
            line.iter()
                .enumerate()
                .filter(|(idx, _)| *idx != i)
                .map(|(_, num)| *num)
                .collect::<Vec<_>>()
        })
        .any(|buffered_line| is_safe(&buffered_line));
}

pub fn part_2(input: &str) -> i32 {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| is_safe_with_buffer(line))
        .count() as i32
}
