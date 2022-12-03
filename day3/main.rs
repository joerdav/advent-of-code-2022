const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test.txt");

fn main() {
    println!("3.1");
    println!("  real: {}", part1(INPUT));
    println!("  test: {}", part1(TEST_INPUT));
    println!("3.2");
    println!("  real: {}", part2(INPUT));
    println!("  test: {}", part2(TEST_INPUT));
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn priority(character: char) -> i32 {
    ALPHABET
        .find(character)
        .expect("Bad char for char_to_alpha_idx") as i32
        + 1
}

fn cut(i: &str) -> Vec<&str> {
    vec![&i[..i.len() / 2], &i[i.len() / 2..]]
}
fn common_char(is: Vec<&str>) -> char {
    ALPHABET
        .chars()
        .find(|l| is.iter().all(|i| i.chars().any(|r| *l == r)))
        .unwrap()
}
fn part1(i: &str) -> String {
    i.trim()
        .split_whitespace()
        .map(cut)
        .map(common_char)
        .map(priority)
        .sum::<i32>()
        .to_string()
}

fn part2(i: &str) -> String {
    i.trim()
        .split_whitespace()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|w| common_char(w.to_vec()))
        .map(priority)
        .sum::<i32>()
        .to_string()
}
