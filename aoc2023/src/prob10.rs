use std::{
    collections::{HashSet, VecDeque},
    convert::{TryFrom, TryInto},
    ops::Sub,
    str::FromStr,
};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();

    grid.find_path().len() / 2
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap().extend();

    let mut unvisited = HashSet::new();
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            unvisited.insert(Pos { row, col });
        }
    }

    let path = grid.find_path().into_iter().collect::<HashSet<_>>();
    unvisited = unvisited.sub(&path);

    let mut visited = HashSet::new();
    while !unvisited.is_empty() {
        let pos = unvisited.iter().next().cloned().unwrap();
        unvisited.remove(&pos);

        let mut queue: VecDeque<Pos> = VecDeque::new();
        queue.push_back(pos);

        let mut current_span = HashSet::new();
        while let Some(pos) = queue.pop_front() {
            unvisited.remove(&pos);
            if pos.row % 2 == 0 && pos.col % 2 == 0 {
                current_span.insert(pos);
            }
            for maybe_neighbour in ALL_DIRECTIONS
                .iter()
                .map(|&d| pos.step(d).map(|p| Step { pos: p, dir: d }))
            {
                if let Some(neighbour) = maybe_neighbour {
                    if !visited.contains(&neighbour.pos) {
                        visited.insert(neighbour.pos);
                        if !current_span.contains(&neighbour.pos) {
                            if neighbour.pos.row >= grid.height()
                                || neighbour.pos.col >= grid.width()
                            {
                                queue.clear();
                                current_span.clear();
                                break;
                            }
                            if !path.contains(&neighbour.pos) {
                                queue.push_back(neighbour.pos);
                            }
                        }
                    }
                } else {
                    queue.clear();
                    current_span.clear();
                }
            }
        }
        if !current_span.is_empty() {
            return current_span.len();
        }
    }
    todo!()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn step(&self, dir: Direction) -> Option<Pos> {
        let (row, col) = match dir {
            Direction::North => (self.row as i64 - 1, self.col as i64),
            Direction::East => (self.row as i64, self.col as i64 + 1),
            Direction::South => (self.row as i64 + 1, self.col as i64),
            Direction::West => (self.row as i64, self.col as i64 - 1),
        };

        if row >= 0 && col >= 0 {
            Some(Pos {
                row: row as usize,
                col: col as usize,
            })
        } else {
            None
        }
    }
}
struct Grid {
    grid: Vec<Vec<Option<Pipe>>>,
    start_pos: Pos,
    start_dir: (Direction, Direction),
}

impl Grid {
    fn find_path(&self) -> Vec<Pos> {
        let mut path = Vec::new();
        path.push(self.start_pos);
        let mut step = Step {
            pos: self.start_pos.step(self.start_dir.0).unwrap(),
            dir: self.start_dir.0,
        };
        path.push(step.pos);

        loop {
            if step.pos == self.start_pos {
                return path;
            }
            if let Some(pipe) = &self.grid[step.pos.row][step.pos.col] {
                let next = pipe.step(step);
                path.push(next.pos);
                step = next;
            } else {
                panic!("No pipe at {:?}", step)
            }
        }
    }

    fn extend(self) -> Grid {
        let mut result = Vec::new();
        for row in self.grid {
            let mut new_row = Vec::new();
            for col in row {
                new_row.push(col);
                match col {
                    Some(Pipe::Horizontal) => new_row.push(Some(Pipe::Horizontal)),
                    Some(Pipe::NorthEast) => new_row.push(Some(Pipe::Horizontal)),
                    Some(Pipe::SouthEast) => new_row.push(Some(Pipe::Horizontal)),
                    _ => new_row.push(None),
                }
            }
            let mut extra_row = Vec::new();
            for col in new_row.iter() {
                let new_col = match col {
                    Some(Pipe::Vertical) => Some(Pipe::Vertical),
                    Some(Pipe::SouthEast) => Some(Pipe::Vertical),
                    Some(Pipe::SouthWest) => Some(Pipe::Vertical),
                    _ => None,
                };
                extra_row.push(new_col);
            }
            result.push(new_row);
            result.push(extra_row);
        }

        Grid {
            grid: result,
            start_pos: Pos {
                row: self.start_pos.row * 2,
                col: self.start_pos.col * 2,
            },
            start_dir: self.start_dir,
        }
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }

    fn height(&self) -> usize {
        self.grid.len()
    }
}

impl FromStr for Grid {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        for row in s.lines() {
            let mut row_result: Vec<Option<Pipe>> = Vec::with_capacity(row.len());
            for col in row.chars() {
                row_result.push(col.try_into().ok())
            }
            grid.push(row_result);
        }
        let start_pos = find_start(&grid);

        fn find_start(grid: &Vec<Vec<Option<Pipe>>>) -> Pos {
            for (ri, r) in grid.iter().enumerate() {
                for (ci, c) in r.iter().enumerate() {
                    if let Some(Pipe::Start) = c {
                        return Pos { row: ri, col: ci };
                    }
                }
            }
            panic!("Couldn't find the start");
        }

        let directions = ALL_DIRECTIONS
            .iter()
            .copied()
            .filter(|&dir| {
                if let Some(pos) = start_pos.step(dir) {
                    if let Some(pipe) = &grid[pos.row][pos.col] {
                        pipe.accessable_from(dir)
                    } else {
                        false
                    }
                } else {
                    false
                }
            })
            .sorted()
            .collect_tuple::<(_, _)>()
            .unwrap();

