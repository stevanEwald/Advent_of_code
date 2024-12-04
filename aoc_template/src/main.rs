use std::env;
use std::fs;
use aoc::{part_1, part_2};
fn main() {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input.txt";
    let input = fs::read_to_string(path).unwrap();
    let part_1_result = part_1(&input);
    dbg!(part_1_result);
    let part_2_result = part_2(&input);
    dbg!(part_2_result);
}
