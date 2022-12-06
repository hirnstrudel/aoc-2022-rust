use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let example = vec![
        "2-4,6-8".to_string(),
        "2-3,4-5".to_string(),
        "5-7,7-9".to_string(),
        "2-8,3-7".to_string(),
        "6-6,4-6".to_string(),
        "2-6,4-8".to_string(),
    ];
    let lines = read_input_from_file("input.txt");

    println!("Part 1: {}", solve_part1(&example));
    println!("Part 1: {}", solve_part1(&lines));
    println!("Part 2: {}", solve_part2(&example));
    println!("Part 2: {}", solve_part2(&lines));
}

fn read_input_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("Unable to open file");
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn solve_part1(instructions: &Vec<String>) -> u32 {
    let mut sum = 0;

    for instruction in instructions {
        let elves = instruction.split(",").collect::<Vec<&str>>();
        let range1 = elves[0]
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let range2 = elves[1]
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if range_includes(&range1, &range2) || range_includes(&range2, &range1) {
            sum += 1;
        }
    }

    return sum;
}

fn solve_part2(instructions: &Vec<String>) -> u32 {
    let mut sum = 0;

    for instruction in instructions {
        let elves = instruction.split(",").collect::<Vec<&str>>();
        let range1 = elves[0]
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        let range2 = elves[1]
            .split("-")
            .map(|s| s.parse::<u32>().unwrap())
            .collect::<Vec<u32>>();
        if range_overlaps(&range1, &range2) || range_overlaps(&range2, &range1) {
            sum += 1;
        }
    }

    return sum;
}

fn range_includes(range1: &Vec<u32>, range2: &Vec<u32>) -> bool {
    return range1[0] <= range2[0] && range2[1] <= range1[1];
}

fn range_overlaps(range1: &Vec<u32>, range2: &Vec<u32>) -> bool {
    return range1[0] <= range2[0] && range2[0] <= range1[1]
        || range1[0] <= range2[1] && range2[1] <= range1[1];
}
