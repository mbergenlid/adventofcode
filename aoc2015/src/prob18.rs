use std::collections::{HashMap, HashSet};

pub fn solve_part_1() -> usize {
    let mut grid: HashSet<Position> = HashSet::new();
    let mut current_row = 0;
    let mut current_col = 0;
    for c in input().chars() {
        match c {
            '#' => {
                grid.insert(Position::new(current_row, current_col));
                current_col += 1;
            }
            '.' => {
                current_col += 1;
            }
            '\n' => {
                current_row += 1;
                current_col = 0;
            }
            _ => panic!("Unknown character {}", c),
        }
    }

    for _ in 0..100 {
        let mut changes: HashMap<Position, u32> = HashMap::new();
        for tile in grid.iter() {
            for neighbour in tile.neighbours() {
                if let Some(count) = changes.get_mut(&neighbour) {
                    *count += 1;
                } else {
                    changes.insert(neighbour, 1);
                }
            }
        }
        grid = changes
            .into_iter()
            .filter(|(pos, count)| *count == 3 || (*count == 2 && grid.contains(pos)))
            .map(|(pos, _)| pos)
            .collect();
    }
    grid.len()
}

pub fn solve_part_2() -> usize {
    let mut grid: HashSet<Position> = HashSet::new();
    let mut current_row = 0;
    let mut current_col = 0;
    for c in input().chars() {
        match c {
            '#' => {
                grid.insert(Position::new(current_row, current_col));
                current_col += 1;
            }
            '.' => {
                current_col += 1;
            }
            '\n' => {
                current_row += 1;
                current_col = 0;
            }
            _ => panic!("Unknown character {}", c),
        }
    }
    grid.insert(Position::new(0, 0));
    grid.insert(Position::new(0, 99));
    grid.insert(Position::new(99, 0));
    grid.insert(Position::new(99, 99));

    for _ in 0..100 {
        let mut changes: HashMap<Position, u32> = HashMap::new();
        for tile in grid.iter() {
            for neighbour in tile.neighbours() {
                if let Some(count) = changes.get_mut(&neighbour) {
                    *count += 1;
                } else {
                    changes.insert(neighbour, 1);
                }
            }
        }
        grid = changes
            .into_iter()
            .filter(|(pos, count)| {
                *count == 3 || (*count == 2 && grid.contains(pos))
            })
            .map(|(pos, _)| pos)
            .collect();
        grid.insert(Position::new(0, 0));
        grid.insert(Position::new(0, 99));
        grid.insert(Position::new(99, 0));
        grid.insert(Position::new(99, 99));
    }
    grid.len()
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    row: u32,
    col: u32,
}

impl Position {
    fn new(row: u32, col: u32) -> Position {
        Position { row, col }
    }
    fn neighbours(&self) -> impl Iterator<Item = Position> {
        let mut neighbours = Vec::new();
        if self.row > 0 {
            neighbours.push(Position::new(self.row - 1, self.col));
            if self.col > 0 {
                neighbours.push(Position::new(self.row - 1, self.col - 1));
            }
            if self.col < 99 {
                neighbours.push(Position::new(self.row - 1, self.col + 1));
            }
        }
        if self.col < 99 {
            neighbours.push(Position::new(self.row, self.col + 1));
        }
        if self.row < 99 {
            neighbours.push(Position::new(self.row + 1, self.col));
            if self.col > 0 {
                neighbours.push(Position::new(self.row + 1, self.col - 1));
            }
            if self.col < 99 {
                neighbours.push(Position::new(self.row + 1, self.col + 1));
            }
        }
        if self.col > 0 {
            neighbours.push(Position::new(self.row, self.col - 1));
        }
        neighbours.into_iter()
    }
}

