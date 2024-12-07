use rayon::prelude::*;
fn valid_equation(nums: &[u64], target_num: u64, operators: &[fn(u64, u64) -> u64]) -> bool {
    fn inner(first_num: u64, nums: &[u64], target_num: u64, operators: &[fn(u64, u64) -> u64]) -> bool {
        if nums.is_empty() {
            return first_num == target_num
        }
        return operators
            .iter()
            .any(|operation| {
                let new_first_num = operation(first_num, nums[0]);
                inner(new_first_num, &nums[1..], target_num, operators)
            })
    }
    return match &nums.len() {
        0 => false,
        1 => nums[0] == target_num,
        _ => inner(nums[0], &nums[1..], target_num, operators)
    }
}
fn apply_operators(input: &str, operators: &[fn(u64, u64) -> u64]) -> u64 {
    return input
        .par_lines()
        .map(|line| line.split_once(":").unwrap())
        .map(|(target_num, nums)| {
            let target_num: u64 = target_num.parse().unwrap();
            let nums: Vec<u64> = nums.trim().split(" ").map(|num| num.trim().parse().unwrap()).collect();
            (target_num, nums)
        })
        .filter(|(target_num, nums)| valid_equation(nums, *target_num, operators))
        .map(|(target_num, _)| target_num)
        .sum()
}

pub fn part_1(input: &str) -> u64 {
    let operators = vec![
        |x, y| x * y,
        |x, y| x + y,
    ];
    return apply_operators(input, &operators)
}
pub fn part_2(input: &str) -> u64 {
    let operators  = vec![
        |x, y| x * y,
        |x, y| x + y,
        |x: u64, y: u64| x * 10u64.pow(y.ilog10() + 1) + y,
    ];
    return apply_operators(input, &operators)
}