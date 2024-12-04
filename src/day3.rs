use utilities;

const DAY: usize = 3;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    const PREFIX: &'static str = "mul(";

    let mut total_value = 0;
    let mut cursor = contents;

    while let Some(offset) = cursor.find(PREFIX) {
        let next_start = offset + PREFIX.len();

        let (adjustment, value) = check_value(&cursor[next_start..]);

        cursor = &cursor[next_start + adjustment..];

        total_value += value.unwrap_or(0);
    }

    total_value
}

fn check_value(cursor: &str) -> (usize, Option<usize>) {
    if let Some(close_position) = cursor.find(")") {
        let number_contents = &cursor[..close_position];

        let pieces: Vec<&str> = number_contents.split(",").collect();

        if pieces.len() == 2 {
            let first_val = parse_num(pieces[0]);
            let second_val = parse_num(pieces[1]);

            if first_val.is_some() && second_val.is_some() {
                return (close_position + 1, Some(first_val.unwrap() * second_val.unwrap()));
            }
        }
    }

    (1, None)
}

fn parse_num(s: &str) -> Option<usize> {
    if s.len() == 0 || s.len() > 3 {
        return None;
    }

    s.parse::<usize>().ok()
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

        assert_eq!(part_1(&contents), 161);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 189600467);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 48);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 0);
    }
}