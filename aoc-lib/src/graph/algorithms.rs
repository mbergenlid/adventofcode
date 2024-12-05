use std::collections::VecDeque;

use super::DirectedGraph;


pub fn topological_sort<G, T>(mut graph: G) -> Vec<T>
where
    G: DirectedGraph<T>,
    T: Eq + Copy + 'static,
{
    let mut result = Vec::new();
    let mut nodes = graph
        .nodes()
        .filter(|&n| graph.incoming_edges(n).count() == 0)
        .copied()
        .collect::<VecDeque<_>>();

    while let Some(node) = nodes.pop_front() {
        result.push(node);

        while let Some((_, m)) = graph.pop_outgoing_edge(&node) {
            if graph.incoming_edges(&m).count() == 0 {
                nodes.push_back(m);
            }
        }
    }

    if graph.is_empty() {
        return result;
    } else {
        panic!("")
    }
}


#[cfg(test)]
mod test {
    use crate::graph::AdjacencyListGraph;

    #[test]
    fn topological_sort() {
        let graph = AdjacencyListGraph::new(vec![(5_usize, 11), (11, 2)]);
        assert_eq!(super::topological_sort(graph), vec![5, 11, 2]);

        let graph = AdjacencyListGraph::new(vec![ (5, 11), (11, 2), (7, 11), (7, 8), (11, 9), (11, 10), (8, 9), (3, 8), (3, 10) ]);
        let sort = super::topological_sort(graph);

        assert!(is_valid(&sort, &vec![ (5, 11), (11, 2), (7, 11), (7, 8), (11, 9), (11, 10), (8, 9), (3, 8), (3, 10) ]));
    }



    fn is_valid(update: &Vec<usize>, rules: &Vec<(usize, usize)>) -> bool {
        for (i, x) in update.iter().enumerate() {
            for other in &update[i + 1..] {
                if rules.iter().any(|(r1, r2)| r1 == other && r2 == x) {
                    return false;
                }
            }
        }
        true
    }
}
