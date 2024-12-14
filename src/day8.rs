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

    find_antinodes(&grid, &antennas, false)
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

fn find_antinodes(grid: &Grid, antennas: &Antennas, resonant: bool) -> usize {
    let mut antinodes = HashSet::new();

    for (_, positions) in antennas {
        // Find all pairs of positions.
        for i in 0..(positions.len() - 1) {
            for j in (i + 1)..positions.len() {
                let pairs = find_antinode_pairs(&positions[i], &positions[j], grid, resonant);

                antinodes.extend(pairs);
            }
        }
    }

    antinodes.len()
}

fn find_antinode_pairs(
    position1: &Point,
    position2: &Point,
    grid: &Grid,
    resonant: bool,
) -> Vec<Point> {
    let mut antinodes = vec![];

    let p1_row = position1.row as isize;
    let p1_col = position1.col as isize;

    let p2_row = position2.row as isize;
    let p2_col = position2.col as isize;

    let delta_r = p2_row - p1_row;
    let delta_c = p2_col - p1_col;

    // Walk "negative" from position1.
    let mut antinode_r = p1_row;
    let mut antinode_c = p1_col;

    loop {
        antinode_r -= delta_r;
        antinode_c -= delta_c;

        if check_position(antinode_r, antinode_c, grid) {
            antinodes.push(Point::new(antinode_r as usize, antinode_c as usize));
        } else {
            break;
        }

        if !resonant {
            break;
        }
    }

    // Walk "positive" from position2.
    let mut antinode_r = p2_row;
    let mut antinode_c = p2_col;

    loop {
        antinode_r += delta_r;
        antinode_c += delta_c;

        if check_position(antinode_r, antinode_c, grid) {
            antinodes.push(Point::new(antinode_r as usize, antinode_c as usize));
        } else {
            break;
        }

        if !resonant {
            break;
        }
    }

    // If resonant, include the two antennas.
    if resonant {
        antinodes.push(*position1);
        antinodes.push(*position2);
    }

    antinodes
}

fn check_position(row: isize, col: isize, grid: &Grid) -> bool {
    let row_valid = row >= 0 && row < grid.len() as isize;
    let col_valid = col >= 0 && col < grid[0].len() as isize;

    row_valid && col_valid
}

fn part_2(contents: &str) -> usize {
    let grid = parse_input(contents);

    let antennas = find_antennas(&grid);

    find_antinodes(&grid, &antennas, true)
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

        assert_eq!(part_2(&contents), 34);
    }

    #[test]
    fn test_example2_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 9);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 884);
    }
}
