const INPUT: &str = include_str!("./input.txt");
//const INPUT: &str = "1000
//2000
//3000

//4000

//5000
//6000

//7000
//8000
//9000

//10000";

fn main() {
    println!("Day 1.1: {}", part1());
    println!("Day 1.2: {}", part2());
}

fn part1() -> i32 {
    INPUT
        .split("\n\n")
        .map(|s| {
            s.trim()
                .split("\n")
                .map(|c| c.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .max()
        .unwrap()
}

fn part2() -> i32 {
    let mut unsorted = INPUT
        .split("\n\n")
        .map(|s| {
            s.trim()
                .split("\n")
                .map(|c| c.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        }).collect::<Vec<i32>>();
    unsorted.sort();
    unsorted.reverse();
    unsorted[..3].iter().sum()
}
