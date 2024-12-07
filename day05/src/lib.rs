use std::collections::HashMap;
fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let (rules, pages) = input.split_once("\n\n").unwrap();

    let rules: HashMap<u32, Vec<u32>> = rules
        .lines()
        .map(|line| {
            let (page1, page2) = line.split_once("|").unwrap();
            let page1 = page1.parse::<u32>().unwrap();
            let page2 = page2.parse::<u32>().unwrap();
            (page1, page2)
        })
        .fold(HashMap::new(), |mut map, (page1, page2)| {
            map.entry(page1).or_insert(Vec::new()).push(page2);
            map
        });

    let pages: Vec<Vec<u32>> = pages
        .lines()
        .map(|page| {
            page
                .split(",")
                .map(|num| num.parse::<u32>().unwrap())
                .collect()
        })
        .collect();

    return (rules, pages);
}

fn page_is_valid(page: &Vec<u32>, rules: &HashMap<u32, Vec<u32>>) -> bool {
    page.is_sorted_by(|a, b| rules[a].contains(b))
}

pub fn part_1(input: &str) -> u32 {
    let (rules, pages) = parse_input(input);

    pages
        .iter()
        .filter(|&page| page_is_valid(page, &rules))
        .map(|page| page[page.len() / 2])
        .sum()
}

pub fn part_2(input: &str) -> u32 {
    let (rules, mut pages) = parse_input(input);
    pages
        .iter_mut()
        .filter(|page| !page_is_valid(page, &rules))
        .map(|page| {
            page.sort_by(|a, b| rules[a].contains(b).cmp(&true));
            page[page.len() / 2]
        })
        .sum()
}
trait Encoder {
    fn encode(data: &[u8]) -> &[u8];
}
trait Decoder {
    fn decode(data: &[u8]) -> &[u8];
}