fn input() -> String {
    String::from("#..####.##..#...#..#...#...###.#.#.#..#....#.##..#...##...#..#.....##..#####....#.##..##....##.#....
.#..#..#..#.###...##..#.##.....#...#..##....#####.##............####.#..######..#.#.##.#...#..#...##
#.....##.##.##.#..##.#..###...#.#.#..##..###.####.####.#.####.#...##.#..###.........#.###...#....###
#.###..#######..##..#.....##.#.#.###.#.##..#.##..##.##.#.##...###.#...#.#####.#.##..#.#####..#.#####
#.##.##.###.##..###.#.##.##...##.#.#..##..###.########.#.####..####...#####...#..#...##....##.##.##.
..#.#.#.#..#.#.###....###...#...#.##..####.###.....#.####.###.###.#......#.#.###..#..#.#....#.#####.
...#.###.#....#.###...#.#.#...#...#.#####....#....#...#####..#..#.#..######..#.##.#.##.#..###.#...##
.###...#...#.#..#.#.####.#...#.....##...###.#....#..##.###....#.##....###..#.#####...###.#.##.####..
#.#....##.#.....#####.#.##..#######.#.####..###.##.#####.##.#...###...#.#...###..#...#.#.###.###.###
...##.##.....##..#.##...#.#...#...#.#####.#...#.#.#.#####.##.#...#.#..##.##..#...#....####..###.###.
#..#....######...#...###.#....#####....#.#.#....#....#.#######.#####..#....#....#.##..#.##.###..#...
#####.#.######.#.#####.#..##..##..####..#....#...#######....##..##.#..###..###.###..###...#...######
#...##..##...###....##..##.##..#.#.#.#....##.#.......###..###..###...###..##.##.##.#.#.#..#.#..#..#.
..###....##.###..#.#..########...###...##..#######....##..###..#####.##.#....###..##.##.##.#...##.#.
###..#.#..#.#.##.##...##.....#..###.#..##.##.#....##.#.######..##..#.#.##.###...#..####...#.#..#.###
.######....#..##..#.####.##..#.#..#.#..#....#..##.#..#.#...####..#....#.####.#.###.#...####.#...#.#.
#.######.##..###.###..#..###.#...#..#...#...###.##....#.#......#...#.##.#.###..#.#####.#.#..###..#.#
...#..#...####..###.########.....###.###.#..##.##....######..#..#.....#.##.##.#..##..#..##...#..#..#
#..#..##..#.#.########.##.#.####..#.#####.#.###.##....###..##..#.#.###..#.##..##.##.####...######.##
.######.###....#...##...#..#....##..#.#...###.######.##...#....##.##.#.#.##..#...###.###.#....#..##.
####.#.##..##.##.###...#.###.##..##....###..####.##..#.#.##..###.#..##...####...#..####.#.#..##...#.
.#.#..#.....##...#..#...#.#...#.#.##..#....#..#......#####.#######....#.#..#..###..##.#.########..##
.##.#..#..##..#..####.#...####...#...#..##.#..###.#..######..#.#...###.##...#..#####..##.#..##.#.##.
.###..##.##.##....###.###..#.#...##.#.#...#.#######.####..#..###.#######.#...#.#...#.##...#..####..#
##.########..#..#....#.###..##.##.#.##.#..#......####..##.##.#..####..#####..#.....#####.###..#.#.#.
.#..####..##.#.#..#####.##..#..#.#....#.#####.#####...######........##.##..##.#.#.###..#.#.#.#..##.#
.##..##..#.######..###....#.#.###.#........#..###..#.########.....#.##...#.....#..#...##...#..#.###.
##.##.#..####....####.#######.....#.#.#...#.######.#.....####.####...###..####.##.##....###..#..#...
#.#..####...#......#...###...##....##.#######..#.###.#...###.##.##...####..#.####..#......##..#####.
.#.#...##...#....#.####.##.....#....#.#.#######..###.#.....#.....####...##...#.#.##.####..##.###.#.#
####.#.#.####...#...####.#.....#.#######.#.......####......###..###.#...######..#.##.#.##..#..##..##
..##.###..#..####..####.......######.##..#.....##.##...##.##......#.###..###...#.##.#####.#.######.#
.###..####.###..#..#.......#.##...##...##.######.....#..####.#......#.#...#...#...###...#.#.##..####
.####....##.##.#.....##.###.####.#.......#.......#.#..#.#.#.....###.#.#####.#..#.#.#####.#####.###.#
.##.#.###.#####..#..#....###.#.#.#..#..###..##..####..##.###....#..####.####.#..###.#..######.######
####.#.....##..###....#.....#.##.#.##..##..########.#####..###.####....##.....######.#.#.##.......#.
#.#.##.....#.....##.###.#..#.##.##....#..##....##.#.###.##.#..#..##.##.###.#..##.###...##..###.#####
#.###.#.#.#.#.#.#.#...#..#.###..####.##...#..####.###....#..#..##.#....####..##.##....#.#.##.##....#
...######....#..####...#.#..#.#.#..#.##.#.#.......#..#......##..#...#..#..##...##.#...#.#.#...##.##.
.#####..#...####....#..###..##....#####..###.#.#...###..###.###..##...#......#...#...#.#.#...#.##..#
......#####.#...#.#.#.##..#.###..##..#.#...###..###....##..#####..#######.#..#.###....###...##.#..#.
..##.########.##..#....##.#...##.##.#.#..#.##..#.#.#.##....#.#.#.#.##....##....#....#####.##..#.##.#
####...#....##.#.###......##.##.#..##...#..#####..#.#....##..#####...#.#.##...#.####.####..##.######
.##.###.##.#...#.#....###.#######...##...##..#..##.###.#.####..#..###......#.#.##.#.#....#..##...#..
.#.###.#.###.###.#.##.#..#......####.##...#..##.#..####.....#...#.###.##.##.#..#.##..#.###......#..#
...##.####......#.#.#..###..#....###....#.##.#####..#..#..#...#.#.###...#.#.#.##....###.####..###.#.
##..#.#.#.#....####...#.##.###..####....#..#####.######..#.##.##..#####.#.....#.#...##.#.##.##.#.#..
#..##.#.#.#.###.#.#.###...#.#...##..#..#.#.#.##..###...#..##.#..#.#.#..#.....#.######.#.###..###.#..
....#.#.##.###.##...#.##.#....#..##.#..##...#...#.##.####...##..####.#.........#..##..#...#...##.#..
.##.......##...###.##.#.##.###.##.#..#..#..####...#...#....#####...###..##..#..#..##...#....#..#####
..####..#...#...#..###....##.#.#####..#..#.....#......#...#.......##....####...##....##.##.#.#####.#
##.#.#.#..##..##..#.####.##.##.###.#...###.#....#.....#.###...#######..###.####.###.####.##...##.#..
..#.#...##.#....#..#..##.####.....#.#.#...#..#..###.#..###.#####.#.#####.#.#.#.#.###.##.###..#....##
.###.#...#....###..#...####....####..#.##..#..##.###..#.#.#.#..#...###.#.#...#......#...#.##.##.#...
..####.####.##.#.##....#...##....#..#....#..###..#...#..###.#####.....#####..##.#.#.#.#.#.##.####...
...##.#.##.####..##.###..#.#.#.#.#.#.#..###...#.##..#.####.##...#.#.##......###..#...###....#.#.###.
##...##..#.#.##..#.#.#....#.####.......#.#.#######.#..#....#.###.#...###.##....###.#.#..#.#.##.####.
...##.......######.....##....#...#..#.##.###.#..#.##.###.#.###.#.#.#...#.#...##.##.##..#.##########.
###..#....#.#.....#....###.#...##.......##.#.#..#.#...########......###..##.#..#..####.##..####...#.
......##.###.#.###.....#..#...#.#......##....#....#........#..#...##.##.....#...##.##.........##....
.##.##.#.#...#....######..##....##..##.#.#.##.#.##..##...#..###......##......#.#....#.#.#.......###.
.......#.##..##.#...#.##..#..#####.#..#.######.........###.#####.####.#...##...........##...##..####
#......#.#..#...#...##..#.#.###.##.##.#.#..#.###.##.#..###..#.###..#...###.##..###..#...#..###...#..
####.##..#####..####.#...#..#..###..##.#.#...#...#...#.##.####.##.###....###...#.#.#..####.######.##
.....#..####...#.#.#.####..####..##.###......#.....########.#...#.#..#..#...#.###..##.#####..###.###
.#######.#.##..###.#...###.#####............##.###...#.##.#.##..##.#.#..#.######..######..#..#..####
...##..#.####...#..#.#.##.#....#.####..#..###.###..#.#...#....##.##.#......##..##..#.#.#.###..#..#..
........#...#.##.#.#..#....####....#.##...###..####...###.#.#..######..###..##.#####.###.###.#.#...#
##......##.#..###.####.##.#.###.#.......#.##..####..#.###.##..##..##...##...#.###...#.#..#..#.#####.
##..#.#.....##.####.#..##.#.##.#.#...#...#.#...####.#.#.##...##....##.###..###.####.#...#.###..#####
.#####.####.####.####.#.##.##......###....###.####...###...#...#..#.##.#.#####.###..##.#..###...##..
.#...#..##...##...#....#.#.#..##..#.##..#.###.#.###..###.#.#.###.#....#######.####.##..#..#...####..
..##.##..#.##..#.#.###..#.##.########...####.#.###.##..#..###.###...##..##.#..#.######.##.#....###.#
##.#####.###.##.#.##.##.##.###..##..##..#.#.#.#.####..#......#.#.#.#.#.#.##...#####.####...#.#...#.#
.#..###..##.#####.#.##.#..##...##..##...#####.#.####..#...##.....######.#.#...##.#..#######.###.###.
#.#..##.#.#####.#.#.....###.###.#..##.#####....#.###.##.##.#.#..##..#.#....#######.###.#.#.....#.###
....###...#.###.####....###.....##....#####.##.###.###.##.##.##.#..###..######...####.#.#..####..#..
###.....#..####..#.####..#..#...##.##..##.######.####.....#...##....#..#.##.#####..###.##.#.####...#
.##.##.#...#..####...##.##.###...#...#..#.#.#####.....####...#.#.#..#.####...####.#...###.#......###
###.##....#.#.#...#.###....####..##...##.##.##.#..#...####..#..#..##...#####.####.####...##.#..###.#
..####.....##..###.#.#.###.########..#...#.##..#.#.#.......#.##.#..#...####.##.#..#.######..#.#...#.
#.#.##.#.#.##.#....##......##......#######.#..#.##...##..#.#.###...#.#..#..###...#..###.....##.....#
..#.##.#.##.#.##..##.....#.#..#.#..#...##..#..#.#....###.#####....####.####..#####.##.###...#..###.#
#....#.###..#..########.###..#.#.#.##...##.#..##.###..#..#..#.#.##..###...###.#.##..#.##.#..#.#.####
#.......#######......#...#...##.##...###.#....##.#..#....####.#.##.###...#.#####...##.###........##.
.##.####.....###.##......####.###.########..#.####..#.##.#.####.....#...#.##....#######.##..#......#
#.#.##.##....##..##.#.###..#.##.#..#..#.#..##.....###..###.##.##.####.##.#.#.##...####..#.#..##.#.#.
...##.#.#.#...###.#.......#.#.....#.#...##....##.##.##.####...#.#..#..#..#.#.##.#..#.#.#....###..#.#
....#.#.###.#####.##..###..##..#...#.##.#......##.####.#..####.#.##..####.#.#...##..#####..##.#.#...
..###.#.##..#....#..#.#.....##.#####..##....#.#...#.##..##.#.#..#...##.##..##..##....#...#..#..#..##
##.#.##.#...#.###.##.##.##.##..##.##...#..##.#..#######.#..#...#.#.##..#....##.#..####.###........#.
.##.#..#.....#####..##.#.#.#.#..###.#######.###.###....##....#.#.#.###....###.#..#.#....#.#..###...#
...###.#.#.###..#...#..###.######..##.#.#..#...####.#####.##..#..###...#..#..#..###..##.#.#...#.###.
#......#.#..#..##.##.#.##.#.###.#.##.#.#..#....#.##..#..##..##.#.#.#....##.###.###.####.#.#####...##
...#.##..#.######.......#.#.###.....#####....##.#.#.###........#.#.###.#.#########.##.##.#..##..#...
##..###..###....####.##.##..##.###....####..##...####.####..####..###.####..##.#...###.#####.##.##.#
###...##.#.#.#####..#..#####...##.#...#.#.###.#..##..###.##.#.#.....####.##.#..##.###.#...##.##...##
...#.#.##.##..##....#..#.#####.##.###..#.#.#........####.###.##....##....####..#.#....#.#.#.###..#.#
..#.#.#.#.###...#....##..######.##....#.#.##..###..#.#.###..#.##..#.#.###......#..#..#.####..#...##.
.....####.#.....###.#.##.#..##.#..###.#####.#..##...###.#..###..#..##....###.#..##.#..#.##.#..#...##")
}
