use std::collections::HashMap;

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

    let mut stone_map: HashMap<usize, usize> = HashMap::new();
    for stone in stones {
        update_map(&mut stone_map, stone, 1);
    }

    for _ in 0..75 {
        let mut new_stone_map: HashMap<usize, usize> = HashMap::new();

        for (stone, count) in stone_map {
            let new_stones = blink_once(&vec![stone]);

            for new_stone in new_stones {
                update_map(&mut new_stone_map, new_stone, count);
            }
        }

        stone_map = new_stone_map;
    }

    stone_map.values().sum()
}

fn update_map(map: &mut HashMap<usize, usize>, stone: usize, count: usize) {
    map.entry(stone)
        .and_modify(|val| *val += count)
        .or_insert(count);
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
