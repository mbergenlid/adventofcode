use std::collections::{HashSet, VecDeque};

use aoc_lib::grid::Pos;

pub fn solve_part_1(input: &str) -> usize {
    _solve_part_1(input, 71, 71, 1024)
}

pub fn solve_part_2(input: &str) -> String {
    let pos = _solve_part_2(input, 71, 71);
    format!("{},{}", pos.col(), pos.row())
}

fn _solve_part_1(input: &str, width: usize, height: usize, bytes_to_drop: usize) -> usize {
    let bytes = input
        .lines()
        .map(|p| {
            let (col, row) = p.split_once(",").unwrap();
            Pos::new(row.parse::<usize>().unwrap(), col.parse::<usize>().unwrap())
        })
        .take(bytes_to_drop)
        .collect::<HashSet<_>>();
    find_path(&bytes, width, height).unwrap()
}

fn _solve_part_2(input: &str, width: usize, height: usize) -> Pos {
    let bytes = input
        .lines()
        .map(|p| {
            let (col, row) = p.split_once(",").unwrap();
            Pos::new(row.parse::<usize>().unwrap(), col.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut upper_bound = bytes.len();
    let mut lower_bound = 0;
    loop {
        if dbg!(upper_bound) - dbg!(lower_bound) < 5 {
            for b in lower_bound..=upper_bound {

                let distance = find_path(
                    &bytes
                        .iter()
                        .take(b)
                        .copied()
                        .collect::<HashSet<_>>(),
                    width,
                    height,
                );

                if distance.is_none() {
                    return *bytes.get(b-1).unwrap();
                }
            }
            panic!("Not found");
        }

        let bytes_to_drop = lower_bound + (upper_bound - lower_bound)/2;
        let distance = find_path(
            &bytes
                .iter()
                .take(bytes_to_drop)
                .copied()
                .collect::<HashSet<_>>(),
            width,
            height,
        );

        if distance.is_some() {
            lower_bound = bytes_to_drop;
        } else {
            upper_bound = bytes_to_drop;
        }
    }
}

fn find_path(bytes: &HashSet<Pos>, width: usize, height: usize) -> Option<usize> {
    let mut nodes = VecDeque::new();
    nodes.push_back((Pos::new(0, 0), 0));

    let mut visited = HashSet::new();

    while let Some((node, distance)) = nodes.pop_front() {
        if node == Pos::new(height - 1, width - 1) {
            return Some(distance);
        }

        //neighbours
        if let Some(&up) = node.up().in_bounds(width, height) {
            if !visited.contains(&up) && !bytes.contains(&up) {
                visited.insert(up);
                nodes.push_back((up, distance + 1));
            }
        }
        if let Some(&right) = node.right().in_bounds(width, height) {
            if !visited.contains(&right) && !bytes.contains(&right) {
                visited.insert(right);
                nodes.push_back((right, distance + 1));
            }
        }
        if let Some(&down) = node.down().in_bounds(width, height) {
            if !visited.contains(&down) && !bytes.contains(&down) {
                visited.insert(down);
                nodes.push_back((down, distance + 1));
            }
        }
        if let Some(&left) = node.left().in_bounds(width, height) {
            if !visited.contains(&left) && !bytes.contains(&left) {
                visited.insert(left);
                nodes.push_back((left, distance + 1));
            }
        }
    }
    None
}

trait PosInBounds {
    fn in_bounds(&self, width: usize, height: usize) -> Option<&Self>;
}

impl PosInBounds for Pos {
    fn in_bounds(&self, width: usize, height: usize) -> Option<&Self> {
        if self.row() < height && self.col() < width {
            Some(self)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use aoc_lib::grid::Pos;


    #[test]
    fn part_1() {
        assert_eq!(super::_solve_part_1(INPUT, 7, 7, 12), 22);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::_solve_part_2(INPUT, 7, 7), Pos::new(1, 6));
    }

    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
}
