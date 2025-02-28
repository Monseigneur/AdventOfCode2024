const DAY: usize = 20;

use std::collections::HashMap;

use crate::day10::get_neighbors;
use crate::day6::Point;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let (grid, start, end) = parse_input(contents);
    let distance_table = build_distance_table(&grid, &start, &end);

    count_cheats(&grid, &distance_table, 100)
}

type Grid = Vec<Vec<char>>;

fn parse_input(contents: &str) -> (Grid, Point, Point) {
    let grid: Grid = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut start = None;
    let mut end = None;

    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.char_indices() {
            match c {
                'S' => start = Some(Point::new(row, col)),
                'E' => end = Some(Point::new(row, col)),
                _ => continue,
            }
        }
    }

    (grid, start.unwrap(), end.unwrap())
}

fn build_distance_table(grid: &Grid, start: &Point, end: &Point) -> Vec<Point> {
    let mut distance_table = vec![];

    let mut current = Some(*end);
    let mut last = None;

    while let Some(point) = current {
        distance_table.push(point);

        if point == *start {
            break;
        }

        for neighbor in get_neighbors(&point, grid) {
            if grid[neighbor.row][neighbor.col] != '#' && last.is_none_or(|p| neighbor != p) {
                last = current;
                current = Some(neighbor);
                break;
            }
        }
    }

    distance_table
}

fn count_cheats(grid: &Grid, distance_table: &[Point], threshold: usize) -> usize {
    let reverse_distance_table: HashMap<Point, usize> = HashMap::from_iter(
        distance_table
            .iter()
            .enumerate()
            .map(|(idx, point)| (*point, idx)),
    );

    let mut cheats: HashMap<usize, Vec<Point>> = HashMap::new();

    for point in distance_table.iter().rev() {
        let mut end_points = vec![];

        if point.row >= 2 {
            end_points.push(Point::new(point.row - 2, point.col));
        }

        if point.row < grid.len() - 2 {
            end_points.push(Point::new(point.row + 2, point.col));
        }

        if point.col >= 2 {
            end_points.push(Point::new(point.row, point.col - 2));
        }

        if point.col < grid[0].len() - 2 {
            end_points.push(Point::new(point.row, point.col + 2));
        }

        for end in end_points {
            if let Some(speedup) =
                calculate_cheat_speedup(grid, &reverse_distance_table, point, &end)
            {
                cheats
                    .entry(speedup)
                    .and_modify(|v| v.push(end))
                    .or_insert(vec![end]);
            }
        }
    }

    cheats
        .iter()
        .filter(|(&k, _)| k >= threshold)
        .map(|(_, v)| v.len())
        .sum()
}

fn calculate_cheat_speedup(
    grid: &Grid,
    reverse_distance_table: &HashMap<Point, usize>,
    start: &Point,
    end: &Point,
) -> Option<usize> {
    let intermediate = Point::new((start.row + end.row) / 2, (start.col + end.col) / 2);

    if grid[intermediate.row][intermediate.col] != '#' || grid[end.row][end.col] == '#' {
        return None;
    }

    let start_score = reverse_distance_table[start];
    let end_score = reverse_distance_table[end];

    if start_score > end_score {
        Some(start_score - end_score - 2)
    } else {
        None
    }
}

fn part_2(contents: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1381);
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
