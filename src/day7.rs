use utilities;

const DAY: usize = 7;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let equations = parse_input(contents);

    equations
        .iter()
        .filter(|(result, operands)| try_evaluate(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

fn parse_input(contents: &str) -> Vec<(usize, Vec<usize>)> {
    contents
        .lines()
        .map(|line| {
            let pieces = line.split(":").collect::<Vec<_>>();
            let result = pieces[0].parse::<usize>().unwrap();
            let operands = pieces[1]
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap());

            (result, operands.collect())
        })
        .collect()
}

fn try_evaluate(result: usize, operands: &Vec<usize>) -> bool {
    // Simple brute force way, stopping early if the result is crossed.
    for i in 0..(1 << (operands.len() - 1)) {
        let mut flags = i;

        let mut current = operands[0];

        for j in 1..operands.len() {
            if flags % 2 == 0 {
                current += operands[j];
            } else {
                current *= operands[j];
            }

            flags >>= 1;

            if current > result {
                break;
            }
        }

        if current == result {
            return true;
        }
    }

    false
}

fn part_2(contents: &str) -> usize {
    let equations = parse_input(contents);

    equations
        .iter()
        .filter(|(result, operands)| try_evaluate_v2(*result, operands))
        .map(|(result, _)| result)
        .sum()
}

fn try_evaluate_v2(result: usize, operands: &Vec<usize>) -> bool {
    // Simple brute force way, stopping early if the result is crossed.
    let num_ops: usize = 3;

    let max_val = num_ops.pow((operands.len() - 1) as u32);

    for i in 0..max_val {
        let mut flags = i;

        let mut current = operands[0];

        for j in 1..operands.len() {
            match flags % num_ops {
                0 => current += operands[j],
                1 => current *= operands[j],
                2 => {
                    let mut s = current.to_string();
                    s.push_str(&operands[j].to_string());

                    current = s.parse::<usize>().unwrap()
                }
                _ => panic!("Illegal operation!"),
            }

            flags /= num_ops;

            if current > result {
                break;
            }
        }

        if current == result {
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

        assert_eq!(part_1(&contents), 3749);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 20281182715321);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 11387);
    }

    #[test]
    #[ignore]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 159490400628354);
    }
}
