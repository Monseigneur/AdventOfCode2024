use std::collections::{HashMap, HashSet};

const DAY: usize = 23;

pub fn run() {
    utilities::run_puzzle(DAY, part_1, part_2);
}

fn part_1(contents: &str) -> usize {
    let connections = parse_input(contents);
    let graph = build_graph(&connections);
    let triples = find_triples(&graph);

    count_triples_with_t(&triples)
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

fn build_graph<'x>(connections: &[(&'x str, &'x str)]) -> HashMap<&'x str, HashSet<&'x str>> {
    let mut graph: HashMap<&str, HashSet<&str>> = HashMap::new();

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

    graph
}

fn find_triples<'x>(graph: &HashMap<&'x str, HashSet<&'x str>>) -> HashSet<Vec<&'x str>> {
    let mut triples = HashSet::new();

    for (first, values) in graph {
        // Go through values to see if they have each other.
        for second in values {
            let possible_thirds = graph.get(second).unwrap();

            for third in values {
                if third == second {
                    continue;
                }

                if possible_thirds.contains(third) {
                    let mut triple = vec![*first, *second, *third];
                    triple.sort();

                    triples.insert(triple);
                }
            }
        }
    }

    triples
}

fn count_triples_with_t(triples: &HashSet<Vec<&str>>) -> usize {
    triples
        .iter()
        .filter(|triple| triple.iter().any(|item| item.starts_with('t')))
        .count()
}

fn part_2(contents: &str) -> String {
    let connections = parse_input(contents);
    let graph = build_graph(&connections);
    let triples = find_triples(&graph);

    find_biggest_group(&graph, &triples).join(",")
}

fn find_biggest_group<'x>(
    graph: &HashMap<&'x str, HashSet<&'x str>>,
    triples: &HashSet<Vec<&'x str>>,
) -> Vec<&'x str> {
    let groups: Vec<HashSet<&str>> = triples
        .iter()
        .map(|triple| HashSet::from_iter(triple.iter().cloned()))
        .collect();

    groups
        .iter()
        .map(|group| expand_group(graph, group))
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
}

fn expand_group<'x>(
    graph: &HashMap<&'x str, HashSet<&'x str>>,
    group: &HashSet<&'x str>,
) -> Vec<&'x str> {
    let others = graph
        .keys()
        .filter(|&&key| !group.contains(key))
        .collect::<HashSet<_>>();

    let mut current_group = Vec::from_iter(group.iter().cloned());

    for other in others.iter() {
        let mut connected = true;
        for item in &current_group {
            if !graph.get(item).unwrap().contains(&**other) {
                connected = false;
                break;
            }
        }

        if connected {
            current_group.push(other);
        }
    }

    current_group.sort();

    current_group
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

        assert_eq!(part_2(&contents), "co,de,ka,ta");
    }

    #[test]
    fn test_input_part_2() {
        let contents = utilities::read_file_data(DAY, "input.txt");

        assert_eq!(part_2(&contents), "ag,gh,hh,iv,jx,nq,oc,qm,rb,sm,vm,wu,zr");
    }
}
