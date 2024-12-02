use utilities;

const DAY: usize = 2;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|str| str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(check_line)
        .count()
}

fn check_line(line: &Vec<usize>) -> bool {
    let mut increasing = None;

    for i in 1..line.len() {
        let delta = line[i].abs_diff(line[i - 1]);

        if delta < 1 || delta > 3 {
            return false;
        }

        if increasing.is_none() {
            increasing = Some(line[i - 1] < line[i]);
        }

        if (line[i - 1] < line[i]) != increasing.unwrap() {
            return false;
        }
    }

    true
}

fn part_2(contents: &str) -> usize {
    contents
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|str| str.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .filter(check_line_v2)
        .count()
}

fn check_line_v2(line: &Vec<usize>) -> bool {
    if check_line(line) {
        return true;
    }

    for skipped_index in 0..line.len() {
        let mut line2 = line.clone();

        line2.remove(skipped_index);

        if check_line(&line2) {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 2);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 670);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 4);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 670);
    }
}
