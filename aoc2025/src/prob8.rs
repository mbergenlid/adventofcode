use std::{collections::BTreeMap, str::FromStr};

use itertools::Itertools;

use crate::prob8::part2::SubGraphs;

pub fn solve_part_1(input: &str) -> usize {
    part1::solve_part_1::<1000>(input)
}

pub fn solve_part_2(input: &str) -> usize {
    let coordinates: Coordinates = input.parse().unwrap();

    let mut graphs = SubGraphs::new(coordinates.0.iter());
    for (p1, p2) in coordinates.edges() {
        graphs.merge_in_edge(&p1, &p2);
        if graphs.len() == 1 {
            return p1.0 as usize * p2.0 as usize;
        }
    }
    panic!("Could not connect all nodes");
}

mod part1 {
    use std::collections::{HashSet, VecDeque};

    use crate::prob8::{Coordinates, Point};
    pub fn solve_part_1<const N: usize>(input: &str) -> usize {
        let coordinates: Coordinates = input.parse().unwrap();

        let edges = coordinates.edges().into_iter().take(N).collect::<Vec<_>>();

        let mut coordinates: HashSet<Point, _> = HashSet::from_iter(coordinates);
        let mut sub_graphs = Vec::new();
        while let Some(coordinate) = pop(&mut coordinates) {
            let mut queue = VecDeque::new();
            queue.push_back(coordinate);
            let mut node_count = 0;
            while let Some(c) = queue.pop_front() {
                node_count += 1;
                for &(n1, n2) in edges.iter() {
                    if n1 == c && coordinates.contains(&n2) {
                        queue.push_back(n2);
                        coordinates.remove(&n2);
                    }
                    if n2 == c && coordinates.contains(&n1) {
                        queue.push_back(n1);
                        coordinates.remove(&n1);
                    }
                }
            }
            sub_graphs.push(node_count);
        }
        sub_graphs.sort();
        sub_graphs.into_iter().rev().take(3).product()
    }

    fn pop(coordinates: &mut HashSet<Point>) -> Option<Point> {
        let first = *(coordinates.iter().next()?);
        coordinates.remove(&first);
        Some(first)
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Point(i64, i64, i64);

impl Point {
    fn square_distance_to(&self, other: &Point) -> u64 {
        (self.0 - other.0).pow(2) as u64
            + (self.1 - other.1).pow(2) as u64
            + (self.2 - other.2).pow(2) as u64
    }
}

struct Coordinates(Vec<Point>);

impl FromStr for Coordinates {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let coordinates: Vec<_> = input
            .trim()
            .lines()
            .map(|line| {
                let (x, y, z) = line
                    .splitn(3, ",")
                    .collect_tuple()
                    .unwrap_or_else(|| panic!("Input should contain 3 coordinates {line}"));
                Point(x.parse().unwrap(), y.parse().unwrap(), z.parse().unwrap())
            })
            .collect();
        Ok(Self(coordinates))
    }
}

impl IntoIterator for Coordinates {
    type Item = Point;

    type IntoIter = <Vec<Point> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Coordinates {
    fn edges(&self) -> impl IntoIterator<Item = (Point, Point)> {
        let mut edges = BTreeMap::new();
        for (c_index, c1) in self.0.iter().enumerate() {
            for c2 in self.0.iter().skip(c_index + 1) {
                edges.insert(c1.square_distance_to(c2), (*c1, *c2));
            }
        }
        edges.into_values()
    }
}

mod part2 {
    use std::collections::HashSet;

    use itertools::Itertools;

    use crate::prob8::Point;

    #[derive(Default)]
    pub struct SubGraphs {
        graphs: Vec<HashSet<Point>>,
    }

    impl SubGraphs {
        pub fn new<'a, I>(points: I) -> SubGraphs
        where
            I: Iterator<Item = &'a Point>,
        {
            let points = points.into_iter();
            let mut graphs = Vec::with_capacity(points.size_hint().0);
            for p in points {
                let mut g = HashSet::new();
                g.insert(*p);
                graphs.push(g);
            }

            Self { graphs }
        }

        pub fn merge_in_edge(&mut self, p1: &Point, p2: &Point) {
            let Some((g1_index, _)) = self.graphs.iter().find_position(|g| g.contains(p1)) else {
                panic!("Unknown junction {:?}", p1);
            };
            let Some((g2_index, _)) = self.graphs.iter().find_position(|g| g.contains(p2)) else {
                panic!("Unknown junction {:?}", p2);
            };
            if g1_index != g2_index {
                let lowest = g1_index.min(g2_index);
                let highest = g1_index.max(g2_index);
                let g2 = self.graphs.swap_remove(highest);
                self.graphs[lowest].extend(g2);
            }
        }

        pub(crate) fn len(&self) -> usize {
            self.graphs.len()
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob8::solve_part_2;

    #[test]
    fn test_part_1() {
        assert_eq!(super::part1::solve_part_1::<10>(INPUT), 40);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), 25272);
    }

    const INPUT: &'static str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
}
