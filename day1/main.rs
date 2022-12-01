const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test.txt");

fn main() {
    println!("1.1");
    println!("  real: {}", part1(INPUT));
    println!("  test: {}", part1(TEST_INPUT));
    println!("1.2");
    println!("  real: {}", part2(INPUT));
    println!("  test: {}", part2(TEST_INPUT));
}

fn part1(i: &str) -> String {
    i.split("\n\n")
        .map(|s| {
            s .split("\n")
                .map(|c| c.parse::<i32>().unwrap_or_default())
                .sum::<i32>()
        })
        .max()
        .unwrap()
        .to_string()
}

fn part2(i: &str) -> String {
    let mut unsorted = i
        .split("\n\n")
        .map(|s| {
            s.split("\n")
                .map(|c| c.parse::<i32>().unwrap_or_default())
                .sum::<i32>()
        }).collect::<Vec<i32>>();
    unsorted.sort();
    unsorted.reverse();
    unsorted[..3].iter().sum::<i32>().to_string()
}
