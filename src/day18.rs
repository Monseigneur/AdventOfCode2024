use std::collections::{BinaryHeap, HashSet};

use utilities;

use crate::day10::get_neighbors;
use crate::day6::Point;

const DAY: usize = 18;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    part_1_with_bounds(contents, 71, 71, 1024)
}

fn part_1_with_bounds(contents: &str, width: usize, height: usize, num_bytes: usize) -> usize {
    let bytes = parse_input(contents, num_bytes);
    let memory_region = build_memory_region(&bytes, width, height);

    find_shortest_path(
        &memory_region,
        &Point::new(0, 0),
        &Point::new(height - 1, width - 1),
    )
}

fn parse_input(contents: &str, num_bytes: usize) -> Vec<Point> {
    contents
        .lines()
        .map(|line| {
            let pieces = line
                .split(",")
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<_>>();

            Point::new(pieces[0], pieces[1])
        })
        .take(num_bytes)
        .collect()
}

type MemoryRegion = Vec<Vec<char>>;

fn build_memory_region(bytes: &[Point], width: usize, height: usize) -> MemoryRegion {
    let mut memory_region = vec![vec!['.'; width]; height];

    for byte in bytes {
        memory_region[byte.row][byte.col] = '#';
    }

    memory_region
}

struct MinHeapNode<T>(usize, T);

impl<T> MinHeapNode<T> {
    fn new(key: usize, value: T) -> Self {
        Self(key, value)
    }
}

impl<T> Eq for MinHeapNode<T> {}

impl<T> PartialEq for MinHeapNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T> PartialOrd for MinHeapNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(&other))
    }
}

impl<T> Ord for MinHeapNode<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Swap the order to be used in a min heap.
        other.0.cmp(&self.0)
    }
}

fn find_shortest_path(memory_region: &MemoryRegion, start: &Point, end: &Point) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(MinHeapNode::new(0, *start));

    while let Some(node) = queue.pop() {
        if node.1 == *end {
            return node.0;
        }

        if visited.contains(&node.1) {
            continue;
        }

        visited.insert(node.1);

        let next_dist = node.0 + 1;

        for neighbor in get_neighbors(&node.1, memory_region) {
            if memory_region[neighbor.row][neighbor.col] != '#' {
                queue.push(MinHeapNode::new(next_dist, neighbor));
            }
        }
    }

    unreachable!("Should have found a path!")
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

        assert_eq!(part_1_with_bounds(&contents, 7, 7, 12), 22);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 340);
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
