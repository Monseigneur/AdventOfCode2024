use std::collections::{HashMap, HashSet};

use utilities;

use crate::day6::Point;

const DAY: usize = 8;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Grid = Vec<Vec<char>>;
type Antennas = HashMap<char, Vec<Point>>;

fn part_1(contents: &str) -> usize {
    let grid = parse_input(contents);

    let antennas = find_antennas(&grid);

    find_antinodes(&grid, &antennas).len()
}

fn parse_input(contents: &str) -> Grid {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_antennas(grid: &Grid) -> Antennas {
    let mut antennas: Antennas = HashMap::new();

    for (row, row_data) in grid.iter().enumerate() {
        for (col, c) in row_data.iter().enumerate() {
            if c == &'.' {
                continue;
            }

            // This spot contains an antenna.
            let position = Point::new(row, col);

            antennas
                .entry(*c)
                .and_modify(|positions| positions.push(position))
                .or_insert(vec![position]);
        }
    }

    antennas
}

fn find_antinodes(grid: &Grid, antennas: &Antennas) -> HashSet<Point> {
    let mut antinodes = HashSet::new();

    for (antenna, positions) in antennas {
        // Find all pairs of positions.
        for i in 0..(positions.len() - 1) {
            for j in (i + 1)..positions.len() {
                let pairs = find_antinode_pairs(&positions[i], &positions[j], grid);

                antinodes.extend(pairs);
            }
        }
    }

    antinodes
}

fn find_antinode_pairs(position1: &Point, position2: &Point, grid: &Grid) -> Vec<Point> {
    let mut antinodes = vec![];

    let cross_pairs = position1.row.cmp(&position2.row) != position1.col.cmp(&position2.col);

    let delta_r = position1.row.abs_diff(position2.row);
    let delta_c = position1.col.abs_diff(position2.col);

    let (smallest_r, largest_r) = min_max(position1.row, position2.row);
    let (smallest_c, largest_c) = min_max(position1.col, position2.col);

    let smallest_r = if smallest_r < delta_r {
        None
    } else {
        Some(smallest_r - delta_r)
    };

    let largest_r = if largest_r + delta_r >= grid.len() {
        None
    } else {
        Some(largest_r + delta_r)
    };

    let smallest_c = if smallest_c < delta_c {
        None
    } else {
        Some(smallest_c - delta_c)
    };

    let largest_c = if largest_c + delta_c >= grid[0].len() {
        None
    } else {
        Some(largest_c + delta_c)
    };

    if cross_pairs {
        if smallest_r.is_some() && largest_c.is_some() {
            antinodes.push(Point::new(smallest_r.unwrap(), largest_c.unwrap()));
        }

        if largest_r.is_some() && smallest_c.is_some() {
            antinodes.push(Point::new(largest_r.unwrap(), smallest_c.unwrap()));
        }
    } else {
        if smallest_r.is_some() && smallest_c.is_some() {
            antinodes.push(Point::new(smallest_r.unwrap(), smallest_c.unwrap()));
        }

        if largest_r.is_some() && largest_c.is_some() {
            antinodes.push(Point::new(largest_r.unwrap(), largest_c.unwrap()));
        }
    }

    antinodes
}

fn min_max(val1: usize, val2: usize) -> (usize, usize) {
    if val1 < val2 {
        (val1, val2)
    } else {
        (val2, val1)
    }
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

        assert_eq!(part_1(&contents), 14);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 222);
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
