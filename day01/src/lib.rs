use std::collections::HashMap;
fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|line| line.split_once("   ").unwrap())
        .map(|(x, y)| {
            let x = x.parse::<i32>().unwrap();
            let y = y.parse::<i32>().unwrap();
            (x, y)
        })
        .unzip()
}

pub fn part_1(input: &str) -> i32 {
    let (mut column_1, mut column_2) = parse_input(input);

    column_1.sort();
    column_2.sort();

    return column_1
        .iter()
        .zip(column_2)
        .map(|(num1, num2)| (num1 - num2).abs())
        .sum();
}

pub fn part_2(input: &str) -> i32 {
    let (column_1, column_2) = parse_input(input);

    let column_2_counts = column_2.into_iter().fold(HashMap::new(), |mut counts, x| {
        *counts.entry(x).or_insert(0) += 1;
        counts
    });

    return column_1
        .iter()
        .map(|x| x * column_2_counts.get(x).unwrap_or(&0))
        .sum();
}
