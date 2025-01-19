use std::collections::{HashMap, HashSet};

use crate::day10::get_neighbors;
use crate::day6::Point;

const DAY: usize = 12;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

type Grid = Vec<Vec<char>>;

fn part_1(contents: &str) -> usize {
    let garden = parse_input(contents);
    let region_map = find_regions(&garden);

    region_map
        .values()
        .map(|regions| calculate_fence_cost(regions, &garden))
        .sum()
}

#[derive(Debug, Clone)]
struct Region {
    plant_type: char,
    points: HashSet<Point>,
}

impl Region {
    fn new(plant_type: char, start_point: Point) -> Self {
        Self {
            plant_type,
            points: HashSet::from([start_point]),
        }
    }

    fn area(&self) -> usize {
        self.points.len()
    }

    fn perimeter(&self, garden: &Grid) -> usize {
        let edge_map = self.build_edge_map(garden);

        edge_map
            .values()
            .map(|v| v.iter().filter(|edge| **edge).count())
            .sum()
    }

    fn build_edge_map(&self, garden: &Grid) -> HashMap<Point, Vec<bool>> {
        let mut edge_map = HashMap::new();

        for point in &self.points {
            let mut edges = vec![false; 4];

            // Up
            if point.row == 0 || garden[point.row - 1][point.col] != self.plant_type {
                edges[0] = true;
            }

            // Right
            if point.col == garden[0].len() - 1
                || garden[point.row][point.col + 1] != self.plant_type
            {
                edges[1] = true;
            }

            // Down
            if point.row == garden.len() - 1 || garden[point.row + 1][point.col] != self.plant_type
            {
                edges[2] = true;
            }

            // Left
            if point.col == 0 || garden[point.row][point.col - 1] != self.plant_type {
                edges[3] = true;
            }

            edge_map.insert(*point, edges);
        }

        edge_map
    }

    fn is_point_adjacent(&self, point: &Point, garden: &Grid) -> bool {
        for neighbor in get_neighbors(point, garden) {
            if self.points.contains(&neighbor) {
                return true;
            }
        }

        false
    }
}

fn parse_input(contents: &str) -> Grid {
    contents
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_regions(garden: &Grid) -> HashMap<char, Vec<Region>> {
    let mut region_map: HashMap<char, Vec<Region>> = HashMap::new();

    for (row, row_data) in garden.iter().enumerate() {
        for (col, plant_type) in row_data.iter().enumerate() {
            let point = Point::new(row, col);

            region_map
                .entry(*plant_type)
                .and_modify(|regions| {
                    let mut adjacent_regions = vec![];

                    for (i, region) in regions.iter_mut().enumerate() {
                        if region.is_point_adjacent(&point, garden) {
                            adjacent_regions.push(i);
                        }
                    }

                    if !adjacent_regions.is_empty() {
                        let main_region_idx = adjacent_regions[0];
                        regions[main_region_idx].points.insert(point);

                        // All other regions are also adjacent and need to be merged.
                        for region_idx in adjacent_regions.iter().skip(1) {
                            let points = regions[*region_idx].points.clone();
                            regions[*region_idx].points.clear();
                            regions[main_region_idx].points.extend(points);
                        }
                    } else {
                        regions.push(Region::new(*plant_type, point));
                    }
                })
                .or_insert(vec![Region::new(*plant_type, point)]);
        }
    }

    region_map
}

fn calculate_fence_cost(regions: &[Region], garden: &Grid) -> usize {
    regions
        .iter()
        .map(|region| region.area() * region.perimeter(garden))
        .sum()
}

fn part_2(contents: &str) -> usize {
    let garden = parse_input(contents);
    let region_map = find_regions(&garden);

    region_map
        .values()
        .map(|regions| calculate_fence_cost_v2(regions, &garden))
        .sum()
}

fn calculate_fence_cost_v2(regions: &[Region], garden: &Grid) -> usize {
    regions
        .iter()
        .map(|region| region.area() * count_region_edges(region, &garden))
        .sum()
}

fn count_region_edges(region: &Region, garden: &Grid) -> usize {
    if region.points.is_empty() {
        return 0;
    }

    let edge_map = region.build_edge_map(garden);
    let total_edges = edge_map
        .values()
        .map(|v| v.iter().filter(|edge| **edge).count())
        .sum::<usize>();

    let mut visited_edges: HashMap<Point, Vec<bool>> = HashMap::new();

    let mut processed_edges = 0;
    let mut turns = 0;

    while processed_edges != total_edges {
        let (mut current, current_edges) = edge_map
            .iter()
            .find(|(k, v)| {
                if let Some(visited) = visited_edges.get(k) {
                    v.iter()
                        .zip(visited.iter())
                        .find(|(edge, visited)| **edge && !**visited)
                        .is_some()
                } else {
                    v.iter().any(|x| *x)
                }
            })
            .map(|(k, v)| (*k, v.clone()))
            .unwrap();

        let mut edge_direction = 0;

        let visited = visited_edges.get(&current);

        for (direction, edge) in current_edges.iter().enumerate() {
            if *edge && visited.is_none_or(|v| !v[direction]) {
                edge_direction = direction;
                break;
            }
        }

        loop {
            if let Some(edges) = visited_edges.get(&current) {
                if edges[edge_direction] {
                    break;
                }
            }

            visited_edges
                .entry(current)
                .and_modify(|v| v[edge_direction] = true)
                .or_insert({
                    let mut edges = vec![false; 4];
                    edges[edge_direction] = true;
                    edges
                });

            let current_edges = edge_map.get(&current).unwrap();

            processed_edges += 1;

            // Check if this the end of the fence and we need to turn right.
            if current_edges[(edge_direction + 1) % 4] {
                turns += 1;
                edge_direction = (edge_direction + 1) % 4;

                continue;
            }

            // Can go straight, or need to turn left.
            let next_point = match edge_direction {
                0 => Point::new(current.row, current.col + 1),
                1 => Point::new(current.row + 1, current.col),
                2 => Point::new(current.row, current.col - 1),
                3 => Point::new(current.row - 1, current.col),
                _ => unreachable!(),
            };

            if edge_map.get(&next_point).unwrap()[edge_direction] {
                // Continue forward.
                current = next_point;

                continue;
            }

            // Need to turn left.
            current = match edge_direction {
                0 => Point::new(current.row - 1, current.col + 1),
                1 => Point::new(current.row + 1, current.col + 1),
                2 => Point::new(current.row + 1, current.col - 1),
                3 => Point::new(current.row - 1, current.col - 1),
                _ => unreachable!(),
            };

            turns += 1;
            edge_direction = (edge_direction + 3) % 4;
        }
    }

    turns
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_part_1() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_1(&contents), 140);
    }

    #[test]
    fn test_example2_part_1() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_1(&contents), 1930);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1434856);
    }

    #[test]
    fn test_example_part_2() {
        let contents = utilities::read_file_data(DAY, "example.txt");

        assert_eq!(part_2(&contents), 80);
    }

    #[test]
    fn test_example2_part_2() {
        let contents = utilities::read_file_data(DAY, "example2.txt");

        assert_eq!(part_2(&contents), 1206);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 891106);
    }
}
