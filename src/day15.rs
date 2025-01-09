use std::ops::Range;

use crate::day6::Point;

const DAY: usize = 15;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let (grid, moves) = parse_input(contents);

    let grid = apply_moves(grid, &moves);
    let boxes = find_boxes(&grid);

    boxes.iter().map(|point| 100 * point.row + point.col).sum()
}

type Grid = Vec<Vec<char>>;

fn parse_input(contents: &str) -> (Grid, Vec<char>) {
    let mut grid = vec![];
    let mut moves = vec![];

    let mut grid_done = false;

    for line in contents.lines() {
        if line.is_empty() {
            grid_done = true;
            continue;
        }

        let line_data = line.chars().collect::<Vec<_>>();

        if !grid_done {
            grid.push(line_data);
        } else {
            moves.push(line_data);
        }
    }

    let moves = moves.into_iter().flatten().collect::<Vec<_>>();

    (grid, moves)
}

fn apply_moves(start_grid: Grid, moves: &Vec<char>) -> Grid {
    let mut grid = start_grid;

    let mut robot_position = find_robot(&grid);
    grid[robot_position.row][robot_position.col] = '.';

    for robot_move in moves {
        robot_position = move_robot(&robot_position, *robot_move, &mut grid);
    }

    grid
}

fn find_robot(grid: &Grid) -> Point {
    for (row, row_data) in grid.iter().enumerate() {
        for (col, c) in row_data.iter().enumerate() {
            if *c == '@' {
                return Point::new(row, col);
            }
        }
    }

    panic!("Couldn't find robot starting position!");
}

fn move_robot(position: &Point, direction: char, grid: &mut Grid) -> Point {
    let next_points = get_points_in_direction(position, direction, grid);

    if next_points.is_empty() {
        return *position;
    }

    let box_count = next_points
        .iter()
        .filter(|point| grid[point.row][point.col] == 'O')
        .count();

    if box_count == next_points.len() {
        return *position;
    }

    // There is space, so try to push the boxes ahead if needed.
    let next_point = next_points[0];

    if grid[next_point.row][next_point.col] != 'O' {
        return next_point;
    }

    grid[next_point.row][next_point.col] = '.';

    for point in next_points.iter().skip(1) {
        if grid[point.row][point.col] == '.' {
            grid[point.row][point.col] = 'O';
            break;
        }
    }

    next_point
}

fn get_points_in_direction(position: &Point, direction: char, grid: &Grid) -> Vec<Point> {
    let mut row_rev = false;
    let mut col_rev = false;

    let (row_range, col_range) = match direction {
        '^' => {
            row_rev = true;
            (0..position.row, position.col..(position.col + 1))
        }
        '>' => (
            position.row..(position.row + 1),
            (position.col + 1)..grid[0].len(),
        ),
        'v' => (
            (position.row + 1)..grid.len(),
            position.col..(position.col + 1),
        ),
        '<' => {
            col_rev = true;
            (position.row..(position.row + 1), (0..position.col))
        }
        _ => panic!("Illegal robot move!"),
    };

    if row_range.is_empty() || col_range.is_empty() {
        return vec![];
    }

    let mut points = vec![];

    for point in get_points_iter(row_range, col_range, row_rev, col_rev) {
        if grid[point.row][point.col] == '#' {
            break;
        }

        points.push(point);
    }

    points
}

fn get_points_iter(
    row_range: Range<usize>,
    col_range: Range<usize>,
    row_rev: bool,
    col_rev: bool,
) -> impl Iterator<Item = Point> {
    let use_row_iter = row_range.len() != 1;
    let (rev_iter, other_constant) = if use_row_iter {
        (row_rev, col_range.start)
    } else {
        (col_rev, row_range.start)
    };

    let iter: Box<dyn Iterator<Item = usize>> = if use_row_iter {
        if rev_iter {
            Box::new(row_range.into_iter().rev())
        } else {
            Box::new(row_range.into_iter())
        }
    } else if rev_iter {
        Box::new(col_range.into_iter().rev())
    } else {
        Box::new(col_range.into_iter())
    };

    iter.map(move |val| {
        if use_row_iter {
            Point::new(val, other_constant)
        } else {
            Point::new(other_constant, val)
        }
    })
}

fn find_boxes(grid: &Grid) -> Vec<Point> {
    grid.iter()
        .enumerate()
        .flat_map(|(row, row_data)| {
            row_data.iter().enumerate().filter_map(move |(col, c)| {
                if *c == 'O' {
                    Some(Point::new(row, col))
                } else {
                    None
                }
            })
        })
        .collect()
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

        assert_eq!(part_1(&contents), 10092);
    }

    #[test]
    fn test_example2_part_1() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_1(&contents), 2028);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1495147);
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
