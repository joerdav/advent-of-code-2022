use std::collections::HashSet;

const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test.txt");

pub fn run() {
    println!("6.1");
    println!("  real: {}", part1(INPUT));
    println!("  test: {}", part1(TEST_INPUT));
    println!("6.2");
    println!("  real: {}", part2(INPUT));
    println!("  test: {}", part2(TEST_INPUT));
}
fn part1(i: &str) -> String {
    find_start(i, 4).to_string()
}

fn part2(i: &str) -> String {
    find_start(i, 14).to_string()
}
fn uniq(cs: &[char]) -> bool {
    let mut h = HashSet::new();
    cs.iter().all(|x| h.insert(x))
}
fn find_start(i: &str, window_size: usize) -> usize {
    i.chars()
        .collect::<Vec<char>>()
        .windows(window_size)
        .enumerate()
        .find_map(|(i, c)| if uniq(c) { Some(i+c.len()) } else { None })
        .unwrap()
}
