use std::collections::{HashSet, VecDeque};

use aoc_lib::grid::{Grid, Pos};
use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().expect("Invalid input");

    let mut areas: Vec<Area> = Vec::new();

    let mut result = 0;

    for p in grid.iter() {
        if areas.iter().any(|a| a.contains(&p.pos)) {
            continue;
        }
        let area = Area::connected_with(&grid, p.pos);
        result += area.area() * area.circumference();
        areas.push(area);
    }

    result
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().expect("Invalid input");

    let mut areas: Vec<Area> = Vec::new();

    let mut result = 0;

    for p in grid.iter() {
        if areas.iter().any(|a| a.contains(&p.pos)) {
            continue;
        }
        dbg!(p.value);
        let area = Area::connected_with(&grid, p.pos);
        result += dbg!(area.area()) * dbg!(area.edges());
        areas.push(area);
    }

    result
}

struct Area {
    area: HashSet<Pos>,
    circumference: usize,
    up_edges: Vec<Pos>,
    down_edges: Vec<Pos>,
    left_edges: Vec<Pos>,
    right_edges: Vec<Pos>,
}

impl Area {
    fn connected_with(grid: &Grid<char>, pos: Pos) -> Self {
        let &plant = grid
            .get(pos)
            .unwrap_or_else(|| panic!("Invalid position {:?}", pos));

        let mut nodes = VecDeque::new();
        nodes.push_back(pos);
        let mut area = HashSet::new();
        let mut circumference = 0;
        let mut all_edges = HashSet::new();
        let mut up_edges = Vec::new();
        let mut down_edges = Vec::new();
        let mut left_edges = Vec::new();
        let mut right_edges = Vec::new();

        while let Some(node) = nodes.pop_front() {
            if area.contains(&node) {
                continue;
            }
            area.insert(node);
            let mut edges = 4;

            let up = node.up();
            if let Some(&up_plant) = grid.get(up) {
                if up_plant == plant {
                    nodes.push_back(up);
                    edges -= 1;
                } else {
                    up_edges.push(node);
                }
            } else {
                up_edges.push(node)
            }
            let down = node.down();
            if let Some(&down_plant) = grid.get(down) {
                if down_plant == plant {
                    nodes.push_back(down);
                    edges -= 1;
                } else {
                    down_edges.push(node);
                }
            } else {
                down_edges.push(node);
            }
            let left = node.left();
            if let Some(&left_plant) = grid.get(left) {
                if left_plant == plant {
                    nodes.push_back(left);
                    edges -= 1;
                } else {
                    left_edges.push(node);
                }
            } else {
                left_edges.push(node);
            }
            let right = node.right();
            if let Some(&right_plant) = grid.get(right) {
                if right_plant == plant {
                    nodes.push_back(right);
                    edges -= 1;
                } else {
                    right_edges.push(node);
                }
            } else {
                right_edges.push(node);
            }
            if edges > 1 {
                all_edges.insert(node);
            }
            circumference += edges;
        }

        Self {
            area,
            circumference,
            up_edges,
            down_edges,
            left_edges,
            right_edges,
        }
    }

    fn contains(&self, pos: &Pos) -> bool {
        self.area.contains(pos)
    }

    fn area(&self) -> usize {
        self.area.len()
    }

    fn circumference(&self) -> usize {
        self.circumference
    }

    fn edges(&self) -> usize {
        let mut edges = 0;
        for (_row, group) in self
            .up_edges
            .iter()
            .sorted_by_key(|p| p.row())
            .group_by(|p| p.row())
            .into_iter()
        {
            let mut current_edge: Option<usize> = None;
            for &p in group.sorted_by_key(|p| p.col()) {
                if let Some(current) = current_edge {
                    if p.col() == current + 1 {
                        current_edge.replace(p.col());
                    } else {
                        //New edge
                        edges += 1;
                        current_edge.replace(p.col());
                    }
                } else {
                    //New edge
                    edges += 1;
                    current_edge = Some(p.col());
                }
            }
        }
        for (_row, group) in self
            .down_edges
            .iter()
            .sorted_by_key(|p| p.row())
            .group_by(|p| p.row())
            .into_iter()
        {
            let mut current_edge: Option<usize> = None;
            for &p in group.sorted_by_key(|p| p.col()) {
                if let Some(current) = current_edge {
                    if p.col() == current + 1 {
                        current_edge.replace(p.col());
                    } else {
                        //New edge
                        edges += 1;
                        current_edge.replace(p.col());
                    }
                } else {
                    //New edge
                    edges += 1;
                    current_edge = Some(p.col());
                }
            }
        }
        for (_row, group) in self
            .left_edges
            .iter()
            .sorted_by_key(|p| p.col())
            .group_by(|p| p.col())
            .into_iter()
        {
            let mut current_edge: Option<usize> = None;
            for &p in group.sorted_by_key(|p| p.row()) {
                if let Some(current) = current_edge {
                    if p.row() == current + 1 {
                        current_edge.replace(p.row());
                    } else {
                        //New edge
                        edges += 1;
                        current_edge.replace(p.row());
                    }
                } else {
                    //New edge
                    edges += 1;
                    current_edge.replace(p.row());
                }
            }
        }
        for (_row, group) in self
            .right_edges
            .iter()
            .sorted_by_key(|p| p.col())
            .group_by(|p| p.col())
            .into_iter()
        {
            let mut current_edge: Option<usize> = None;
            for &p in group.sorted_by_key(|p| p.row()) {
                if let Some(current) = current_edge {
                    if p.row() == current + 1 {
                        current_edge.replace(p.row());
                    } else {
                        //New edge
                        edges += 1;
                        current_edge.replace(p.row());
                    }
                } else {
                    //New edge
                    edges += 1;
                    current_edge = Some(p.row());
                }
            }
        }
        dbg!(edges);

        edges
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 1930);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 1206);
    }
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
}
