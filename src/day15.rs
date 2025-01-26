use std::{
    collections::{HashSet, VecDeque},
    ops::Range,
};

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
                if *c == 'O' || *c == '[' {
                    Some(Point::new(row, col))
                } else {
                    None
                }
            })
        })
        .collect()
}

fn part_2(contents: &str) -> usize {
    let (grid, moves) = parse_input(contents);

    let grid = expand_grid(grid);

    let grid = apply_moves_v2(grid, &moves);
    let boxes = find_boxes(&grid);

    boxes.iter().map(|point| 100 * point.row + point.col).sum()
}

fn expand_grid(grid: Grid) -> Grid {
    grid.iter()
        .map(|row| {
            row.iter()
                .flat_map(|c| match c {
                    'O' => ['[', ']'],
                    '@' => ['@', '.'],
                    _ => [*c, *c],
                })
                .collect()
        })
        .collect()
}

fn apply_moves_v2(start_grid: Grid, moves: &Vec<char>) -> Grid {
    let mut grid = start_grid;

    let mut robot_position = find_robot(&grid);
    grid[robot_position.row][robot_position.col] = '.';

    for robot_move in moves {
        robot_position = move_robot_v2(&robot_position, *robot_move, &mut grid);
    }

    grid
}

fn get_point_in_direction(position: &Point, direction: char, grid: &Grid) -> Option<Point> {
    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;

    match direction {
        '^' if position.row > 0 => Some(Point::new(position.row - 1, position.col)),
        '>' if position.col < last_col => Some(Point::new(position.row, position.col + 1)),
        'v' if position.row < last_row => Some(Point::new(position.row + 1, position.col)),
        '<' if position.col > 0 => Some(Point::new(position.row, position.col - 1)),
        _ => None,
    }
}

fn move_robot_v2(position: &Point, direction: char, grid: &mut Grid) -> Point {
    let Some(next_point) = get_point_in_direction(position, direction, grid) else {
        return *position;
    };

    let next_position_item = grid[next_point.row][next_point.col];

    // If there is a free space or a wall, then already know what to do.
    if next_position_item == '.' {
        return next_point;
    } else if next_position_item == '#' {
        return *position;
    }

    // If it's a box, try to move it and any other boxes on the other side of it.
    let box_position = match next_position_item {
        '[' => next_point,
        ']' => Point::new(next_point.row, next_point.col - 1),
        _ => unreachable!(),
    };

    if let Some(boxes) = find_boxes_to_move(grid, &box_position, direction) {
        move_boxes(grid, direction, &boxes);

        next_point
    } else {
        *position
    }
}

fn find_boxes_to_move(
    grid: &Grid,
    box_position: &Point,
    direction: char,
) -> Option<HashSet<Point>> {
    // Ran into a box, try to move them.
    let mut boxes = HashSet::new();

    let mut box_queue = VecDeque::new();
    box_queue.push_back(*box_position);

    while let Some(box_position) = box_queue.pop_front() {
        if !boxes.insert(box_position) {
            continue;
        }

        let next_position = match direction {
            '^' | 'v' | '<' => get_point_in_direction(&box_position, direction, grid),
            '>' => get_point_in_direction(
                &Point::new(box_position.row, box_position.col + 1),
                direction,
                grid,
            ),
            _ => unreachable!(),
        };

        let Some(next_position) = next_position else {
            // Off the edge, can't push the boxes.
            return None;
        };

        match grid[next_position.row][next_position.col] {
            '#' => return None,
            '[' => box_queue.push_back(next_position),
            ']' => box_queue.push_back(Point::new(next_position.row, next_position.col - 1)),
            _ => {}
        }

        if direction == '^' || direction == 'v' {
            // For up and down, need to also consider the point above or below the right side of the box.
            let other_next_position = Point::new(next_position.row, next_position.col + 1);

            match grid[other_next_position.row][other_next_position.col] {
                '#' => return None,
                '[' => box_queue.push_back(other_next_position),
                _ => {}
            }
        }
    }

    Some(boxes)
}

fn move_boxes(grid: &mut Grid, direction: char, boxes: &HashSet<Point>) {
    let (row_offset, col_offset) = match direction {
        '^' => (-1, 0),
        '>' => (0, 1),
        'v' => (1, 0),
        '<' => (0, -1),
        _ => unreachable!(),
    };

    // First clear out old boxes, and then fill in the new positions.
    for box_point in boxes.iter() {
        grid[box_point.row][box_point.col] = '.';
        grid[box_point.row][box_point.col + 1] = '.';
    }

    for box_point in boxes.iter() {
        let new_row = (box_point.row as isize + row_offset) as usize;
        let new_col = (box_point.col as isize + col_offset) as usize;

        grid[new_row][new_col] = '[';
        grid[new_row][new_col + 1] = ']';
    }
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

        assert_eq!(part_2(&contents), 9021);
    }

    #[test]
    fn test_example3_part_2() {
        let contents = utilities::read_file_data(DAY, "example3.txt");

        assert_eq!(part_2(&contents), 618);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 1524905);
    }
}
