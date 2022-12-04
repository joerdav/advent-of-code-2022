const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test.txt");

fn main() {
    println!("4.1");
    println!("  real: {}", part1(INPUT));
    println!("  test: {}", part1(TEST_INPUT));
    println!("4.2");
    println!("  real: {}", part2(INPUT));
    println!("  test: {}", part2(TEST_INPUT));
}

struct Range(i32, i32);
struct Pair(Range, Range);

impl From<&str> for Pair {
    fn from(i: &str) -> Self {
        let s = i.trim().split(",").collect::<Vec<&str>>();
        Pair(Range::from(s[0]), Range::from(s[1]))
    }
}
impl From<&str> for Range {
    fn from(i: &str) -> Self {
        let s = i.trim().split("-").collect::<Vec<&str>>();
        Range(s[0].parse::<i32>().unwrap(), s[1].parse::<i32>().unwrap())
    }
}
impl Range {
    fn fits(&self, other: &Range) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
    fn overlap(&self, other: &Range) -> bool {
        (self.0 <= other.0 && self.1 >= other.0)
            || (self.0 <= other.1 && self.1 >= other.1)
            || self.fits(other)
            || other.fits(self)
    }
}

fn part1(i: &str) -> String {
    i.trim()
        .split("\n")
        .into_iter()
        .map(Pair::from)
        .filter(|p| p.0.fits(&p.1) || p.1.fits(&p.0))
        .count()
        .to_string()
}

fn part2(i: &str) -> String {
    i.trim()
        .split("\n")
        .into_iter()
        .map(Pair::from)
        .filter(|p| p.0.overlap(&p.1))
        .count()
        .to_string()
}
