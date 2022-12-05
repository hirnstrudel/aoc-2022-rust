use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

fn main() {
    let lines = read_input_from_file("/home/benjamin/aoc-2022-rust/day03/input.txt");
    let test = vec![
        "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
        "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
        "PmmdzqPrVvPwwTWBwg".to_string(),
        "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
        "ttgJtRGJQctTZtZT".to_string(),
        "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
    ];
    solve_part1(&test);
    solve_part1(&lines);
    solve_part2(&test);
    solve_part2(&lines);
}

fn read_input_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn solve_part1(rucksacks: &Vec<String>) {
    let mut sum = 0;

    let mut compartments = [""; 2];
    for rucksack in rucksacks {
        let len = rucksack.len();
        compartments[0] = &rucksack[0..len / 2];
        compartments[1] = &rucksack[len / 2..len];

        for supply in compartments[0].chars() {
            if compartments[1].contains(supply) {
                let value: u32;
                if supply.is_lowercase() {
                    value = supply as u32 - 'a' as u32 + 1;
                } else {
                    value = supply as u32 - 'A' as u32 + 27;
                }
                sum += value as u32;
                break;
            }
        }
    }
    println!("Part 1: {}", sum);
}

fn solve_part2(rucksacks: &Vec<String>) {
    let mut sum = 0;

    for (rucksack, pos) in rucksacks.iter().zip(0..).step_by(3) {
        for badge in rucksack.chars() {
            if rucksacks[pos + 1].contains(badge) && rucksacks[pos + 2].contains(badge) {
                let value: u32;
                if badge.is_lowercase() {
                    value = badge as u32 - 'a' as u32 + 1;
                } else {
                    value = badge as u32 - 'A' as u32 + 27;
                }
                sum += value as u32;
                break;
            }
        }
    }

    println!("Part 2: {}", sum);
}
