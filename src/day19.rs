use std::collections::HashMap;

use utilities;

const DAY: usize = 19;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let (towels, patterns) = parse_input(contents);

    patterns
        .iter()
        .filter(|pattern| check_towel_pattern(pattern, &towels))
        .count()
}

fn parse_input(contents: &str) -> (HashMap<char, Vec<String>>, Vec<String>) {
    let mut iter = contents.lines();

    let mut towels: HashMap<char, Vec<String>> = HashMap::new();

    for towel in iter.next().unwrap().split(",").map(|s| s.trim()) {
        let c = towel.chars().next().unwrap();
        let towel = towel.to_string();
        towels
            .entry(c)
            .and_modify(|v| {
                v.push(towel.clone());
            })
            .or_insert(vec![towel]);
    }

    let patterns = iter.filter(|s| !s.is_empty()).map(|s| s.into()).collect();

    (towels, patterns)
}

fn check_towel_pattern(pattern: &String, towels: &HashMap<char, Vec<String>>) -> bool {
    check_towel_helper(pattern, towels)
}

fn check_towel_helper(pattern: &str, towels: &HashMap<char, Vec<String>>) -> bool {
    if pattern.is_empty() {
        return true;
    }

    let first_char = pattern.chars().next().unwrap();

    let Some(possible_patterns) = towels.get(&first_char) else {
        return false;
    };

    for possible_pattern in possible_patterns {
        if pattern.starts_with(possible_pattern) {
            if check_towel_helper(&pattern[possible_pattern.len()..], towels) {
                return true;
            }
        }
    }

    false
}

fn part_2(contents: &str) -> usize {
    let (towels, patterns) = parse_input(contents);

    patterns
        .iter()
        .map(|pattern| check_towel_pattern_v2(pattern, &towels))
        .sum()
}

fn check_towel_pattern_v2<'a>(pattern: &'a String, towels: &HashMap<char, Vec<String>>) -> usize {
    let mut memo_table: HashMap<&'a str, usize> = HashMap::new();

    check_towel_helper_v2(pattern, towels, &mut memo_table)
}

fn check_towel_helper_v2<'a>(
    pattern: &'a str,
    towels: &HashMap<char, Vec<String>>,
    memo_table: &mut HashMap<&'a str, usize>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(count) = memo_table.get(pattern) {
        return *count;
    }

    let first_char = pattern.chars().next().unwrap();

    let Some(possible_patterns) = towels.get(&first_char) else {
        return 0;
    };

    let mut count = 0;
    for possible_pattern in possible_patterns {
        if pattern.starts_with(possible_pattern) {
            count += check_towel_helper_v2(&pattern[possible_pattern.len()..], towels, memo_table);
        }
    }

    memo_table.insert(pattern, count);

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 6);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 228);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 16);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 584553405070389);
    }
}
