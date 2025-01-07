use utilities;

const DAY: usize = 22;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let initial_values = parse_input(contents);

    initial_values
        .iter()
        .map(|val| calculate_secret_value(*val, 2000))
        .sum()
}

fn parse_input(contents: &str) -> Vec<usize> {
    contents
        .lines()
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn calculate_secret_value(initial: usize, iterations: usize) -> usize {
    let mut val = initial;

    for _ in 0..iterations {
        val = get_next_value(val);
    }

    val
}

fn get_next_value(val: usize) -> usize {
    let val = (val ^ (val * 64)) % 16777216;
    let val = (val ^ (val / 32)) % 16777216;
    let val = (val ^ (val * 2048)) % 16777216;

    val
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

        assert_eq!(part_1(&contents), 37327623);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 18317943467);
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
