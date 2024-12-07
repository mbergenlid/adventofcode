use std::{collections::HashSet, iter::Peekable};

use aoc_lib::grid::{DiagonalIterator, Grid, Point, Pos};

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Invalid input");
    GuardIter::start(&grid).map(|(p, _)| p.pos).collect::<HashSet<_>>().len()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().expect("Invalid input");

    let mut iter = GuardIter::start(&grid).peekable();

    let start_pos = iter.peek().expect("No start pos found").0.pos;
    let visited = iter.map(|(p, _)| p.pos).collect::<HashSet<_>>();

    let mut result = 0;
    for pos in visited {
        if pos == start_pos {
            continue;
        }

        grid.insert(pos, '#');
        if has_cycle(&grid) {
            result += 1;
        }
        grid.insert(pos, '.');
    }

    result
}

struct GuardIter<'a> {
    grid: &'a Grid,
    current_iter: Peekable<DiagonalIterator<'a>>,
    current_pos: Pos,
    current_dir: Direction,
}

impl<'a> GuardIter<'a> {

    fn start(grid: &'a Grid) -> Self {
        let start_pos = grid
            .iter()
            .find(|point| point.value == '^')
            .expect("No start pos found");

        Self {
            grid,
            current_iter: grid.up(start_pos.pos).peekable(),
            current_pos: start_pos.pos,
            current_dir: Direction::Up,
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn turn(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

impl<'a> Iterator for GuardIter<'a> {
    type Item = (Point, Direction);

    fn next(&mut self) -> Option<Self::Item> {
        let directions = [Grid::up, Grid::right, Grid::down, Grid::left];

        let iter = &mut self.current_iter;

        if let Some(peek) = iter.peek() {
            if peek.value == '#' {
                self.current_dir = self.current_dir.turn();
                let iter = (directions[self.current_dir as usize])(self.grid, self.current_pos).peekable();
                self.current_iter = iter;
                return self.next();
            }

            let next = iter.next().expect("We have already peeked at it");
            self.current_pos = next.pos;
            Some((next, self.current_dir))
        } else {
            //Outside
            None
        }
    }
}

fn has_cycle(grid: &Grid) -> bool {
    let mut visited = HashSet::new();
    for (p, dir) in GuardIter::start(grid) {
        let pos = (p.pos, dir);
        if visited.contains(&pos) {
            return true;
        }

        visited.insert(pos);
    }
    false
}


#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 41);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 6);
    }

    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
}
