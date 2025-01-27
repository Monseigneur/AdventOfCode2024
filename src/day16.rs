use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use crate::day10::get_neighbors;
use crate::day18::MinHeapNode;
use crate::day6::Point;

const DAY: usize = 16;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let (grid, start, end) = parse_input(contents);

    find_shortest_path(&grid, &start, &end)
}

type Grid = Vec<Vec<char>>;

fn parse_input(contents: &str) -> (Grid, Point, Point) {
    let grid = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut start = None;
    let mut end = None;

    for (row, row_data) in grid.iter().enumerate() {
        for (col, c) in row_data.iter().enumerate() {
            if *c == 'S' {
                start = Some(Point::new(row, col));
            } else if *c == 'E' {
                end = Some(Point::new(row, col));
            }
        }
    }

    (grid, start.unwrap(), end.unwrap())
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn opposite(&self) -> Self {
        match *self {
            Direction::North => Direction::South,
            Direction::East => Direction::West,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
        }
    }
}

fn find_shortest_path(grid: &Grid, start: &Point, end: &Point) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();

    queue.push(MinHeapNode::new(0, (*start, Direction::East)));

    while let Some(MinHeapNode(score, (point, direction))) = queue.pop() {
        if point == *end {
            return score;
        }

        if visited.contains(&point) {
            continue;
        }

        visited.insert(point);

        for neighbor in get_neighbors(&point, grid) {
            if grid[neighbor.row][neighbor.col] == '#' {
                continue;
            }

            let new_direction = direction_to_point(&point, &neighbor);

            let adjusted_score = if new_direction == direction {
                score + 1
            } else if new_direction == direction.opposite() {
                score + 2000 + 1
            } else {
                score + 1000 + 1
            };

            queue.push(MinHeapNode::new(adjusted_score, (neighbor, new_direction)));
        }
    }

    unreachable!("Did not find a path to the end!");
}

fn direction_to_point(start: &Point, other: &Point) -> Direction {
    let comp_row = (start.row as isize).cmp(&(other.row as isize));
    let comp_col = (start.col as isize).cmp(&(other.col as isize));

    match (comp_row, comp_col) {
        (std::cmp::Ordering::Less, std::cmp::Ordering::Equal) => Direction::South,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Less) => Direction::East,
        (std::cmp::Ordering::Equal, std::cmp::Ordering::Greater) => Direction::West,
        (std::cmp::Ordering::Greater, std::cmp::Ordering::Equal) => Direction::North,
        (_, _) => unreachable!("Points are not a cardinal direction from each other!"),
    }
}

fn part_2(contents: &str) -> usize {
    let (grid, start, end) = parse_input(contents);

    find_shortest_path_v2(&grid, &start, &end)
}

fn find_shortest_path_v2(grid: &Grid, start: &Point, end: &Point) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited: HashMap<(Point, Direction), usize> = HashMap::new();
    let mut parents: HashMap<(Point, usize), HashSet<(Point, usize)>> = HashMap::new();

    let mut best_score = None;

    queue.push(MinHeapNode::new(0, (*start, Direction::East, None)));

    while let Some(MinHeapNode(score, (point, direction, parent_info))) = queue.pop() {
        if let Some(parent) = parent_info {
            parents
                .entry((point, score))
                .and_modify(|p| {
                    p.insert(parent);
                })
                .or_insert(HashSet::from_iter([parent]));
        }

        visited.insert((point, direction), score);

        if point == *end {
            if let Some(best) = best_score {
                if score > best {
                    // No more shortest paths to the end, can stop searching
                    break;
                }
            } else {
                best_score = Some(score);
            }

            continue;
        }

        for neighbor in get_neighbors(&point, grid) {
            if grid[neighbor.row][neighbor.col] == '#' {
                continue;
            }

            let new_direction = direction_to_point(&point, &neighbor);

            let adjusted_score = if new_direction == direction {
                score + 1
            } else if new_direction == direction.opposite() {
                score + 2000 + 1
            } else {
                score + 1000 + 1
            };

            if visited.get(&(neighbor, new_direction)).is_some_and(|best| *best < adjusted_score) {
                continue;
            }

            queue.push(MinHeapNode::new(
                adjusted_score,
                (neighbor, new_direction, Some((point, score))),
            ));
        }
    }

    count_shortest_path_spaces(&parents, end, best_score.unwrap())
}

fn count_shortest_path_spaces(
    parents: &HashMap<(Point, usize), HashSet<(Point, usize)>>,
    end: &Point,
    best_score: usize,
) -> usize {
    let mut queue = VecDeque::new();

    let mut spaces = HashSet::new();

    queue.push_back((*end, best_score));

    while let Some((point, score)) = queue.pop_back() {
        spaces.insert(point);

        if let Some(point_parents) = parents.get(&(point, score)) {
            queue.extend(point_parents);
        }
    }

    spaces.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 7036);
    }

    #[test]
    fn test_example2_part_1() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_1(&contents), 11048);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 104516);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 45);
    }

    #[test]
    fn test_example2_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 64);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 545);
    }
}
