use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

fn main() {
    let example = vec![
        "    [D]    ".to_string(),
        "[N] [C]    ".to_string(),
        "[Z] [M] [P]".to_string(),
        " 1   2   3 ".to_string(),
        "".to_string(),
        "move 1 from 2 to 1".to_string(),
        "move 3 from 1 to 3".to_string(),
        "move 2 from 2 to 1".to_string(),
        "move 1 from 1 to 2".to_string(),
    ];

    solve_part1(&example);
    solve_part1(&read_input_from_file(
        "/home/benjamin/aoc-2022-rust/day05/input.txt",
    ));
    solve_part2(&example);
    solve_part2(&read_input_from_file(
        "/home/benjamin/aoc-2022-rust/day05/input.txt",
    ));
}

fn read_input_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn solve_part1(input: &Vec<String>) {
    let num_stacks = (input[0].len() + 1) / 4;
    let mut stacks = vec![Vec::new(); num_stacks];
    let mut iter = input.iter();

    let re = Regex::new(r"(?:\[([A-Z])\])|(?:\s{4})").unwrap();
    for state in iter.by_ref() {
        if state.len() == 0 {
            break;
        }

        for (i, cap) in re.captures_iter(state).enumerate() {
            match cap.get(1) {
                Some(m) => {
                    stacks[i].insert(0, m.as_str().chars().nth(0).unwrap());
                }
                None => {}
            }
        }
    }

    let re_command = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for state in iter.by_ref() {
        let cap = re_command.captures(state).unwrap();
        let loops = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        for _ in 0..loops {
            let c_opt = stacks[cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1].pop();
            match c_opt {
                Some(c) => {
                    stacks[cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1].push(c);
                }
                None => {
                    panic!("No more elements to move");
                }
            }
        }
    }

    let result = stacks
        .iter()
        .map(|s| {
            s.iter()
                .collect::<String>()
                .chars()
                .last()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    println!("{}", result);
}

fn solve_part2(input: &Vec<String>) {
    let num_stacks = (input[0].len() + 1) / 4;
    let mut stacks = vec![Vec::new(); num_stacks];
    let mut iter = input.iter();

    let re = Regex::new(r"(?:\[([A-Z])\])|(?:\s{4})").unwrap();
    for state in iter.by_ref() {
        if state.len() == 0 {
            break;
        }

        for (i, cap) in re.captures_iter(state).enumerate() {
            match cap.get(1) {
                Some(m) => {
                    stacks[i].insert(0, m.as_str().chars().nth(0).unwrap());
                }
                None => {}
            }
        }
    }

    let re_command = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for state in iter.by_ref() {
        let cap = re_command.captures(state).unwrap();
        let loops = cap.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let end_of_target =
            stacks[cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1].len();
        for _ in 0..loops {
            let c_opt = stacks[cap.get(2).unwrap().as_str().parse::<usize>().unwrap() - 1].pop();
            match c_opt {
                Some(c) => {
                    stacks[cap.get(3).unwrap().as_str().parse::<usize>().unwrap() - 1]
                        .insert(end_of_target, c);
                }
                None => {
                    panic!("No more elements to move");
                }
            }
        }
    }

    let result = stacks
        .iter()
        .map(|s| {
            s.iter()
                .collect::<String>()
                .chars()
                .last()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>()
        .join("");

    println!("{}", result);
}
