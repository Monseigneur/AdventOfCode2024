const DAY: usize = 17;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> String {
    let mut computer = parse_input(contents);

    while computer.step(false) {}

    computer.get_output()
}

struct Computer {
    a: usize,
    b: usize,
    c: usize,
    ip: usize,
    instructions: Vec<usize>,
    output: Vec<usize>,
    initial_a: usize,
    initial_b: usize,
    initial_c: usize,
}

impl Computer {
    fn new(a: usize, b: usize, c: usize, instructions: Vec<usize>) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            instructions,
            output: vec![],
            initial_a: a,
            initial_b: b,
            initial_c: c,
        }
    }

    fn step(&mut self, exit_on_branch: bool) -> bool {
        let opcode = self.instructions[self.ip];
        let operand = self.instructions[self.ip + 1];
        let combo_operand = self.get_combo_operand(operand);

        let mut advance_ip = true;

        match opcode {
            0 => self.a = self.dv(combo_operand),
            1 => self.b ^= operand,
            2 => self.b = combo_operand % 8,
            3 => {
                if self.a != 0 {
                    self.ip = operand;
                    advance_ip = false;

                    if exit_on_branch {
                        return false;
                    }
                }
            }
            4 => self.b ^= self.c,
            5 => self.output.push(combo_operand % 8),
            6 => self.b = self.dv(combo_operand),
            7 => self.c = self.dv(combo_operand),
            _ => unreachable!("Illegal opcode"),
        }

        if advance_ip {
            self.ip += 2;
        }

        self.ip < self.instructions.len()
    }

    fn get_combo_operand(&self, operand: usize) -> usize {
        if operand <= 3 {
            operand
        } else {
            match operand {
                4 => self.a,
                5 => self.b,
                6 => self.c,
                _ => panic!("Illegal operand"),
            }
        }
    }

    fn dv(&self, combo_operand: usize) -> usize {
        self.a / 2_usize.pow(combo_operand as u32)
    }

    fn get_output(&self) -> String {
        self.output
            .iter()
            .map(usize::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }

    fn reset(&mut self) {
        self.output.clear();
        self.ip = 0;

        self.a = self.initial_a;
        self.b = self.initial_b;
        self.c = self.initial_c;
    }
}

fn parse_input(contents: &str) -> Computer {
    let mut registers = vec![];
    let mut instructions = vec![];

    let mut done_registers = false;

    for line in contents.lines() {
        if line.is_empty() {
            done_registers = true;
            continue;
        }

        let line_data = line.split(':').nth(1).unwrap().trim();

        if !done_registers {
            let register = line_data.parse::<usize>().unwrap();

            registers.push(register);
        } else {
            instructions.extend(line_data.split(',').map(|s| s.parse::<usize>().unwrap()));
        }
    }

    Computer::new(registers[0], registers[1], registers[2], instructions)
}

fn part_2(contents: &str) -> usize {
    let mut computer = parse_input(contents);

    calculate_value(&mut computer)
}

fn calculate_value(computer: &mut Computer) -> usize {
    let instructions = computer
        .instructions
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<_>>();

    let result = calculate_value_helper(computer, &instructions, 0, 0).unwrap();

    // Validate
    computer.reset();
    computer.a = result;

    while computer.step(false) {}

    if computer.output != computer.instructions {
        panic!("Incorrect value!");
    }

    result
}

fn calculate_value_helper(
    computer: &mut Computer,
    instructions: &Vec<usize>,
    value: usize,
    idx: usize,
) -> Option<usize> {
    if idx == instructions.len() {
        return Some(value);
    }

    let instruction = instructions[idx];

    for i in 0..8 {
        let current_value = value << 3 | i;

        computer.reset();
        computer.a = current_value;

        while computer.step(true) {}

        if let Some(last) = computer.output.first() {
            if last == &instruction {
                let result = calculate_value_helper(computer, instructions, current_value, idx + 1);

                if result.is_some() {
                    return result;
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), "7,5,4,3,4,5,3,4,6");
    }

    #[test]
    fn test_example2_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 117440);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 164278899142333);
    }
}
