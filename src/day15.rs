use utilities;

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
        return position.clone();
    }

    let box_count = next_points
        .iter()
        .filter(|point| grid[point.row][point.col] == 'O')
        .count();

    if box_count == next_points.len() {
        return position.clone();
    }

    // There is space, so try to push the boxes ahead if needed.
    let next_point = next_points[0];

    if grid[next_point.row][next_point.col] != 'O' {
        return next_point;
    }

    grid[next_point.row][next_point.col] = '.';

    for i in 1..next_points.len() {
        let point = &next_points[i];
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

    // Not sure how to make this better.
    if row_range.len() == 1 {
        let r = row_range.start;
        if col_rev {
            for c in col_range.into_iter().rev() {
                if grid[r][c] == '#' {
                    break;
                }

                points.push(Point::new(r, c));
            }
        } else {
            for c in col_range.into_iter() {
                if grid[r][c] == '#' {
                    break;
                }

                points.push(Point::new(r, c));
            }
        }
    } else {
        let c = col_range.start;
        if row_rev {
            for r in row_range.into_iter().rev() {
                if grid[r][c] == '#' {
                    break;
                }

                points.push(Point::new(r, c));
            }
        } else {
            for r in row_range.into_iter() {
                if grid[r][c] == '#' {
                    break;
                }

                points.push(Point::new(r, c));
            }
        }
    }

    points
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
