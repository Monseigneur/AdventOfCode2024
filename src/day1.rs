use std::collections::HashMap;

use utilities;

const DAY: usize = 1;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let mut left = vec![];
    let mut right = vec![];

    for line in contents.lines() {
        let pieces: Vec<&str>= line.split_ascii_whitespace().collect();

        left.push(pieces[0].parse::<usize>().unwrap());
        right.push(pieces[1].parse::<usize>().unwrap());
    }

    left.sort();
    right.sort();

    let mut total = 0;

    for i in 0..left.len() {
        let delta = left[i].abs_diff(right[i]);

        total += delta;
    }

    total
}

fn part_2(contents: &str) -> usize {
    let mut left = vec![];
    let mut right: HashMap<usize, usize> = HashMap::new();

    for line in contents.lines() {
        let pieces: Vec<&str> = line.split_ascii_whitespace().collect();

        left.push(pieces[0].parse::<usize>().unwrap());

        let right_val = pieces[1].parse::<usize>().unwrap();

        right.entry(right_val).and_modify(|rv| *rv += 1).or_insert(1);
    }

    let mut simularity_score = 0;

    for left_val in left {
        let right_count = right.get(&left_val).unwrap_or(&0);

        simularity_score += left_val * right_count;
    }

    simularity_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 11);
    }

    #[test]
    fn test_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 31);
    }
}