use std::collections::HashMap;

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
    let stones = parse_input(contents);
    let mut data_table: HashMap<usize, HashMap<usize, usize>> = HashMap::new();

    stones
        .iter()
        .map(|stone| process_stone(*stone, 75, &mut data_table))
        .sum()
}

fn process_stone(
    stone: usize,
    level: usize,
    data_table: &mut HashMap<usize, HashMap<usize, usize>>,
) -> usize {
    if level == 0 {
        return 1;
    }

    if let Some(data) = data_table.get(&stone) {
        if let Some(count) = data.get(&level) {
            return *count;
        }
    }

    // Not at the bottom, need to continue.
    let mut current_level = level;
    let mut count = 0;

    let mut stone_pieces = vec![stone];

    while current_level > 0 {
        if stone_pieces.is_empty() {
            break;
        }

        stone_pieces = blink_once(&stone_pieces);
        current_level -= 1;

        let mut new_stone_pieces = vec![];
        for stone in stone_pieces {
            if stone < 10 {
                let result = process_stone(stone, current_level, data_table);

                data_table
                    .entry(stone)
                    .and_modify(|data| {
                        data.insert(current_level, result);
                    })
                    .or_insert({
                        let mut data = HashMap::new();
                        data.insert(current_level, result);
                        data
                    });

                count += result;

                continue;
            }

            new_stone_pieces.push(stone);
        }

        stone_pieces = new_stone_pieces;
    }

    count + stone_pieces.len()
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
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 241394363462435);
    }
}
