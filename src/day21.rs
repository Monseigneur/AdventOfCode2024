use std::collections::HashMap;

use crate::day6::Point;

const DAY: usize = 21;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let codes = parse_input(contents);

    let (numeric_keypad, robot_keypad) = build_keypad_maps();

    codes
        .iter()
        .map(|code| {
            let input_length = find_shortest_input(code, &numeric_keypad, &robot_keypad, 3);
            calculate_complexity_code(code, input_length)
        })
        .sum()
}

fn parse_input(contents: &str) -> Vec<Vec<char>> {
    contents.lines().map(|s| s.chars().collect()).collect()
}

type Keypad = HashMap<char, Point>;

fn build_keypad_maps() -> (Keypad, Keypad) {
    let numeric_keypad = build_button_map("789\n456\n123\n_0A");
    let robot_keypad = build_button_map("_^A\n<v>");

    (numeric_keypad, robot_keypad)
}

fn build_button_map(keypad_layout: &str) -> Keypad {
    let mut button_map = HashMap::new();
    for (row, row_data) in keypad_layout.lines().enumerate() {
        for (col, c) in row_data.char_indices() {
            button_map.insert(c, Point::new(row, col));
        }
    }

    button_map
}

fn find_shortest_input(code: &[char], numeric_keypad: &Keypad, robot_keypad: &Keypad, num_robots: usize) -> usize {
    let mut path = process_keypad(code, &numeric_keypad);

    for _ in 0..(num_robots - 1) {
        path = process_keypad(&path, &robot_keypad);
    }

    path.len()
}

fn process_keypad(input_path: &[char], keypad: &Keypad) -> Vec<char> {
    let mut new_path = vec![];

    for i in 0..input_path.len() {
        let first_key = if i == 0  { 'A' } else { input_path[i - 1] };
        let second_key = input_path[i];

        let path = find_path(first_key, second_key, &keypad);

        new_path.extend(path.iter());
        new_path.push('A');
    }

    new_path
}

fn find_path(start: char, end: char, keypad: &Keypad) -> Vec<char> {
    let start_point = keypad[&start];
    let end_point = keypad[&end];
    let dead_spot = keypad[&'_'];

    let (delta_v, delta_h) = manhattan_distance(&start_point, &end_point);

    let mut path = vec![];

    if delta_v == 0 || delta_h == 0 {
        // only moving vertically or horizontally
        fill_path(&mut path, delta_h, true);
        fill_path(&mut path, delta_v, false);

        return path;
    }

    // Moving two directions, check if the dead spot is one of the corners.
    let vert_corner_row = (start_point.row as isize + delta_v) as usize;
    let horz_corner_col = (start_point.col as isize + delta_h) as usize;

    let vert_corner_dead = Point::new(vert_corner_row, start_point.col) == dead_spot;
    let horz_corner_dead = Point::new(start_point.row, horz_corner_col) == dead_spot;

    if horz_corner_dead {
        // go vertical first
        fill_path(&mut path, delta_v, false);
        fill_path(&mut path, delta_h, true);

        return path;
    } else if vert_corner_dead {
        // go horizontal first
        fill_path(&mut path, delta_h, true);
        fill_path(&mut path, delta_v, false);

        return path;
    }

    // Both paths are viable.
    if delta_h < 0 {
        // go left first
        fill_path(&mut path, delta_h, true);
        fill_path(&mut path, delta_v, false);
    } else if delta_h.abs() > delta_v.abs() {
        // bigger delta_h, go horizontal first
        fill_path(&mut path, delta_h, true);
        fill_path(&mut path, delta_v, false);
    } else if delta_h.abs() < delta_v.abs() {
        // bigger delta_v, go vertical first
        fill_path(&mut path, delta_v, false);
        fill_path(&mut path, delta_h, true);
    } else {
        // if the same magnitude, go vertical first
        fill_path(&mut path, delta_v, false);
        fill_path(&mut path, delta_h, true);
    }

    path
}

fn fill_path(current_path: &mut Vec<char>, amount: isize, is_horizontal: bool)
{
    let c = match (amount > 0, is_horizontal) {
        (true, true) => '>',
        (false, true) => '<',
        (true, false) => 'v',
        (false, false) => '^',
    };

    for _ in 0..amount.abs() {
        current_path.push(c);
    }
}

fn manhattan_distance(start: &Point, end: &Point) -> (isize, isize) {
    let vertical = end.row as isize - start.row as isize;
    let horizontal = end.col as isize - start.col as isize;

    (vertical, horizontal)
}

fn calculate_complexity_code(code: &[char], input_length: usize) -> usize {
    let str_value = String::from_iter(code.iter().filter(|c| c.is_numeric()));
    let numeric_value = str_value.parse::<usize>().unwrap();

    numeric_value * input_length
}

fn part_2(_contents: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 126384);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 270084);
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
