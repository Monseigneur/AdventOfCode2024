const DAY: usize = 22;

const ITERATIONS: usize = 2000;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let initial_values = parse_input(contents);

    initial_values
        .iter()
        .map(|val| calculate_secret_value(*val, ITERATIONS))
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
    let val = (val ^ (val << 6)) & 0x00FFFFFF;
    let val = (val ^ (val >> 5)) & 0x00FFFFFF;
    (val ^ (val << 11)) & 0x00FFFFFF
}

fn part_2(contents: &str) -> usize {
    let initial_values = parse_input(contents);

    let prices = initial_values
        .iter()
        .map(|iv| build_prices(*iv, ITERATIONS + 1))
        .collect::<Vec<_>>();
    let deltas = prices
        .iter()
        .map(|price_table| build_deltas(price_table))
        .collect::<Vec<_>>();

    let mut results = vec![0; 19 * 19 * 19 * 19];

    for (idx, delta_table) in deltas.iter().enumerate() {
        let mut seen_patterns = vec![false; 19 * 19 * 19 * 19];

        for (i, pattern) in delta_table.windows(4).enumerate() {
            let pattern_code = convert_pattern(pattern);

            if seen_patterns[pattern_code] {
                continue;
            }

            seen_patterns[pattern_code] = true;

            let price = prices[idx][i + pattern.len()];

            results[pattern_code] += price;
        }
    }

    *results.iter().max().unwrap()
}

fn build_prices(initial_value: usize, length: usize) -> Vec<usize> {
    let mut prices = Vec::with_capacity(length);
    let mut val = initial_value;

    for _ in 0..prices.capacity() {
        prices.push(val % 10);
        val = get_next_value(val);
    }

    prices
}

fn build_deltas(prices: &[usize]) -> Vec<isize> {
    let mut delta_table = Vec::with_capacity(prices.len() - 1);

    for window in prices.windows(2) {
        delta_table.push(window[1] as isize - window[0] as isize);
    }

    delta_table
}

fn convert_pattern(pattern: &[isize]) -> usize {
    let mut value = 0;

    for delta in pattern {
        // Prices are 0-9, so deltas are -9 to 9
        value *= 19;
        value += (delta + 9) as usize;
    }

    value
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
    fn test_example2_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 23);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 2018);
    }
}
