use utilities;

const DAY: usize = 11;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let mut stones = parse_input(contents);

    for _ in 0..25 {
        stones = blink_once(&stones);
    }

    stones.len()
}

fn parse_input(contents: &str) -> Vec<usize> {
    contents
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect()
}

fn blink_once(stones: &Vec<usize>) -> Vec<usize> {
    let mut new_stones = vec![];

    for stone in stones {
        if stone == &0 {
            new_stones.push(1);
            continue;
        }

        let digits = stone.ilog10() + 1;

        if digits % 2 == 0 {
            let factor = 10_usize.pow(digits / 2);

            new_stones.push(stone / factor);
            new_stones.push(stone % factor);

            continue;
        }

        new_stones.push(stone * 2024);
    }

    new_stones
}

fn part_2(contents: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 55312);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 203457);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 0);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 0);
    }
}
