use std::collections::HashMap;

const INPUT: &str = include_str!("./input.txt");
const TEST_INPUT: &str = include_str!("./test.txt");

pub fn run() {
    println!("7.1");
    println!("  real: {}", part1(INPUT));
    println!("  test: {}", part1(TEST_INPUT));
    println!("7.2");
    println!("  real: {}", part2(INPUT));
    println!("  test: {}", part2(TEST_INPUT));
}

enum Node {
    File(i32),
    Dir(Dir),
}

struct Dir {
    map: HashMap<String, Node>,
}
impl Dir {
    fn new() -> Self {
        Dir {
            map: HashMap::new(),
        }
    }
    fn add_file(&mut self, name: String, size: i32) {
        self.map.insert(name, Node::File(size));
    }
    fn add_dir(&mut self, name: String) {
        self.map.insert(name, Node::Dir(Self::new()));
    }
    fn get_dir(&mut self, path: &Vec<&str>) -> &mut Dir {
        if path.is_empty() {
            self
        } else {
            match self.map.get_mut(path[0]).unwrap() {
                Node::Dir(d) => d.get_dir(&path[1..].to_vec()),
                _ => panic!("not a dir"),
            }
        }
    }
    fn size(&self) -> i32 {
        self.map
            .iter()
            .map(|(_, n)| match n {
                Node::Dir(d) => d.size(),
                Node::File(f) => *f,
            })
            .sum()
    }
    fn values<'a>(&'a self) -> Box<dyn Iterator<Item = &Dir> + 'a> {
        Box::new(
            self.map
                .iter()
                .filter_map(|v| match v.1 {
                    Node::Dir(d) => Some(d),
                    _ => None,
                })
                .chain(
                    self.map
                        .iter()
                        .filter_map(|v| match v.1 {
                            Node::Dir(n) => Some(n.values()),
                            _ => None,
                        })
                        .flatten(),
                ),
        )
    }
}

fn part1(i: &str) -> String {
    let mut cwd: Vec<&str> = Vec::new();
    let mut tree = Dir::new();
    i.split("$").skip(1).for_each(|seg| {
        let lines = seg.trim().lines().collect::<Vec<&str>>();
        let command_segments: Vec<&str> = lines.first().unwrap().split(" ").collect();
        match (
            command_segments.first().unwrap_or(&""),
            command_segments.get(1).unwrap_or(&""),
        ) {
            (&"cd", dir) => match dir {
                &".." => {
                    cwd.remove(cwd.len() - 1);
                }
                &"/" => {
                    cwd.clear();
                }
                &name => cwd.push(name),
            },
            (&"ls", _) => lines[1..].iter().for_each(|nl| {
                let node_info: Vec<&str> = nl.split(" ").collect();
                match node_info[0] {
                    "dir" => tree.get_dir(&cwd).add_dir(node_info[1].to_string()),
                    _ => tree
                        .get_dir(&cwd)
                        .add_file(node_info[1].to_string(), node_info[0].parse().unwrap()),
                }
            }),
            _ => println!("other"),
        };
    });
    tree.values().map(|n| n.size()).filter(|s| s <= &100000).sum::<i32>().to_string()
}

fn part2(i: &str) -> String {
    let mut cwd: Vec<&str> = Vec::new();
    let mut tree = Dir::new();
    i.split("$").skip(1).for_each(|seg| {
        let lines = seg.trim().lines().collect::<Vec<&str>>();
        let command_segments: Vec<&str> = lines.first().unwrap().split(" ").collect();
        match (
            command_segments.first().unwrap_or(&""),
            command_segments.get(1).unwrap_or(&""),
        ) {
            (&"cd", dir) => match dir {
                &".." => {
                    cwd.remove(cwd.len() - 1);
                }
                &"/" => {
                    cwd.clear();
                }
                &name => cwd.push(name),
            },
            (&"ls", _) => lines[1..].iter().for_each(|nl| {
                let node_info: Vec<&str> = nl.split(" ").collect();
                match node_info[0] {
                    "dir" => tree.get_dir(&cwd).add_dir(node_info[1].to_string()),
                    _ => tree
                        .get_dir(&cwd)
                        .add_file(node_info[1].to_string(), node_info[0].parse().unwrap()),
                }
            }),
            _ => println!("other"),
        };
    });
    let free =70000000-tree.size();
    let free_target = 30000000-free;
    let mut options = tree.values().map(|n| n.size()).filter(|s| s >= &free_target).collect::<Vec<i32>>();
    options.sort();
    options.first().unwrap().to_string()
}
