use std::collections::{HashMap, HashSet};

use utilities;

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
        let mut perimeter = 0;

        for point in &self.points {
            // Calculate the perimeter contributed by each point, depending on the neighbors. There is only no
            // fence if the neighbor exists and is the same plant type.
            let mut point_perimeter = 4;
            for neighbor in get_neighbors(point, garden) {
                if garden[neighbor.row][neighbor.col] == self.plant_type {
                    point_perimeter -= 1;
                }
            }

            perimeter += point_perimeter;
        }

        perimeter
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
                        for i in 1..adjacent_regions.len() {
                            let region_idx = adjacent_regions[i];
                            let points = regions[region_idx].points.clone();
                            regions[region_idx].points.clear();
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

fn calculate_fence_cost(regions: &Vec<Region>, garden: &Grid) -> usize {
    regions
        .iter()
        .map(|region| region.area() * region.perimeter(garden))
        .sum()
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

        assert_eq!(part_2(&contents), 0);
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), 0);
    }
}
