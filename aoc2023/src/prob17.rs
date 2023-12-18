use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    grid.shortest_path(0, 3)
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    grid.shortest_path(4, 10)
}

struct Grid {
    data: Vec<Vec<usize>>,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
struct Node {
    node: (usize, usize),
    dir: Direction,
}

impl Node {
    fn neighbours(&self, min_steps: u8, max_steps: u8) -> Vec<Node> {
        let mut result = Vec::with_capacity(4);
        let (row, col) = self.node;

        if let Some(s) = self.dir.steps() {
            if s < min_steps {
                match self.dir {
                    Direction::None => panic!(),
                    Direction::Up(s) => {
                        result.push(Node {
                            node: (row.wrapping_sub(1), col),
                            dir: Direction::Up(s + 1),
                        });
                    }
                    Direction::Left(s) => {
                        result.push(Node {
                            node: (row, col.wrapping_sub(1)),
                            dir: Direction::Left(s + 1),
                        });
                    }
                    Direction::Down(s) => {
                        result.push(Node {
                            node: (row + 1, col),
                            dir: Direction::Down(s + 1),
                        });
                    }
                    Direction::Right(s) => {
                        result.push(Node {
                            node: (row, col + 1),
                            dir: Direction::Right(s + 1),
                        });
                    }
                }
                return result;
            }
        }

        {
            let node = (row.wrapping_sub(1), col);
            if let Direction::Up(s) = self.dir {
                if s < max_steps {
                    result.push(Node {
                        node,
                        dir: Direction::Up(s + 1),
                    });
                }
            } else if let Direction::Down(_) = self.dir {
                // Do nothing
            } else {
                result.push(Node {
                    node,
                    dir: Direction::Up(1),
                });
            }
        }
        {
            let node = (row, col + 1);
            if let Direction::Right(s) = self.dir {
                if s < max_steps {
                    result.push(Node {
                        node,
                        dir: Direction::Right(s + 1),
                    });
                }
            } else if let Direction::Left(_) = self.dir {
                //Do nothing
            } else {
                result.push(Node {
                    node,
                    dir: Direction::Right(1),
                });
            }
        }
        {
            let node = (row + 1, col);
            if let Direction::Down(s) = self.dir {
                if s < max_steps {
                    result.push(Node {
                        node,
                        dir: Direction::Down(s + 1),
                    });
                }
            } else if let Direction::Up(_) = self.dir {
                //Do nothing
            } else {
                result.push(Node {
                    node,
                    dir: Direction::Down(1),
                });
            }
        }
        {
            let node = (row, col.wrapping_sub(1));
            if let Direction::Left(s) = self.dir {
                if s < max_steps {
                    result.push(Node {
                        node,
                        dir: Direction::Left(s + 1),
                    });
                }
            } else if let Direction::Right(_) = self.dir {
                //Do nothing
            } else {
                result.push(Node {
                    node,
                    dir: Direction::Left(1),
                });
            }
        }

        result
    }
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct State {
    node: Node,
    cost: usize,
    //path: Vec<Node>,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
enum Direction {
    None,
    Up(u8),
    Left(u8),
    Down(u8),
    Right(u8),
}

impl Direction {
    fn steps(&self) -> Option<u8> {
        match self {
            Direction::None => None,
            Direction::Up(s) => Some(*s),
            Direction::Left(s) => Some(*s),
            Direction::Down(s) => Some(*s),
            Direction::Right(s) => Some(*s),
        }
    }
}

impl Grid {
    fn shortest_path(&self, min_steps: u8, max_steps: u8) -> usize {
        // dist[node] = current shortest distance from `start` to `node`
        let mut dist: HashMap<Node, usize> = HashMap::new();
        for row in 0..self.data.len() {
            for col in 0..self.data[0].len() {
                let node = (row, col);
                dist.insert(
                    Node {
                        node,
                        dir: Direction::None,
                    },
                    usize::MAX,
                );
                for steps in 1..=max_steps {
                    dist.insert(
                        Node {
                            node,
                            dir: Direction::Up(steps),
                        },
                        usize::MAX,
                    );
                    dist.insert(
                        Node {
                            node,
                            dir: Direction::Left(steps),
                        },
                        usize::MAX,
                    );
                    dist.insert(
                        Node {
                            node,
                            dir: Direction::Down(steps),
                        },
                        usize::MAX,
                    );
                    dist.insert(
                        Node {
                            node,
                            dir: Direction::Right(steps),
                        },
                        usize::MAX,
                    );
                }
            }
        }

        let mut heap = BinaryHeap::new();

        let start = Node {
            node: (0, 0),
            dir: Direction::None,
        };
        // We're at `start`, with a zero cost
        *dist.get_mut(&start).unwrap() = 0;
        heap.push(State {
            node: start,
            cost: 0,
        });

        let goal = (self.data.len() - 1, self.data[0].len() - 1);
        // Examine the frontier with lower cost nodes first (min-heap)
        while let Some(State { cost, node }) = heap.pop() {
            if node.node == goal && node.dir.steps().unwrap_or(0) >= min_steps {
                return cost;
            }

            // Important as we may have already found a better way
            if cost > *dist.get(&node).unwrap() {
                continue;
            }

            // For each node we can reach, see if we can find a way with
            // a lower cost going through this node
            for neighbour in node.neighbours(min_steps, max_steps) {
                if neighbour.node.0 > goal.0 || neighbour.node.1 > goal.1 {
                    continue;
                }
                let heat_loss = self.data[neighbour.node.0][neighbour.node.1];
                let next = State {
                    cost: cost + heat_loss,
                    node: neighbour,
                };

                // If so, add it to the frontier and continue
                if next.cost < *dist.get(&next.node).unwrap() {
                    // Relaxation, we have now found a better way
                    *dist.get_mut(&next.node).unwrap() = next.cost;
                    heap.push(next);
                }
            }
        }

        // Goal not reachable
        unreachable!()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Ok(Grid { data })
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 102);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 94);
    }

    const TEST_INPUT: &'static str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
}
