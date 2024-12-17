use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

use aoc_lib::grid::{Grid, Pos};

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().unwrap();
    dijkstra(&grid).unwrap().0
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input.parse::<Grid<char>>().unwrap();
    dijkstra(&grid).unwrap().1
}

fn dijkstra(grid: &Grid<char>) -> Option<(usize, usize)> {
    let source = grid.iter().find(|p| p.value == 'S')?;

    let mut prevs: HashMap<State, HashSet<State>> = HashMap::new();
    let mut heap = BinaryHeap::new();
    let mut distances: HashMap<(Pos, Direction), usize> = HashMap::new();
    for n in grid.iter() {
        distances.insert((n.pos, Direction::Up), usize::MAX);
        distances.insert((n.pos, Direction::Right), usize::MAX);
        distances.insert((n.pos, Direction::Down), usize::MAX);
        distances.insert((n.pos, Direction::Left), usize::MAX);
    }
    distances.insert((source.pos, Direction::Right), 0);
    heap.push(State {
        cost: 0,
        position: source.pos,
        direction: Direction::Right,
    });

    let goal = grid.iter().find(|p| p.value == 'E').map(|p| p.pos)?;

    while let Some(State {
        cost,
        position,
        direction,
    }) = heap.pop()
    {
        if position == goal {
            let mut nodes_on_path: HashSet<Pos> = HashSet::new();
            let mut nodes_to_visit = VecDeque::new();
            nodes_to_visit.push_back(State {
                cost,
                position: goal,
                direction,
            });
            while let Some(node) = nodes_to_visit.pop_front() {
                nodes_on_path.insert(node.position);

                if let Some(parents) = prevs.get(&node) {
                    for p in parents {
                        nodes_to_visit.push_back(p.clone());
                    }
                }
            }
            return Some((cost, nodes_on_path.len()));
        }

        if cost > distances[&(position, direction)] {
            continue;
        }

        {
            let clockwise = State {
                cost: cost + 1000,
                position,
                direction: direction.turn_clockwise(),
            };
            if clockwise.cost <= distances[&(clockwise.position, clockwise.direction)] {
                distances.insert((clockwise.position, clockwise.direction), clockwise.cost);
                if let Some(parents) = prevs.get_mut(&clockwise) {
                    parents.insert(State {
                        cost,
                        position,
                        direction,
                    });
                    heap.push(clockwise);
                } else {
                    prevs.insert(
                        clockwise.clone(),
                        vec![State {
                            cost,
                            position,
                            direction,
                        }]
                        .into_iter()
                        .collect(),
                    );
                    heap.push(clockwise);
                }
            }
        }

        {
            let c_clockwise = State {
                cost: cost + 1000,
                position,
                direction: direction.turn_counter_clockwise(),
            };
            if c_clockwise.cost <= distances[&(c_clockwise.position, c_clockwise.direction)] {
                distances.insert(
                    (c_clockwise.position, c_clockwise.direction),
                    c_clockwise.cost,
                );
                if let Some(parents) = prevs.get_mut(&c_clockwise) {
                    parents.insert(State {
                        cost,
                        position,
                        direction,
                    });
                    heap.push(c_clockwise);
                } else {
                    prevs.insert(
                        c_clockwise.clone(),
                        vec![State {
                            cost,
                            position,
                            direction,
                        }]
                        .into_iter()
                        .collect(),
                    );
                    heap.push(c_clockwise);
                }
            }
        }

        {
            let new_position = direction.step(&position);
            if let Some(&value) = grid.get(new_position) {
                if value == '.' || value == 'S' || value == 'E' {
                    let forward = State {
                        cost: cost + 1,
                        position: new_position,
                        direction,
                    };
                    if forward.cost <= distances[&(forward.position, forward.direction)] {
                        if let Some(parents) = prevs.get_mut(&forward) {
                            parents.insert(State {
                                cost,
                                position,
                                direction,
                            });
                            heap.push(forward);
                        } else {
                            prevs.insert(
                                forward.clone(),
                                vec![State {
                                    cost,
                                    position,
                                    direction,
                                }]
                                .into_iter()
                                .collect(),
                            );
                            heap.push(forward);
                        }
                    }
                }
            }
        }
    }

    None
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct State {
    cost: usize,
    position: Pos,
    direction: Direction,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.row().cmp(&other.position.row()))
            .then_with(|| self.position.col().cmp(&other.position.col()))
            .then_with(|| self.direction.cmp(&other.direction))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Eq, PartialEq, Hash, Debug, Ord, PartialOrd, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }

    fn turn_counter_clockwise(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }

    fn step(&self, position: &Pos) -> Pos {
        match self {
            Direction::Up => position.up(),
            Direction::Right => position.right(),
            Direction::Down => position.down(),
            Direction::Left => position.left(),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT_1), 7036);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT_1), 45);
    }

    const INPUT_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
}
