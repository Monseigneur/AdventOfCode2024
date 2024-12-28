use utilities;

const DAY: usize = 13;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let claw_machines = parse_input(contents);

    claw_machines.iter().map(|cm| play_claw_machine(cm)).sum()
}

struct ClawMachine {
    a_button: (usize, usize),
    b_button: (usize, usize),
    prize: (usize, usize),
}

impl ClawMachine {
    fn new(a_button: (usize, usize), b_button: (usize, usize), prize: (usize, usize)) -> Self {
        Self {
            a_button,
            b_button,
            prize,
        }
    }
}

fn parse_input(contents: &str) -> Vec<ClawMachine> {
    let mut a_buttons = vec![];
    let mut b_buttons = vec![];
    let mut prizes = vec![];

    for line in contents.lines() {
        if line.is_empty() {
            continue;
        }

        let line_pieces = line.split(":").collect::<Vec<_>>();

        let values = line_pieces[1]
            .split(",")
            .map(|s| s.trim())
            .map(|s| {
                s.split(&['+', '='])
                    .last()
                    .unwrap()
                    .parse::<usize>()
                    .unwrap()
            })
            .collect::<Vec<_>>();
        let values = (values[0], values[1]);

        match line_pieces[0] {
            "Button A" => a_buttons.push(values),
            "Button B" => b_buttons.push(values),
            "Prize" => prizes.push(values),
            _ => panic!("Illegal text!"),
        };
    }

    let mut claw_machines = vec![];

    for i in 0..a_buttons.len() {
        claw_machines.push(ClawMachine::new(a_buttons[i], b_buttons[i], prizes[i]));
    }

    claw_machines
}

fn play_claw_machine(claw_machine: &ClawMachine) -> usize {
    let (ax, ay) = claw_machine.a_button;
    let (bx, by) = claw_machine.b_button;
    let (px, py) = claw_machine.prize;

    let num = (py * ax).abs_diff(px * ay);
    let denom = (ax * by).abs_diff(ay * bx);

    if denom == 0 || num % denom != 0 {
        return 0;
    }

    let m = num / denom;

    let num = px - (m * bx);
    let denom = ax;

    if denom == 0 || num % denom != 0 {
        return 0;
    }

    let n = num / denom;

    n * 3 + m
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

        assert_eq!(part_1(&contents), 480);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 36838);
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
