use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let (instructions, graph) = input.split("\n\n").collect_tuple().unwrap();

    let graph = graph.parse::<Graph>().unwrap();

    let mut steps = 0;
    let mut current_node = "AAA";
    for ins in instructions.chars().cycle() {
        if current_node == "ZZZ" {
            return steps;
        }

        let n = graph.nodes.get(current_node).unwrap();
        let next = match ins {
            'L' => &n.left,
            'R' => &n.right,
            _ => panic!(),
        };

        current_node = next;
        steps += 1;
    }
    unreachable!()
}

pub fn solve_part_2(input: &str) -> usize {
    let (instructions, graph) = input.split("\n\n").collect_tuple().unwrap();

    let graph = graph.parse::<Graph>().unwrap();

    solve(&graph, instructions, "LLA");
    let current_nodes = graph
        .nodes
        .keys()
        .filter(|k| k.ends_with("A"))
        .collect::<Vec<_>>();
    let result = current_nodes
        .into_iter()
        .map(|n| {
            let mut steps = 0;
            let mut current = n;
            for ins in instructions.chars().cycle() {
                if current.ends_with("Z") {
                    break;
                } else {
                    let n = graph.nodes.get(current).unwrap();
                    let next = match ins {
                        'L' => &n.left,
                        'R' => &n.right,
                        _ => panic!(),
                    };

                    current = next;
                    steps += 1;
                }
            }
            (steps, n, current)
        })
        .collect::<Vec<_>>();

    dbg!(result);
    panic!()
}

fn solve(graph: &Graph, instructions: &str, start_node: &str) -> usize {
    let mut current = start_node;
    let mut steps = 0;
    let mut iterations = 5;
    for ins in instructions.chars().cycle() {
        if current.ends_with("Z") {
            println!("Steps: {}, node: {:?}", steps, current);
            iterations -= 1;
            if iterations == 0 {
                break;
            }
        } else {
            let n = graph.nodes.get(current).unwrap();
            let next = match ins {
                'L' => &n.left,
                'R' => &n.right,
                _ => panic!(),
            };

            current = next;
            steps += 1;
        }
    }

    return steps;
}

struct Graph {
    nodes: HashMap<String, Node>,
}

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}

impl FromStr for Graph {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = HashMap::new();
        for line in s.lines() {
            let (id, node) = line.split(" = ").collect_tuple().unwrap();
            let (left, right) = node[1..node.len() - 1].split(", ").collect_tuple().unwrap();
            nodes.insert(
                id.to_string(),
                Node {
                    left: left.to_string(),
                    right: right.to_string(),
                },
            );
        }
        Ok(Graph { nodes })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 6);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT_2), 6);
    }

    const TEST_INPUT: &'static str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    const TEST_INPUT_2: &'static str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
}
