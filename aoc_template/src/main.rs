use std::fs;
use aoc::{part_1, part_2};

fn read_input() -> String {
    let path = env::var("CARGO_MANIFEST_DIR").unwrap() + "/input.txt";
    return fs::read_to_string(path).unwrap()
}
fn main() {
    let input = read_input();
    let part_1_result = part_1(&input);
    dbg!(part_1_result);
    let part_2_result = part_2(&input);
    dbg!(part_2_result);
}

// mod tests {
//     use super::*;

//     #[test]
//     fn part_1_test() {
//         let input = read_input();
//         let part_1_result = part_1(&input);
//         assert_eq!(part_1_result, );
//     }
//     #[test]
//     fn part_2_test() {
//         let input = read_input();
//         let part_2_result = part_2(&input);
//         assert_eq!(part_2_result, );
//     }
// }