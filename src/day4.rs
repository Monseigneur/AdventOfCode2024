use utilities;

const DAY: usize = 4;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Grid = Vec<Vec<char>>;

fn part_1(contents: &str) -> usize {
    let grid = parse_grid(contents);

    let row_count = count_rows(&grid);
    let col_count = count_columns(&grid);
    let diag_bl_tr_count = count_diagonals_bl_tr(&grid);
    let diag_tl_br_count = count_diagonals_tl_br(&grid);

    row_count + col_count + diag_bl_tr_count + diag_tl_br_count
}

fn parse_grid(contents: &str) -> Grid {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn count_rows(grid: &Grid) -> usize {
    let rows = grid
        .iter()
        .map(|row_data| row_data.iter().collect::<String>())
        .collect::<Vec<String>>();

    count_instances(&rows)
}

fn count_columns(grid: &Grid) -> usize {
    let mut columns = vec![String::new(); grid[0].len()];

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            columns[col].push(grid[row][col]);
        }
    }

    count_instances(&columns)
}

fn count_diagonals_bl_tr(grid: &Grid) -> usize {
    let mut diagonals = vec![String::new(); grid.len() + grid[0].len() - 1];

    for row in (0..grid.len()).rev() {
        for col in 0..grid[0].len() {
            diagonals[row + col].push(grid[row][col]);
        }
    }

    for (i, d) in diagonals.iter().enumerate() {
        println!("{i} -> {d}");
    }

    count_instances(&diagonals)
}

fn count_diagonals_tl_br(grid: &Grid) -> usize {
    let mut diagonals = vec![String::new(); grid.len() + grid[0].len() - 1];

    let num_cols = grid[0].len();

    for row in (0..grid.len()).rev() {
        for col in (0..grid[0].len()).rev() {
            diagonals[num_cols - 1 - col + row].push(grid[row][col]);
        }
    }

    for (i, d) in diagonals.iter().enumerate() {
        println!("tlbr {i} -> {d}");
    }

    count_instances(&diagonals)
}

fn count_instances(lines: &Vec<String>) -> usize {
    const XMAS: &'static str = "XMAS";

    let mut count = 0;

    for (idx, line) in lines.iter().enumerate() {
        let mut line_count = 0;

        let mut cursor = &line[..];

        while let Some(position) = cursor.find(XMAS) {
            line_count += 1;
            cursor = &cursor[position + XMAS.len()..];
        }

        let line = line.chars().rev().collect::<String>();

        let mut cursor = &line[..];
        while let Some(position) = cursor.find(XMAS) {
            line_count += 1;
            cursor = &cursor[position + XMAS.len()..];
        }

        count += line_count;
    }

    count
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

        assert_eq!(part_1(&contents), 18);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 2551);
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
