use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    let example_1 = vec!["mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()];
    let example_2 = vec!["bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()];
    let example_3 = vec!["nppdvjthqldpwncqszvftbrmjlhg".to_string()];
    let example_4 = vec!["nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()];
    let example_5 = vec!["zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()];
    let input = read_input_from_file("input.txt");

    solve_part1(&example_1);
    solve_part1(&example_2);
    solve_part1(&example_3);
    solve_part1(&example_4);
    solve_part1(&example_5);
    solve_part1(&input);

    solve_part2(&example_1);
    solve_part2(&example_2);
    solve_part2(&example_3);
    solve_part2(&example_4);
    solve_part2(&example_5);
    solve_part2(&input);
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
    let signal = input[0].chars().collect::<Vec<char>>();
    let position = find_unique_sequence(&signal, 4);
    println!("{}", position);
}

fn solve_part2(input: &Vec<String>) {
    let signal = input[0].chars().collect::<Vec<char>>();
    let position = find_unique_sequence(&signal, 14);
    println!("{}", position);
}

fn find_unique_sequence(signal: &Vec<char>, length: usize) -> usize {
    let mut position = length;
    while position < signal.len() {
        let seq = signal[position - length..position].to_vec();
        if !(1..seq.len()).any(|i| seq[i..].contains(&seq[i - 1])) {
            break;
        }
        position += 1;
    }

    position
}
