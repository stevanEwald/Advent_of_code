use regex::Regex;
pub fn part_1(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|capture| capture.extract())
        .map(|(_, [num_1, num_2])| {
            let num_1 = num_1.parse::<i32>().unwrap();
            let num_2 = num_2.parse::<i32>().unwrap();
            num_1 * num_2
        })
        .sum()
}

pub fn part_2(input: &str) -> i32 {
    let regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    return regex
        .captures_iter(input)
        .filter_map(|cap| {
            return match &cap[0] {
                "do()" => {
                    enabled = true;
                    None
                }
                "don't()" => {
                    enabled = false;
                    None
                }
                _mul => {
                    if !enabled {
                        return None;
                    }
                    let num_1 = cap[1].parse::<i32>().unwrap();
                    let num_2 = cap[2].parse::<i32>().unwrap();
                    return Some(num_1 * num_2);
                }
            };
        })
        .sum();
}
