use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    solve(&grid, EnergyTile::default())
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut max_energy = 0;
    for row in 0..grid.len() {
        max_energy = max_energy.max(solve(
            &grid,
            EnergyTile {
                row,
                col: 0,
                dir: Direction::Right,
            },
        ));
        max_energy = max_energy.max(solve(
            &grid,
            EnergyTile {
                row,
                col: grid.len() - 1,
                dir: Direction::Left,
            },
        ));
    }
    for col in 0..grid[0].len() {
        max_energy = max_energy.max(solve(
            &grid,
            EnergyTile {
                row: 0,
                col,
                dir: Direction::Down,
            },
        ));
        max_energy = max_energy.max(solve(
            &grid,
            EnergyTile {
                row: grid[0].len(),
                col,
                dir: Direction::Up,
            },
        ));
    }
    max_energy
}

fn solve(grid: &Vec<Vec<char>>, start_tile: EnergyTile) -> usize {
    let height = grid.len();
    let width = grid[0].len();
    let mut branches: Vec<EnergyTile> = Vec::new();
    branches.push(start_tile);
    let mut energized_tiles: HashSet<EnergyTile> = HashSet::new();
    while let Some(mut current) = branches.pop() {
        loop {
            if !current.in_bounds(width, height) {
                break;
            }
            energized_tiles.insert(current.clone());
            let tile = grid[current.row][current.col];

            let next = match tile {
                '.' => current.step(),
                '/' => current.reflect_forward(),
                '\\' => current.reflect_backward(),
                '|' => current.split_vertical(&mut branches),
                '-' => current.split_horizontal(&mut branches),
                _ => unreachable!(),
            };
            if energized_tiles.contains(&next) || !next.in_bounds(width, height) {
                break;
            }
            current = next;
        }
    }
    energized_tiles
        .into_iter()
        .map(|t| (t.row, t.col))
        .collect::<HashSet<_>>()
        .len()
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
struct EnergyTile {
    row: usize,
    col: usize,
    dir: Direction,
}

impl Default for EnergyTile {
    fn default() -> Self {
        Self {
            row: Default::default(),
            col: Default::default(),
            dir: Direction::Right,
        }
    }
}

impl EnergyTile {
    fn step(self) -> EnergyTile {
        match self.dir {
            Direction::Up => EnergyTile {
                row: self.row.wrapping_sub(1),
                col: self.col,
                dir: self.dir,
            },
            Direction::Right => EnergyTile {
                row: self.row,
                col: self.col + 1,
                dir: self.dir,
            },
            Direction::Down => EnergyTile {
                row: self.row + 1,
                col: self.col,
                dir: self.dir,
            },
            Direction::Left => EnergyTile {
                row: self.row,
                col: self.col.wrapping_sub(1),
                dir: self.dir,
            },
        }
    }
    fn reflect_forward(self) -> EnergyTile {
        match self.dir {
            Direction::Up => self.turn(Direction::Right).step(),
            Direction::Right => self.turn(Direction::Up).step(),
            Direction::Down => self.turn(Direction::Left).step(),
            Direction::Left => self.turn(Direction::Down).step(),
        }
    }
    fn reflect_backward(self) -> EnergyTile {
        match self.dir {
            Direction::Up => self.turn(Direction::Left).step(),
            Direction::Right => self.turn(Direction::Down).step(),
            Direction::Down => self.turn(Direction::Right).step(),
            Direction::Left => self.turn(Direction::Up).step(),
        }
    }

    fn split_vertical(self, extra_branches: &mut Vec<EnergyTile>) -> EnergyTile {
        if self.dir == Direction::Left || self.dir == Direction::Right {
            extra_branches.push(self.clone().turn(Direction::Down).step());
            self.turn(Direction::Up).step()
        } else {
            self.step()
        }
    }

    fn split_horizontal(self, extra_branches: &mut Vec<EnergyTile>) -> EnergyTile {
        if self.dir == Direction::Up || self.dir == Direction::Down {
            extra_branches.push(self.clone().turn(Direction::Right).step());
            self.turn(Direction::Left).step()
        } else {
            self.step()
        }
    }

    fn in_bounds(&self, width: usize, height: usize) -> bool {
        self.row < height && self.col < width
    }

    fn turn(mut self, dir: Direction) -> Self {
        self.dir = dir;
        return self;
    }
}

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 46);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 51);
    }

    const TEST_INPUT: &'static str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
}
