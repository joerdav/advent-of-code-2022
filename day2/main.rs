const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test.txt");

fn main() {
    println!("2.1");
    println!("  real: {}", part1(INPUT));
    println!("  test: {}", part1(TEST_INPUT));
    println!("2.2");
    println!("  real: {}", part2(INPUT));
    println!("  test: {}", part2(TEST_INPUT));
}

fn part1(i: &str) -> String {
    i.trim().split("\n").map(|l| {
        let codes: Vec<&str> = l.split(" ").collect();
        let opp = get_hand(codes[0]);
        let rec = get_hand(codes[1]);
        rec.score()+outcome(&opp, &rec).score()
    }).sum::<i32>().to_string()
}

fn part2(i: &str) -> String {
    i.trim().split("\n").map(|l| {
        let codes: Vec<&str> = l.split(" ").collect();
        let opp = get_hand(codes[0]);
        let rec = rec_hand(codes[1], &opp);
        rec.score()+outcome(&opp, &rec).score()
    }).sum::<i32>().to_string()
}

#[derive(Copy, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn score(&self) -> i32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

#[derive(Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

impl Outcome {
    fn score(&self) -> i32 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn get_hand(c: &str) -> Hand {
    match c {
        "A" | "X" => Hand::Rock,
        "B" | "Y" => Hand::Paper,
        "C" | "Z" => Hand::Scissors,
        _ => panic!("Invalid Hand: {}", c),
    }
}
fn rec_hand(outcome: &str, opp: &Hand) -> Hand {
    match (opp, outcome) {
        (Hand::Rock, "X") => Hand::Scissors,
        (Hand::Rock, "Z") => Hand::Paper,

        (Hand::Paper, "X") => Hand::Rock,
        (Hand::Paper, "Z") => Hand::Scissors,
        
        (Hand::Scissors, "X") => Hand::Paper,
        (Hand::Scissors, "Z") => Hand::Rock,

        (_, "Y") => *opp,
        _ => panic!("Invalid Hand: {}",outcome),
    }
}

fn outcome(o: &Hand, p: &Hand) -> Outcome {
    match (o, p) {
        (Hand::Scissors, Hand::Paper) |
            (Hand::Paper, Hand::Rock) |
            (Hand::Rock, Hand::Scissors)  => Outcome::Lose,
        (Hand::Paper, Hand::Scissors) |
            (Hand::Rock, Hand::Paper) |
            (Hand::Scissors, Hand::Rock)  => Outcome::Win,
            _ => Outcome::Draw
    }
}

