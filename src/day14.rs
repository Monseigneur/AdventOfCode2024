use std::cmp::Ordering;

use utilities;

use crate::day6::Point;

const DAY: usize = 14;

const ROOM_WIDTH: usize = 101;
const ROOM_HEIGHT: usize = 103;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    part_1_with_bounds(contents, ROOM_WIDTH, ROOM_HEIGHT)
}

fn part_1_with_bounds(contents: &str, room_width: usize, room_height: usize) -> usize {
    let robots = parse_input(contents);

    let robot_positions = robots
        .iter()
        .map(|robot| {
            step_with_iterations(&robot.start, robot.velocity, room_width, room_height, 100)
        })
        .collect::<Vec<_>>();

    calculate_safety_score(&robot_positions, room_width, room_height)
}

#[derive(Debug)]
struct Robot {
    start: Point,
    velocity: (isize, isize),
}

impl Robot {
    fn new(start: Point, velocity: (isize, isize)) -> Self {
        Self { start, velocity }
    }
}

fn parse_input(contents: &str) -> Vec<Robot> {
    contents
        .lines()
        .map(|line| {
            let pieces = line
                .split_ascii_whitespace()
                .flat_map(|pieces| {
                    pieces
                        .split(&['p', 'v', '=', ','])
                        .filter_map(|val| val.parse::<isize>().ok())
                })
                .collect::<Vec<_>>();

            // X maps to column, and Y maps to row.
            let start = Point::new(pieces[1] as usize, pieces[0] as usize);
            let velocity = (pieces[3], pieces[2]);

            Robot::new(start, velocity)
        })
        .collect()
}

fn adjust_value(value: isize, max: usize) -> usize {
    if value >= 0 {
        let value = value as usize;

        value % max
    } else {
        let rem = value.abs() as usize % max;

        if rem != 0 {
            max - rem
        } else {
            rem
        }
    }
}

fn calculate_safety_score(
    robot_positions: &Vec<Point>,
    room_width: usize,
    room_height: usize,
) -> usize {
    let center_vertical = room_width / 2;
    let center_horizontal = room_height / 2;

    let mut upper_left_count = 0;
    let mut upper_right_count = 0;
    let mut lower_left_count = 0;
    let mut lower_right_count = 0;

    for robot_position in robot_positions {
        match (
            robot_position.row.cmp(&center_horizontal),
            robot_position.col.cmp(&center_vertical),
        ) {
            (Ordering::Less, Ordering::Less) => upper_left_count += 1,
            (Ordering::Less, Ordering::Greater) => upper_right_count += 1,
            (Ordering::Greater, Ordering::Less) => lower_left_count += 1,
            (Ordering::Greater, Ordering::Greater) => lower_right_count += 1,
            _ => {}
        };
    }

    upper_left_count * upper_right_count * lower_left_count * lower_right_count
}

fn part_2(contents: &str) -> usize {
    let robots = parse_input(contents);

    let mut positions = vec![];
    let mut velocities = vec![];

    for robot in robots {
        positions.push(robot.start);
        velocities.push(robot.velocity);
    }

    let initial_positions = positions.clone();

    let mut iteration = 0;
    let mut safety_scores = vec![];

    loop {
        if positions == initial_positions && iteration != 0 {
            break;
        }

        let score = calculate_safety_score(&positions, ROOM_WIDTH, ROOM_HEIGHT);
        safety_scores.push(score);

        for i in 0..positions.len() {
            positions[i] =
                step_with_iterations(&positions[i], velocities[i], ROOM_WIDTH, ROOM_HEIGHT, 1);
        }

        iteration += 1;
    }

    let (idx, _) = safety_scores
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    idx
}

fn step_with_iterations(
    start: &Point,
    velocity: (isize, isize),
    room_width: usize,
    room_height: usize,
    iterations: usize,
) -> Point {
    let final_row = start.row as isize + iterations as isize * velocity.0;
    let final_col = start.col as isize + iterations as isize * velocity.1;

    let final_row = adjust_value(final_row, room_height);
    let final_col = adjust_value(final_col, room_width);

    Point::new(final_row, final_col)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1_with_bounds(&contents, 11, 7), 12);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 226179492);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 7502);
    }
}
