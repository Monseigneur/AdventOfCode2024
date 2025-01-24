use std::collections::{HashMap, VecDeque};

const DAY: usize = 24;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let (gates, gate_inputs, starting_values) = parse_input(contents);

    run_simulation(gates, &gate_inputs, starting_values)
}

enum GateType {
    And,
    Or,
    Xor,
}

impl GateType {
    fn from_str(s: &str) -> Self {
        match s {
            "AND" => GateType::And,
            "OR" => GateType::Or,
            "XOR" => GateType::Xor,
            _ => unreachable!(),
        }
    }
}

struct Gate<'a> {
    gate_type: GateType,
    input: Option<usize>,
    output: &'a str,
}

impl<'a> Gate<'a> {
    fn new(output: &'a str, gate_type: GateType) -> Self {
        Self {
            gate_type,
            input: None,
            output,
        }
    }

    fn apply_input(&mut self, input: usize) -> Option<usize> {
        if let Some(prev_input) = self.input {
            match self.gate_type {
                GateType::And => Some(prev_input & input),
                GateType::Or => Some(prev_input | input),
                GateType::Xor => Some(prev_input ^ input),
            }
        } else {
            self.input = Some(input);
            None
        }
    }
}

type GateInputs<'a> = HashMap<&'a str, Vec<usize>>;
type StartingValues<'a> = HashMap<&'a str, usize>;

fn parse_input(contents: &str) -> (Vec<Gate>, GateInputs, StartingValues) {
    let mut starting_values = HashMap::new();
    let mut gates = vec![];
    let mut gate_inputs: GateInputs = HashMap::new();

    let mut done_starting_values = false;

    for line in contents.lines() {
        if line.is_empty() {
            done_starting_values = true;
            continue;
        }

        if !done_starting_values {
            let pieces = line.split(": ").collect::<Vec<_>>();

            starting_values.insert(pieces[0], pieces[1].parse::<usize>().unwrap());
        } else {
            let pieces = line.split(" -> ").collect::<Vec<_>>();
            let gate_pieces = pieces[0].split_ascii_whitespace().collect::<Vec<_>>();

            let idx = gates.len();

            gates.push(Gate::new(pieces[1], GateType::from_str(gate_pieces[1])));

            gate_inputs
                .entry(gate_pieces[0])
                .and_modify(|v| v.push(idx))
                .or_insert(vec![idx]);
            gate_inputs
                .entry(gate_pieces[2])
                .and_modify(|v| v.push(idx))
                .or_insert(vec![idx]);
        }
    }

    (gates, gate_inputs, starting_values)
}

fn run_simulation(
    gates: Vec<Gate>,
    gate_inputs: &GateInputs,
    starting_values: StartingValues,
) -> usize {
    let mut gates = gates;
    let mut wire_queue = VecDeque::new();
    let mut end_wires = HashMap::new();

    // Prefill the wire queue.
    for (wire, initial_value) in starting_values {
        if let Some(dest_indices) = gate_inputs.get(wire) {
            push_all(&mut wire_queue, dest_indices, initial_value);
        }
    }

    while let Some((dest_idx, value)) = wire_queue.pop_front() {
        let gate = gates.get_mut(dest_idx).unwrap();

        let Some(value) = gate.apply_input(value) else {
            continue;
        };

        if gate.output.starts_with('z') {
            let digit_position = gate.output[1..].parse::<usize>().unwrap();
            end_wires.insert(digit_position, value);
        } else if let Some(dest_indices) = gate_inputs.get(gate.output) {
            push_all(&mut wire_queue, dest_indices, value);
        }
    }

    build_value(&end_wires)
}

fn push_all(wire_queue: &mut VecDeque<(usize, usize)>, dest_indices: &Vec<usize>, value: usize) {
    for dest_idx in dest_indices {
        wire_queue.push_back((*dest_idx, value));
    }
}

fn build_value(digits: &HashMap<usize, usize>) -> usize {
    let mut value = 0;

    for (digit, digit_value) in digits {
        value |= digit_value << digit;
    }

    value
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

        assert_eq!(part_1(&contents), 4);
    }

    #[test]
    fn test_example2_part_1() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_1(&contents), 2024);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 36902370467952);
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
