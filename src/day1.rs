use std::collections::HashMap;

const DAY: usize = 1;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let mut left = vec![];
    let mut right = vec![];

    for line in contents.lines() {
        let pieces: Vec<&str> = line.split_ascii_whitespace().collect();

        left.push(pieces[0].parse::<usize>().unwrap());
        right.push(pieces[1].parse::<usize>().unwrap());
    }

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .fold(0, |acc, (left_val, right_val)| {
            acc + left_val.abs_diff(right_val)
        })
}

fn part_2(contents: &str) -> usize {
    let mut left = vec![];
    let mut right: HashMap<usize, usize> = HashMap::new();

    for line in contents.lines() {
        let pieces: Vec<&str> = line.split_ascii_whitespace().collect();

        left.push(pieces[0].parse::<usize>().unwrap());

        let right_val = pieces[1].parse::<usize>().unwrap();

        right
            .entry(right_val)
            .and_modify(|rv| *rv += 1)
            .or_insert(1);
    }

    left.into_iter()
        .fold(0, |acc, val| acc + val * right.get(&val).unwrap_or(&0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 11);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 2176849);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 31);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 23384288);
    }
}
