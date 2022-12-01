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
    let mut calories_per_elf = INPUT
        .split("\n\n")
        .map(|s| {
            s.trim()
                .split("\n")
                .map(|c| c.parse::<i32>().unwrap_or(0))
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    calories_per_elf.sort();
    calories_per_elf.reverse();
    println!("Day 1.1: {}", calories_per_elf.iter().max().unwrap());
    println!("Day 1.2: {}", calories_per_elf[..3].iter().sum::<i32>());
}
