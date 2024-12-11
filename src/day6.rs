use std::collections::HashSet;

use utilities;

const DAY: usize = 6;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Grid = Vec<Vec<char>>;

fn part_1(contents: &str) -> usize {
    let (grid, starting_position) = parse_input(contents);

    let visited = walk_path(starting_position, &grid);

    visited.len()
}

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    row: usize,
    col: usize,
}

impl Point {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
}

fn parse_input(contents: &str) -> (Grid, Point) {
    let grid: Grid = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut starting_position = None;

    for (row, line) in contents.lines().enumerate() {
        for (col, c) in line.char_indices() {
            match c {
                '^' => starting_position = Some(Point::new(row, col)),
                _ => continue,
            }
        }
    }

    (grid, starting_position.unwrap())
}

fn get_next_position(current: &Point, facing: usize, grid: &Grid) -> Option<(Point, usize)> {
    let next_row = match facing {
        0 => {
            if current.row > 0 {
                Some(current.row - 1)
            } else {
                None
            }
        }
        2 => {
            if current.row < grid.len() - 1 {
                Some(current.row + 1)
            } else {
                None
            }
        }
        _ => Some(current.row),
    };

    let next_col = match facing {
        3 => {
            if current.col > 0 {
                Some(current.col - 1)
            } else {
                None
            }
        }
        1 => {
            if current.col < grid[0].len() - 1 {
                Some(current.col + 1)
            } else {
                None
            }
        }
        _ => Some(current.col),
    };

    if next_row.is_none() || next_col.is_none() {
        // We walked off the edge.
        return None;
    }

    let next_row = next_row.unwrap();
    let next_col = next_col.unwrap();

    if grid[next_row][next_col] == '#' {
        // Obstacle, need to turn.
        Some((*current, (facing + 1) % 4))
    } else {
        Some((Point::new(next_row, next_col), facing))
    }
}

fn walk_path(starting_position: Point, grid: &Grid) -> HashSet<Point> {
    let mut visited = HashSet::new();
    visited.insert(starting_position);

    let mut current = starting_position;
    let mut facing = 0;

    while let Some((next_position, next_facing)) = get_next_position(&current, facing, &grid) {
        visited.insert(next_position);

        current = next_position;
        facing = next_facing;
    }

    visited
}

fn part_2(contents: &str) -> usize {
    // Crappy brute force way
    let (mut grid, starting_position) = parse_input(contents);

    let visited = walk_path(starting_position, &grid);

    let mut count = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            // Set a temporary obstacle, but only if there wasn't something there and
            // it was actually reached on the path.
            if grid[row][col] != '.' || !visited.contains(&Point::new(row, col)) {
                continue;
            }

            grid[row][col] = '#';

            if single_walk(starting_position, &grid) {
                count += 1;
            }

            grid[row][col] = '.';
        }
    }

    count
}

fn single_walk(starting_position: Point, grid: &Grid) -> bool {
    let mut visited = HashSet::new();
    visited.insert((starting_position, 0));

    let mut current = starting_position;
    let mut facing = 0;

    while let Some((next_position, next_facing)) = get_next_position(&current, facing, &grid) {
        let new_position = (next_position, next_facing);

        if visited.contains(&new_position) {
            // Found loop.
            return true;
        }

        visited.insert(new_position);

        current = next_position;
        facing = next_facing;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 41);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 5239);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 6);
    }

    #[test]
    #[ignore]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 1753);
    }
}