        let starting_pipe = match directions {
            (Direction::North, Direction::South) => Pipe::Vertical,
            (Direction::North, Direction::East) => Pipe::NorthEast,
            (Direction::North, Direction::West) => Pipe::NorthWest,
            (Direction::East, Direction::South) => Pipe::SouthEast,
            (Direction::East, Direction::West) => Pipe::Horizontal,
            (Direction::South, Direction::West) => Pipe::SouthWest,
            _ => unreachable!(),
        };

        grid[start_pos.row][start_pos.col] = Some(starting_pipe);

        Ok(Grid {
            grid,
            start_pos,
            start_dir: directions,
        })
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthEast,
    SouthWest,
    Start,
}

impl Pipe {
    fn accessable_from(&self, dir: Direction) -> bool {
        match self {
            Pipe::Vertical => dir == Direction::North || dir == Direction::South,
            Pipe::Horizontal => dir == Direction::East || dir == Direction::West,
            Pipe::NorthEast => dir == Direction::West || dir == Direction::South,
            Pipe::NorthWest => dir == Direction::East || dir == Direction::South,
            Pipe::SouthEast => dir == Direction::West || dir == Direction::North,
            Pipe::SouthWest => dir == Direction::East || dir == Direction::North,
            Pipe::Start => todo!(),
        }
    }
    fn step(&self, from: Step) -> Step {
        match self {
            Pipe::Vertical => match from.dir {
                Direction::North => Step {
                    dir: from.dir,
                    pos: Pos {
                        row: from.pos.row - 1,
                        col: from.pos.col,
                    },
                },
                Direction::South => Step {
                    dir: from.dir,
                    pos: Pos {
                        row: from.pos.row + 1,
                        col: from.pos.col,
                    },
                },
                Direction::East => unreachable!(),
                Direction::West => unreachable!(),
            },
            Pipe::Horizontal => match from.dir {
                Direction::North => unreachable!(),
                Direction::South => unreachable!(),
                Direction::East => Step {
                    dir: from.dir,
                    pos: Pos {
                        row: from.pos.row,
                        col: from.pos.col + 1,
                    },
                },
                Direction::West => Step {
                    dir: from.dir,
                    pos: Pos {
                        row: from.pos.row,
                        col: from.pos.col - 1,
                    },
                },
            },
            Pipe::NorthEast => match from.dir {
                Direction::North => unreachable!(),
                Direction::East => unreachable!(),
                Direction::South => Step {
                    dir: Direction::East,
                    pos: Pos {
                        row: from.pos.row,
                        col: from.pos.col + 1,
                    },
                },
                Direction::West => Step {
                    dir: Direction::North,
                    pos: Pos {
                        row: from.pos.row - 1,
                        col: from.pos.col,
                    },
                },
            },
            Pipe::NorthWest => match from.dir {
                Direction::North => unreachable!(),
                Direction::East => Step {
                    dir: Direction::North,
                    pos: Pos {
                        row: from.pos.row - 1,
                        col: from.pos.col,
                    },
                },
                Direction::South => Step {
                    dir: Direction::West,
                    pos: Pos {
                        row: from.pos.row,
                        col: from.pos.col - 1,
                    },
                },
                Direction::West => unreachable!(),
            },
            Pipe::SouthEast => match from.dir {
                Direction::North => Step {
                    dir: Direction::East,
                    pos: Pos {
                        row: from.pos.row,
                        col: from.pos.col + 1,
                    },
                },
                Direction::East => unreachable!(),
                Direction::South => unreachable!(),
                Direction::West => Step {
                    dir: Direction::South,
                    pos: Pos {
                        row: from.pos.row + 1,
                        col: from.pos.col,
                    },
                },
            },
            Pipe::SouthWest => match from.dir {
                Direction::North => Step {
                    dir: Direction::West,
                    pos: Pos {
                        row: from.pos.row,
                        col: from.pos.col - 1,
                    },
                },
                Direction::East => Step {
                    dir: Direction::South,
                    pos: Pos {
                        row: from.pos.row + 1,
                        col: from.pos.col,
                    },
                },
                Direction::South => unreachable!(),
                Direction::West => unreachable!(),
            },
            Pipe::Start => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Step {
    dir: Direction,
    pos: Pos,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

const ALL_DIRECTIONS: [Direction; 4] = [
    Direction::North,
    Direction::East,
    Direction::South,
    Direction::West,
];

impl TryFrom<char> for Pipe {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Pipe::Vertical),
            '-' => Ok(Pipe::Horizontal),
            'L' => Ok(Pipe::NorthEast),
            'J' => Ok(Pipe::NorthWest),
            'F' => Ok(Pipe::SouthEast),
            '7' => Ok(Pipe::SouthWest),
            'S' => Ok(Pipe::Start),
            _ => Err(format!("Invalid pipe {}", value)),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 8);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT_2), 4);
        assert_eq!(
            super::solve_part_2(
                ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ..."
            ),
            8
        );
        assert_eq!(
            super::solve_part_2(
                "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L"
            ),
            10
        );
    }

    const TEST_INPUT: &'static str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";

    const TEST_INPUT_2: &'static str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
}
