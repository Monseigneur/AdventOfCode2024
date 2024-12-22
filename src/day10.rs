use std::collections::{HashSet, VecDeque};

use utilities;

use crate::day6::Point;

const DAY: usize = 10;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type TopographicMap = Vec<Vec<usize>>;

fn part_1(contents: &str) -> usize {
    let map = parse_input(contents);
    let trailheads = find_trailheads(&map);

    trailheads
        .iter()
        .map(|trailhead| calculate_trail_score(trailhead, &map, false))
        .sum()
}

fn parse_input(contents: &str) -> TopographicMap {
    contents
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn find_trailheads(map: &TopographicMap) -> Vec<Point> {
    let mut trailheads = vec![];

    for (row, row_data) in map.iter().enumerate() {
        for (col, height) in row_data.iter().enumerate() {
            if *height == 0 {
                trailheads.push(Point::new(row, col));
            }
        }
    }

    trailheads
}

fn calculate_trail_score(trailhead: &Point, map: &TopographicMap, unique_paths: bool) -> usize {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut score = 0;

    queue.push_back(*trailhead);

    while let Some(current) = queue.pop_front() {
        if !unique_paths {
            if seen.contains(&current) {
                continue;
            }

            seen.insert(current);
        }

        let current_height = map[current.row][current.col];

        if current_height == 9 {
            score += 1;

            continue;
        }

        // Find the neighbors of the next height.
        for neighbor in get_neighbors(&current, map) {
            if map[neighbor.row][neighbor.col] == current_height + 1 {
                queue.push_back(neighbor);
            }
        }
    }

    score
}

fn get_neighbors(current: &Point, map: &TopographicMap) -> Vec<Point> {
    let mut neighbors = vec![];

    if current.row > 0 {
        neighbors.push(Point::new(current.row - 1, current.col));
    }

    if current.col > 0 {
        neighbors.push(Point::new(current.row, current.col - 1));
    }

    if current.row < map.len() - 1 {
        neighbors.push(Point::new(current.row + 1, current.col));
    }

    if current.col < map[0].len() - 1 {
        neighbors.push(Point::new(current.row, current.col + 1));
    }

    neighbors
}

fn part_2(contents: &str) -> usize {
    let map = parse_input(contents);
    let trailheads = find_trailheads(&map);

    trailheads
        .iter()
        .map(|trailhead| calculate_trail_score(trailhead, &map, true))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 36);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 698);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 81);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 1436);
    }
}
