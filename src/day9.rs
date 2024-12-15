use utilities;

const DAY: usize = 9;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let section_list = create_section_list(contents);
    let block_list = defragment_sections(&section_list);

    calculate_checksum(&block_list)
}

#[derive(Debug)]
struct Section {
    file_id: Option<usize>,
    len: usize,
}

impl Section {
    fn new(file_id: Option<usize>, len: usize) -> Self {
        Self { file_id, len }
    }
}

fn create_section_list(contents: &str) -> Vec<Section> {
    let mut section_list = vec![];

    let mut is_file = true;
    let mut current_file_id: usize = 0;
    for c in contents.chars() {
        let length = c.to_digit(10).unwrap() as usize;

        let file_id = if is_file {
            let file_id = current_file_id;
            current_file_id += 1;
            Some(file_id)
        } else {
            None
        };

        let section = Section::new(file_id, length);

        section_list.push(section);

        is_file = !is_file;
    }

    section_list
}

fn defragment_sections(section_list: &Vec<Section>) -> Vec<usize> {
    let mut block_list = vec![];

    let mut available_file_blocks = section_list.iter().filter_map(|section| section.file_id.map(|_| section.len)).collect::<Vec<usize>>();
    let total_file_blocks = available_file_blocks.iter().sum::<usize>();

    if total_file_blocks == 0 {
        return block_list;
    }

    let mut last_file_id = available_file_blocks.len() - 1;

    for section in section_list {
        if block_list.len() == total_file_blocks {
            break;
        }

        if let Some(file_id) = section.file_id {
            // If it's a file, copy over any available file blocks.
            let available_len = available_file_blocks[file_id];

            for _ in 0..available_len {
                block_list.push(file_id);
            }

            available_file_blocks[file_id] = 0;
        } else {
            // Need to grab file blocks from the last file, considering how many blocks are remaining.
            let blocks_to_copy = section.len.min(total_file_blocks - block_list.len());

            let mut blocks_copied = 0;

            while blocks_copied < blocks_to_copy {
                let available_blocks = available_file_blocks[last_file_id];

                if available_blocks == 0 {
                    // No more blocks in the current file, need to go to the previous file.
                    last_file_id -= 1;

                    continue;
                }

                let file_blocks_to_copy = available_blocks.min(blocks_to_copy - blocks_copied);

                for _ in 0..file_blocks_to_copy {
                    block_list.push(last_file_id);
                }

                available_file_blocks[last_file_id] = available_blocks - file_blocks_to_copy;

                blocks_copied += file_blocks_to_copy;
            }
        }
    }

    block_list
}

fn calculate_checksum(block_list: &Vec<usize>) -> usize {
    block_list.iter().enumerate().fold(0, |acc, (idx, val)| acc + (idx * val))
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

        assert_eq!(part_1(&contents), 1928);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 6356833654075);
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
