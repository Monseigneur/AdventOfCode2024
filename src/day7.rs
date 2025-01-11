const DAY: usize = 7;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let equations = parse_input(contents);

    equations
        .iter()
        .filter(|(result, operands)| try_evaluate(*result, operands, false))
        .map(|(result, _)| result)
        .sum()
}

fn parse_input(contents: &str) -> Vec<(usize, Vec<usize>)> {
    contents
        .lines()
        .map(|line| {
            let pieces = line.split(':').collect::<Vec<_>>();
            let result = pieces[0].parse::<usize>().unwrap();
            let operands = pieces[1]
                .split_ascii_whitespace()
                .map(|s| s.parse::<usize>().unwrap());

            (result, operands.collect())
        })
        .collect()
}

fn part_2(contents: &str) -> usize {
    let equations = parse_input(contents);

    equations
        .iter()
        .filter(|(result, operands)| try_evaluate(*result, operands, true))
        .map(|(result, _)| result)
        .sum()
}

enum Operation {
    Add,
    Multiply,
    Concatenate,
}

fn try_evaluate(result: usize, operands: &[usize], use_concatenation: bool) -> bool {
    let all_operations = [Operation::Concatenate, Operation::Multiply, Operation::Add];

    let available_operations = if use_concatenation {
        &all_operations[..]
    } else {
        &all_operations[1..]
    };

    try_evaluate_helper(result, operands, available_operations)
}

fn try_evaluate_helper(
    value: usize,
    operands: &[usize],
    available_operations: &[Operation],
) -> bool {
    if operands.is_empty() {
        return value == 0;
    }

    let last_idx = operands.len() - 1;
    let operand = operands[last_idx];
    let next_operands = &operands[..last_idx];

    for operation in available_operations {
        let Some(new_value) = apply_operation(value, operand, operation) else {
            continue;
        };

        if try_evaluate_helper(new_value, next_operands, available_operations) {
            return true;
        }
    }

    false
}

fn apply_operation(value: usize, operand: usize, operation: &Operation) -> Option<usize> {
    match operation {
        Operation::Add if value >= operand => Some(value - operand),
        Operation::Multiply if value % operand == 0 => Some(value / operand),
        Operation::Concatenate => remove_concatenation(value, operand),
        _ => None,
    }
}

fn remove_concatenation(value: usize, operand: usize) -> Option<usize> {
    let mut value = value;
    let mut operand = operand;

    while operand != 0 {
        if operand % 10 != value % 10 {
            return None;
        }

        operand /= 10;
        value /= 10;
    }

    Some(value)
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
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 159490400628354);
    }
}
