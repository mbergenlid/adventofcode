use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;

pub fn solve_part_1(input: &str) -> usize {
    let grid = Part1Grid::from_str(input).unwrap();
    dijkstra(grid)
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = Part2Grid::from_str(input).unwrap();
    dijkstra(grid)
}

trait Grid {
    fn cost(&self, index: usize) -> usize;
    fn side_length(&self) -> usize;
}

struct Part1Grid {
    data: Vec<usize>,
    side_length: usize,
}

impl FromStr for Part1Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<_> = s
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
            .collect();
        let side_length = (data.len() as f64).sqrt().floor() as usize;
        Ok(Part1Grid {
            data,
            side_length,
        })
    }
}

impl Grid for Part1Grid {
    fn cost(&self, index: usize) -> usize {
        self.data[index]
    }

    fn side_length(&self) -> usize {
        self.side_length
    }
}

struct Part2Grid {
    data: Vec<usize>,
    side_length: usize,
    original_side_length: usize,
}

impl Grid for Part2Grid {
    fn cost(&self, index: usize) -> usize {
        let row = index / self.side_length;
        let col = index % self.side_length;
        let larger_row = row / self.original_side_length;
        let larger_col = col / self.original_side_length;
        let increase = larger_col + larger_row;

        let sub_row = row % self.original_side_length;
        let sub_col = col % self.original_side_length;
        let value = self.data[sub_row * self.original_side_length + sub_col] + increase;
        if value > 9 { value % 9 } else { value }
    }

    fn side_length(&self) -> usize {
        self.side_length
    }
}

impl FromStr for Part2Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<_> = s
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
            .collect();
        let original_side_length = (data.len() as f64).sqrt().floor() as usize;
        let side_length = original_side_length*5;
        Ok(Part2Grid {
            data,
            original_side_length,
            side_length,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: usize,
    distance: usize,
}

impl Node {
    fn new(position: usize, distance: usize) -> Self {
        Node { position, distance }
    }

    fn neighbours(&self, grid_size: usize) -> Vec<usize> {
        let mut res = Vec::new();
        if self.position >= grid_size {
            res.push(self.position - grid_size);
        }
        if self.position < grid_size * grid_size - grid_size {
            res.push(self.position + grid_size);
        }
        if self.position % grid_size > 0 {
            res.push(self.position - 1);
        }
        if self.position % grid_size < grid_size - 1 {
            res.push(self.position + 1);
        }
        res
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<T: Grid>(grid: T) -> usize {
    let grid_side = grid.side_length();
    let mut best_distances = vec![usize::MAX; grid_side * grid_side];
    best_distances[0] = 0;
    let goal = grid_side * grid_side - 1;
    let mut heap = BinaryHeap::new();
    heap.push(Node::new(0, 0));

    while let Some(node) = heap.pop() {
        if node.position == goal {
            return node.distance;
        }

        if node.distance > best_distances[node.position] {
            continue;
        }

        for next in node.neighbours(grid_side) {
            let next = Node::new(next, node.distance + grid.cost(next)); // { cost: cost + edge.cost, position: edge.node };

            if next.distance < best_distances[next.position] {
                heap.push(next);
                best_distances[next.position] = next.distance;
            }
        }
    }
    panic!("No sulution found")
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 40);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 315);
    }

    const TESTCASE: &'static str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";
}
