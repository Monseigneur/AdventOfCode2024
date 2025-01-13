use std::collections::HashSet;

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

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn new(row: usize, col: usize) -> Self {
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

    let (Some(next_row), Some(next_col)) = (next_row, next_col) else {
        return None;
    };

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

    while let Some((next_position, next_facing)) = get_next_position(&current, facing, grid) {
        visited.insert(next_position);

        current = next_position;
        facing = next_facing;
    }

    visited
}

fn part_2(contents: &str) -> usize {
    let (mut grid, starting_position) = parse_input(contents);

    walk_path_v2(starting_position, &mut grid)
}

fn walk_path_v2(starting_position: Point, grid: &mut Grid) -> usize {
    let mut current = starting_position;
    let mut facing = 0;

    let mut visited = HashSet::new();

    let mut count = 0;

    while let Some((next_position, next_facing)) = get_next_position(&current, facing, grid) {
        // If the facing is the same way, pretend first we had a block there.
        if next_facing == facing && !visited.contains(&next_position) {
            grid[next_position.row][next_position.col] = '#';

            if single_walk(current, facing, grid) {
                count += 1;
            }

            grid[next_position.row][next_position.col] = '.';
        }

        visited.insert(next_position);

        current = next_position;
        facing = next_facing;
    }

    count
}

fn single_walk(starting_position: Point, starting_facing: usize, grid: &Grid) -> bool {
    // Use the tortoise and hare cycle detection algorithm.
    let mut current_hare = starting_position;
    let mut facing_hare = starting_facing;

    let mut current_tortoise = starting_position;
    let mut facing_tortoise = starting_facing;

    loop {
        let Some((next_position_hare, next_facing_hare)) =
            get_next_position(&current_hare, facing_hare, grid)
        else {
            return false;
        };

        current_hare = next_position_hare;
        facing_hare = next_facing_hare;

        let Some((next_position_hare, next_facing_hare)) =
            get_next_position(&current_hare, facing_hare, grid)
        else {
            return false;
        };

        current_hare = next_position_hare;
        facing_hare = next_facing_hare;

        // The tortoise can be advanced as well, and it should always be valid (since the hare already went past this point).
        let Some((next_position_tortoise, next_facing_tortoise)) =
            get_next_position(&current_tortoise, facing_tortoise, grid)
        else {
            panic!("Tortoise shouldn't have fallen off the grid!");
        };

        current_tortoise = next_position_tortoise;
        facing_tortoise = next_facing_tortoise;

        if current_hare == current_tortoise && facing_hare == facing_tortoise {
            return true;
        }
    }
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
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 1753);
    }
}
