use std::collections::{HashMap, VecDeque};

const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test.txt");

pub fn run() {
    println!("5.1");
    println!("  real: {}", part1(INPUT));
    println!("  test: {}", part1(TEST_INPUT));
    println!("5.2");
    println!("  real: {}", part2(INPUT));
    println!("  test: {}", part2(TEST_INPUT));
}

fn part1(i: &str) -> String {
    let (stack_string, instruction_string) = i.split_once("\n\n").unwrap();
    let stack_lines = stack_string.split("\n").collect::<Vec<&str>>();
    let stack_strings = &stack_lines[..stack_lines.len()-1];
    let stack = stack_strings.iter().fold(HashMap::new(), |mut acc: HashMap<usize, VecDeque<char>>, curr| {
        curr.chars().collect::<Vec<char>>().chunks(4).enumerate().for_each(|(idx, name)| {
            if name.iter().all(|c| c.is_whitespace()) {
                return;
            }
            acc.entry(idx+1).or_insert(VecDeque::new()).push_front(*name.get(1).unwrap());
        });
        acc
    });
    let result = instruction_string.trim().split("\n")
        .map(|l| l.split_whitespace().map(|n| n.parse::<usize>().unwrap_or(0)).collect::<Vec<usize>>())
        .fold(stack.clone(), |mut acc: HashMap<usize, VecDeque<char>>, curr| {
            let amt = curr[1];
            let from = curr[3];
            let to = curr[5];
            for _idx in 0..amt {
                let item = acc.get_mut(&from).unwrap().pop_back().unwrap();
                acc.get_mut(&to).unwrap().push_back(item);
            }
            acc.clone()
        }).clone();
    let mut result_string = String::from("");
    for idx in 1..=result.len() {
        result_string+=&result.get(&idx).unwrap().iter().last().unwrap().to_string();
    }
    result_string
}

fn part2(i: &str) -> String {
    let (stack_string, instruction_string) = i.split_once("\n\n").unwrap();
    let stack_lines = stack_string.split("\n").collect::<Vec<&str>>();
    let stack_strings = &stack_lines[..stack_lines.len()-1];
    let stack = stack_strings.iter().fold(HashMap::new(), |mut acc: HashMap<usize, VecDeque<char>>, curr| {
        curr.chars().collect::<Vec<char>>().chunks(4).enumerate().for_each(|(idx, name)| {
            if name.iter().all(|c| c.is_whitespace()) {
                return;
            }
            acc.entry(idx+1).or_insert(VecDeque::new()).push_front(*name.get(1).unwrap());
        });
        acc
    });
    let result = instruction_string.trim().split("\n")
        .map(|l| l.split_whitespace().map(|n| n.parse::<usize>().unwrap_or(0)).collect::<Vec<usize>>())
        .fold(stack.clone(), |mut acc: HashMap<usize, VecDeque<char>>, curr| {
            let amt = curr[1];
            let from = curr[3];
            let to = curr[5];
            let len = acc.get(&from).unwrap().len();
            let item = acc.get_mut(&from).unwrap().drain(len-amt..).collect::<Vec<char>>();
            for idx in 0..item.len() {
                acc.get_mut(&to).unwrap().push_back(item[idx]);
            }
            acc.clone()
        }).clone();
    let mut result_string = String::from("");
    for idx in 1..=result.len() {
        result_string+=&result.get(&idx).unwrap().iter().last().unwrap().to_string();
    }
    result_string
}
