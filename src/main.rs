use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};
use std::num::ParseIntError;
use std::path::Path;
use std::str::FromStr;

fn main() {
    let path = Path::new("../input.txt");

    //part 1
    {
        let file = File::open(path).unwrap();
        let mut lines = io::BufReader::new(file).lines().map(|l| l.unwrap());
        let mut stacks_p1 = stacks_parse_lines(lines.by_ref().take_while(|s| !String::is_empty(s)));
        let mut stacks_p2 = stacks_p1.clone();
        let commands = lines.map(|l| Command::from_str(&l).unwrap());
        for command in commands {
            // let sorted = stacks_p1.iter().sorted_by_key(|(k, _)| **k).collect_vec();
            // println!(
            //     "{}",
            //     sorted
            //         .iter()
            //         .map(|(_, v)| v.len())
            //         .map(|s| format!("{:>2}", s))
            //         .join(",")
            // );
            // println!(
            //     "{}",
            //     sorted
            //         .iter()
            //         .map(|(k, _)| k)
            //         .map(|s| format!("{:>2}", s))
            //         .join(",")
            // );
            // println!("\n{:?}\n", command);
            command.run(&mut stacks_p1, true);
            command.run(&mut stacks_p2, false);
        }
        println!(
            "part 1: {}",
            stacks_p1
                .iter()
                .sorted_by_key(|(k, _)| **k)
                .map(|(_, stack)| stack.last().unwrap())
                .join("")
        );
        println!(
            "part 2: {}",
            stacks_p2
                .iter()
                .sorted_by_key(|(k, _)| **k)
                .map(|(_, stack)| stack.last().unwrap())
                .join("")
        );
    }
}

#[derive(Debug)]
struct Command<Key> {
    amount: usize,
    from: Key,
    to: Key,
}

impl<Key: Eq + Hash> Command<Key> {
    fn run<Item>(&self, stacks: &mut HashMap<Key, Vec<Item>>, reverse: bool) {
        let from_stack = stacks.get_mut(&self.from).unwrap();
        let l = from_stack.len();
        let range = (l - self.amount)..l;
        let items: Vec<Item> = from_stack.drain(range).collect();
        let to_stack = stacks.get_mut(&self.to).unwrap();
        if reverse {
            to_stack.extend(items.into_iter().rev());
        } else {
            to_stack.extend(items);
        }
    }
}

impl FromStr for Command<char> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MOVE_REG: Regex = Regex::new(r"move ([^ ]+)").unwrap();
            static ref FROM_REG: Regex = Regex::new(r"from ([^ ])").unwrap();
            static ref TO_REG: Regex = Regex::new(r"to ([^ ])").unwrap();
        }
        Ok(Command {
            amount: MOVE_REG
                .captures(&s)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .parse::<usize>()?,
            from: FROM_REG
                .captures(&s)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap(),
            to: TO_REG
                .captures(&s)
                .unwrap()
                .get(1)
                .unwrap()
                .as_str()
                .chars()
                .next()
                .unwrap(),
        })
    }
}

fn stacks_parse_lines<I>(lines: I) -> HashMap<char, Vec<char>>
where
    I: Iterator<Item = String>,
{
    lazy_static! {
        static ref CRATES_REG: Regex = Regex::new(r"\[([^ ])\]").unwrap();
        static ref NAMES_REG: Regex = Regex::new(r" ([^ ]) ").unwrap();
    }

    let mut stacks: HashMap<usize, Vec<char>> = HashMap::new();
    let mut lines = lines.peekable();
    let mut last_line = String::default();
    while let Some(line) = lines.next() {
        if let None = lines.peek() {
            last_line = line;
            break;
        } else {
            for captures in CRATES_REG.captures_iter(&line) {
                let mat = captures.get(1).unwrap();
                stacks
                    .entry(mat.start())
                    .or_default()
                    .push(mat.as_str().chars().next().unwrap());
            }
        };
    }
    let mut named_stacks: HashMap<char, Vec<char>> = HashMap::new();
    for captures in NAMES_REG.captures_iter(&last_line) {
        let mat = captures.get(1).unwrap();
        let key = mat.start();
        let name = mat.as_str().chars().next().unwrap();
        let mut stack = stacks.remove(&key).unwrap();
        stack.reverse();
        named_stacks.insert(name, stack);
    }
    named_stacks
}
