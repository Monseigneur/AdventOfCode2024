use std::collections::{HashMap, HashSet};

use utilities;

const DAY: usize = 5;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Mapping = HashMap<usize, HashSet<usize>>;

fn part_1(contents: &str) -> usize {
    let (mapping, updates) = parse_input(contents);

    let mut middle_sum = 0;

    for update in updates {
        middle_sum += validate_updates(&update, &mapping).unwrap_or(0);
    }

    middle_sum
}

fn parse_input(contents: &str) -> (Mapping, Vec<Vec<usize>>) {
    let mut mapping: Mapping = HashMap::new();
    let mut updates = vec![];

    let mut done_mapping = false;

    for line in contents.lines() {
        if line.is_empty() {
            done_mapping = true;

            continue;
        }

        if !done_mapping {
            // Parsing the mapping section.
            let pages = line
                .split("|")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            mapping
                .entry(pages[0])
                .and_modify(|val| {
                    val.insert(pages[1]);
                })
                .or_insert({
                    let mut val = HashSet::new();
                    val.insert(pages[1]);
                    val
                });
        } else {
            // Parsing the updates.
            let pages = line
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();

            updates.push(pages);
        }
    }

    (mapping, updates)
}

fn validate_updates(update: &Vec<usize>, mapping: &Mapping) -> Option<usize> {
    let mut seen_pages = HashSet::new();

    for page in update {
        // Just need to check that this page is after all pages I have already seen.
        if let Some(pages_after) = mapping.get(page) {
            if seen_pages.intersection(&pages_after).count() != 0
            {
                return None;
            }
        }

        seen_pages.insert(*page);
    }

    Some(update[update.len() / 2])
}

fn part_2(contents: &str) -> usize {
    let (mapping, updates) = parse_input(contents);

    let mut middle_sum = 0;

    for update in updates {
        if let Some(_) = validate_updates(&update, &mapping) {
            continue;
        }

        // Maybe I can do it linearly? validate_updates fails because the intersection of mapping[item] and pages
        // already seen is not empty. In theory, if a page fails, it can be moved just before the "earliest" page
        // in the intersection.

        middle_sum += validate_updates_v2(&update, &mapping).unwrap_or(0);
    }

    middle_sum
}

fn validate_updates_v2(update: &Vec<usize>, mapping: &Mapping) -> Option<usize> {
    let mut seen_pages = HashSet::new();

    let mut adjusted = vec![];

    for page in update {
        if let Some(pages_after) = mapping.get(page) {
            let intersection = seen_pages.intersection(&pages_after).collect::<HashSet<_>>();

            if intersection.is_empty() {
                adjusted.push(*page);
            } else if let Some(index) = adjusted.iter().position(|item| intersection.contains(item)) {
                adjusted.insert(index, *page);
            }
        } else {
            adjusted.push(*page);
        }

        seen_pages.insert(*page);
    }

    Some(adjusted[adjusted.len() / 2])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 143);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 7307);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 123);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 4713);
    }
}
