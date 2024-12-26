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

#[derive(Debug, Copy, Clone)]
struct Section {
    file_id: Option<usize>,
    start: usize,
    len: usize,
}

impl Section {
    fn new(file_id: Option<usize>, start: usize, len: usize) -> Self {
        Self {
            file_id,
            len,
            start,
        }
    }
}

fn create_section_list(contents: &str) -> Vec<Section> {
    let mut section_list = vec![];

    let mut is_file = true;
    let mut current_file_id: usize = 0;
    let mut current_position = 0;
    for c in contents.chars() {
        let length = c.to_digit(10).unwrap() as usize;

        let file_id = if is_file {
            let file_id = current_file_id;
            current_file_id += 1;
            Some(file_id)
        } else {
            None
        };

        let section = Section::new(file_id, current_position, length);

        section_list.push(section);

        current_position += length;

        is_file = !is_file;
    }

    section_list
}

fn defragment_sections(section_list: &Vec<Section>) -> Vec<usize> {
    let mut block_list = vec![];

    let mut available_file_blocks = section_list
        .iter()
        .filter_map(|section| section.file_id.map(|_| section.len))
        .collect::<Vec<usize>>();
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
    block_list
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, val)| acc + (idx * val))
}

fn part_2(contents: &str) -> usize {
    let section_list = create_section_list(contents);
    let defragmented_sections = defragment_sections_v2(&section_list);

    calculate_checksum_v2(&defragmented_sections)
}

fn defragment_sections_v2(section_list: &Vec<Section>) -> Vec<Section> {
    let mut file_sections = vec![];
    let mut empty_sections = vec![];

    for section in section_list {
        if section.file_id.is_some() {
            file_sections.push(section.clone());
        } else {
            empty_sections.push(section.clone());
        }
    }

    // For each file section in reverse, try to fit it in an earlier slot.
    for file_section in file_sections.iter_mut().rev() {
        for empty_section in empty_sections.iter_mut() {
            if file_section.start < empty_section.start {
                break;
            }

            if file_section.len <= empty_section.len {
                file_section.start = empty_section.start;

                empty_section.start += file_section.len;
                empty_section.len -= file_section.len;

                break;
            }
        }
    }

    file_sections.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

    file_sections
}

fn calculate_checksum_v2(section_list: &Vec<Section>) -> usize {
    section_list
        .iter()
        .map(|section| {
            let Some(file_id) = section.file_id else {
                return 0;
            };

            let mut checksum = 0;
            for i in section.start..(section.start + section.len) {
                checksum += file_id * i;
            }

            checksum
        })
        .sum()
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

        assert_eq!(part_2(&contents), 2858);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 6389911791746);
    }
}
