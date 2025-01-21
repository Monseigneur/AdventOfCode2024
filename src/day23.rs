use std::collections::{HashMap, HashSet};

const DAY: usize = 23;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let connections = parse_input(contents);

    count_triples_with_t(&connections)
}

fn parse_input(contents: &str) -> Vec<(&str, &str)> {
    contents
        .lines()
        .map(|line| {
            let pieces = line.split('-').collect::<Vec<_>>();
            (pieces[0], pieces[1])
        })
        .collect()
}

fn count_triples_with_t(connections: &[(&str, &str)]) -> usize {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

    let mut triples = HashSet::new();

    for (a, b) in connections {
        graph
            .entry(a)
            .and_modify(|v| {
                v.insert(*b);
            })
            .or_insert(HashSet::from_iter([*b]));
        graph
            .entry(b)
            .and_modify(|v| {
                v.insert(*a);
            })
            .or_insert(HashSet::from_iter([*a]));
    }

    for (first, values) in &graph {
        // Go through values to see if they have each other.
        for second in values {
            let possible_thirds = graph.get(second).unwrap();

            for third in values {
                if third == second {
                    continue;
                }

                if possible_thirds.contains(third) {
                    triples.insert(sort_nodes(first, second, third));
                }
            }
        }
    }

    triples
        .iter()
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count()
}

fn sort_nodes<'x>(a: &'x str, b: &'x str, c: &'x str) -> (&'x str, &'x str, &'x str) {
    let mut items = vec![a, b, c];
    items.sort();

    (items[0], items[1], items[2])
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

        assert_eq!(part_1(&contents), 7);
    }

    #[test]
    fn test_input_part_1() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_1(&contents), 1200);
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
