use std::hash::Hash;

use itertools::Itertools;


pub mod algorithms;

pub trait DirectedGraph<T> {
    fn edges(&self) -> impl Iterator<Item = &(T, T)>
    where
        T: 'static;

    fn nodes(&self) -> impl Iterator<Item = &T>
    where
        T: 'static;

    fn incoming_edges(&self, node: &T) -> impl Iterator<Item = &(T, T)>
    where
        T: Eq + 'static;
    fn outgoing_edges(&self, node: &T) -> impl Iterator<Item = &(T, T)>
    where
        T: Eq + 'static;

    fn pop_outgoing_edge(&mut self, node: &T) -> Option<(T, T)>
    where
        T: Eq;

    fn is_empty(&self) -> bool;
}

#[derive(Debug)]
pub struct AdjacencyListGraph<T> {
    data: Vec<(T, T)>,
    nodes: Vec<T>,
}

impl <T> AdjacencyListGraph<T> where T: Clone + Eq + Hash {

    pub fn new(edges: Vec<(T, T)>) -> Self {
        let nodes = edges.iter().flat_map(|(a, b)| [a.clone(), b.clone()].into_iter()).unique().collect::<Vec<_>>();
        AdjacencyListGraph {
            data: edges,
            nodes,
        }
    }
}

impl<T> DirectedGraph<T> for AdjacencyListGraph<T> {
    fn edges(&self) -> impl Iterator<Item = &(T, T)>
    where
        T: 'static,
    {
        self.data.iter()
    }

    fn nodes(&self) -> impl Iterator<Item = &T>
    where
        T: 'static,
    {
        self.nodes.iter()
    }

    fn incoming_edges(&self, node: &T) -> impl Iterator<Item = &(T, T)>
    where
        T: Eq + 'static,
    {
        self.data.iter().filter(move |(_, b)| b == node)
    }

    fn outgoing_edges(&self, _node: &T) -> impl Iterator<Item = &(T, T)>
    where
        T: Eq + 'static,
    {
        Vec::new().into_iter()
    }

    fn pop_outgoing_edge(&mut self, node: &T) -> Option<(T, T)>
    where
        T: Eq,
    {
        if let Some((index, _)) = self.data.iter().enumerate().find(|(_, (n, _))| n == node) {
            return Some(self.data.remove(index));
        }
        None
    }

    fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}
