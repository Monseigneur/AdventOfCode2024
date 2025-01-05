use std::collections::{BinaryHeap, HashMap, HashSet};

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
    let bytes = parse_input(contents, Some(num_bytes));
    let memory_region = build_memory_region(&bytes, width, height);

    find_shortest_path(
        &memory_region,
        &Point::new(0, 0),
        &Point::new(height - 1, width - 1),
    )
}

fn parse_input(contents: &str, num_bytes: Option<usize>) -> Vec<Point> {
    let points = contents.lines().map(|line| {
        let pieces = line
            .split(",")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        Point::new(pieces[0], pieces[1])
    });

    if let Some(num_bytes) = num_bytes {
        points.take(num_bytes).collect()
    } else {
        points.collect()
    }
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
    find_shortest_path_points(memory_region, start, end)
        .unwrap()
        .len()
        - 1
}

fn find_shortest_path_points(
    memory_region: &MemoryRegion,
    start: &Point,
    end: &Point,
) -> Option<HashSet<Point>> {
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();

    queue.push(MinHeapNode::new(0, (*start, None)));

    let mut found_end = false;
    while let Some(MinHeapNode(key, (point, parent))) = queue.pop() {
        if visited.contains_key(&point) {
            continue;
        }

        visited.insert(point, parent);

        if point == *end {
            found_end = true;
            break;
        }

        let next_dist = key + 1;

        for neighbor in get_neighbors(&point, memory_region) {
            if memory_region[neighbor.row][neighbor.col] != '#' {
                queue.push(MinHeapNode::new(next_dist, (neighbor, Some(point))));
            }
        }
    }

    if found_end {
        let mut path_points = HashSet::new();
        path_points.insert(*end);

        let mut current = *end;
        while let Some(Some(parent)) = visited.get(&current) {
            path_points.insert(*parent);
            current = *parent;
        }

        return Some(path_points);
    }

    None
}

fn part_2(contents: &str) -> String {
    part_2_with_bounds(contents, 71, 71, 1024)
}

fn part_2_with_bounds(
    contents: &str,
    width: usize,
    height: usize,
    prefill_byte_count: usize,
) -> String {
    let bytes = parse_input(contents, None);
    let start = Point::new(0, 0);
    let end = Point::new(height - 1, width - 1);

    let mut memory_region = build_memory_region(&bytes[..prefill_byte_count], width, height);

    let mut byte_idx = prefill_byte_count;
    while let Some(path_points) = find_shortest_path_points(&memory_region, &start, &end) {
        for i in byte_idx..bytes.len() {
            let byte = bytes[i];

            memory_region[byte.row][byte.col] = '#';

            if path_points.contains(&byte) {
                byte_idx = i;
                break;
            }
        }
    }

    let byte = bytes[byte_idx];
    format!("{},{}", byte.row, byte.col)
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

        assert_eq!(part_2_with_bounds(&contents, 7, 7, 0), "6,1");
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), "34,32");
    }
}
