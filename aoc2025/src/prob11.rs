use aoc_lib::graph::{AdjacencyListGraph, DirectedGraph};

pub fn solve_part_1(input: &str) -> usize {
    let edges: Vec<(String, String)> = input
        .lines()
        .flat_map(|line| {
            let (source, targets) = line.split_once(": ").unwrap();

            targets
                .split(" ")
                .map(|t| (source.to_string(), t.to_string()))
        })
        .collect();

    let graph = AdjacencyListGraph::new(edges);

    dfs::<String>(&graph, &"you".to_string(), &"out".to_string())
}

fn dfs<T: Eq + Sized + 'static>(graph: &AdjacencyListGraph<T>, start: &T, end: &T) -> usize {
    if start == end {
        return 1;
    }
    let mut count = 0;
    for (_x, to) in graph.outgoing_edges(start) {
        count += dfs::<T>(graph, to, end);
    }
    count
}

pub fn solve_part_2(input: &str) -> usize {
    let edges: Vec<(String, String)> = input
        .lines()
        .flat_map(|line| {
            let (source, targets) = line.split_once(": ").unwrap();

            targets
                .split(" ")
                .map(|t| (source.to_string(), t.to_string()))
        })
        .collect();

    let graph = AdjacencyListGraph::new(edges);
    part2::solve(&graph)
}

mod part2 {
    use std::collections::HashMap;

    use aoc_lib::graph::{algorithms::topological_sort, AdjacencyListGraph, DirectedGraph};

    pub fn solve(graph: &AdjacencyListGraph<String>) -> usize {
        let mut nodes = HashMap::<&str, NodeResult>::new();
        let mut out = NodeResult::default();
        out.paths_both = 1;
        nodes.insert("out", out);

        let order = topological_sort(graph.clone());
        for node in order.iter().filter(|n| *n != "out").rev() {
            let mut result = NodeResult::default();
            for (_, n) in graph.outgoing_edges(node) {
                let Some(neighbour_result) = nodes.get(n.as_str()) else {
                    panic!();
                };
                result.paths_none += neighbour_result.paths_none;
                result.paths_fft += neighbour_result.paths_fft + neighbour_result.paths_none;
                result.paths_dac += neighbour_result.paths_dac + neighbour_result.paths_none;
                result.paths_both += neighbour_result.paths_both
                    + neighbour_result.paths_fft
                    + neighbour_result.paths_dac
                    + neighbour_result.paths_none;

                if node == "dac" {
                    result.paths_none += neighbour_result.paths_dac;
                    result.paths_fft += neighbour_result.paths_both;
                }
                if node == "fft" {
                    result.paths_none += neighbour_result.paths_fft;
                    result.paths_dac += neighbour_result.paths_both;
                }
            }

            nodes.insert(node.as_str(), result);
        }

        nodes
            .get("svr")
            .expect("All nodes should be there")
            .paths_none
    }

    #[derive(Default, Debug)]
    pub struct NodeResult {
        paths_none: usize,
        paths_dac: usize,
        paths_fft: usize,
        paths_both: usize,
    }
}

#[cfg(test)]
mod test {
    use crate::prob11::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 5);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT_2), 2);
    }

    const INPUT: &'static str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";

    const INPUT_2: &'static str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";
}
