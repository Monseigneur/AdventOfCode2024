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
                return (
                    close_position + 1,
                    Some(first_val.unwrap() * second_val.unwrap()),
                );
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
    const MUL_PREFIX: &'static str = "mul(";
    const DO_TOKEN: &'static str = "do()";
    const DONT_TOKEN: &'static str = "don't()";

    let mut total_value = 0;
    let mut cursor = contents;

    let mut token_offsets = vec![
        cursor.find(MUL_PREFIX),
        cursor.find(DO_TOKEN),
        cursor.find(DONT_TOKEN)
    ];

    let mut mul_enabled = true;

    while !cursor.is_empty() {
        // Find the closest token.
        let Some((idx, offset)) = get_closest(&token_offsets) else {
            break;
        };

        // Act on the item depending on idx.
        let adjustment = match idx {
            0 => {
                let next_start = offset + MUL_PREFIX.len();
                let (mul_adjustment, value) = check_value(&cursor[next_start..]);

                if mul_enabled {
                    total_value += value.unwrap_or(0);
                }

                next_start + mul_adjustment
            },
            1 => {
                mul_enabled = true;
                DO_TOKEN.len()
            },
            2 => {
                mul_enabled = false;
                DONT_TOKEN.len()
            },
            _ => panic!("Illegal value for idx {idx}")
        };

        // Clear out the "consumed" token.
        token_offsets[idx] = None;

        // Adjust positions by adjustment offset.
        for token_offset in token_offsets.iter_mut() {
            let Some(current_offset) = token_offset else {
                continue;
            };

            *token_offset = Some(*current_offset - adjustment);
        }

        // Adjust cursor and refill consumed item.
        cursor = &cursor[adjustment..];

        token_offsets[idx] = match idx {
            0 => cursor.find(MUL_PREFIX),
            1 => cursor.find(DO_TOKEN),
            2 => cursor.find(DONT_TOKEN),
            _ => panic!("Illegal value for idx {idx}")
        };
    }

    total_value
}

fn get_closest(token_offsets: &Vec<Option<usize>>) -> Option<(usize, usize)> {
    let mut best = None;

    for (idx, data) in token_offsets.iter().enumerate() {
        if let Some(position) = data {
            let Some((_, best_position)) = best else {
                best = Some((idx, *position));

                continue;
            };

            if *position < best_position {
                best = Some((idx, *position));
            }
        }
    }

    best
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

        assert_eq!(part_2(&contents), 107069718);
    }
}
