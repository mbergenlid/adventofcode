use std::str::FromStr;

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Invalid input");

    let mut result = 0;
    for point in grid.iter() {
        if point.value != 'X' {
            continue;
        }
        //Look left
        if grid.right(point.pos).starts_with("XMAS".chars()) {
            result += 1;
        }
        if grid.left(point.pos).starts_with("XMAS".chars()) {
            result += 1;
        }
        //Look up
        if grid.up(point.pos).starts_with("XMAS".chars()) {
            result += 1;
        }
        //Look down
        if grid.down(point.pos).starts_with("XMAS".chars()) {
            result += 1;
        }
        if grid.up_left(point.pos).starts_with("XMAS".chars()) {
            result += 1;
        }
        if grid.up_right(point.pos).starts_with("XMAS".chars()) {
            result += 1;
        }
        if grid.down_left(point.pos).starts_with("XMAS".chars()) {
            result += 1;
        }
        if grid.down_right(point.pos).starts_with("XMAS".chars()) {
            result += 1;
        }
    }
    result
}

pub fn solve_part_2(input: &str) -> usize {
    let grid = input.parse::<Grid>().expect("Invalid input");

    let mut result = 0;
    for point in grid.iter() {
        if point.value != 'A' {
            continue;
        }

        let up_left = point.pos.up_left();
        let down_left = point.pos.down_left();
        if (grid.down_right(up_left).starts_with("MAS".chars())
            || grid.down_right(up_left).starts_with("SAM".chars()))
            && (grid.up_right(down_left).starts_with("MAS".chars())
                || grid.up_right(down_left).starts_with("SAM".chars()))
        {
            result += 1;
        }
    }
    result
}

impl<T> StartsWithIter for T
where
    T: Iterator,
    T::Item: Eq,
{
    fn starts_with(mut self, other: impl Iterator<Item = Self::Item>) -> bool {
        for x in other {
            if let Some(self_val) = self.next() {
                if x != self_val {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}

trait StartsWithIter: Iterator {
    fn starts_with(self, other: impl Iterator<Item = Self::Item>) -> bool;
}

struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    fn iter(&self) -> RowWiseIter<'_> {
        RowWiseIter {
            data: &self.data,
            row: 0,
            col: 0,
        }
    }

    fn up(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up,
        }
    }
    fn down(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down,
        }
    }
    fn left(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::left,
        }
    }
    fn right(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::right,
        }
    }

    fn up_right(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up_right,
        }
    }

    fn down_right(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down_right, //Box::new(|r, c| (r+1, c+1)),
        }
    }

    fn up_left(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up_left,
        }
    }
    fn down_left(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down_left, //Box::new(|r, c| (r+1, c+1)),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Pos(usize, usize);

impl Pos {
    fn up(self) -> Pos {
        Pos(self.0.wrapping_sub(1), self.1)
    }
    fn down(self) -> Pos {
        Pos(self.0 + 1, self.1)
    }

    fn left(self) -> Pos {
        Pos(self.0, self.1.wrapping_sub(1))
    }
    fn right(self) -> Pos {
        Pos(self.0, self.1 + 1)
    }

    fn down_right(self) -> Self {
        Pos(self.0 + 1, self.1 + 1)
    }

    fn up_right(self) -> Self {
        Pos(self.0.wrapping_sub(1), self.1 + 1)
    }

    fn up_left(self) -> Self {
        Pos(self.0.wrapping_sub(1), self.1.wrapping_sub(1))
    }

    fn down_left(self) -> Self {
        Self(self.0 + 1, self.1.wrapping_sub(1))
    }
}

struct RowWiseIter<'a> {
    data: &'a Vec<Vec<char>>,
    row: usize,
    col: usize,
}

impl<'a> Iterator for RowWiseIter<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(grid_row) = self.data.get(self.row) {
            if let Some(col_value) = grid_row.get(self.col) {
                let res = Some(Point {
                    pos: Pos(self.row, self.col),
                    value: *col_value,
                });
                self.col += 1;
                res
            } else {
                self.col = 0;
                self.row += 1;
                self.next()
            }
        } else {
            None
        }
    }
}

struct Point {
    pos: Pos,
    value: char,
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let puzzle = input
            .lines()
            .map(|line| line.chars().collect_vec())
            .collect_vec();
        Ok(Self { data: puzzle })
    }
}

struct DiagonalIterator<'a> {
    data: &'a Vec<Vec<char>>,
    pos: Pos,
    next_fn: &'static dyn Fn(Pos) -> Pos,
}


impl<'a> Iterator for DiagonalIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row) = self.data.get(self.pos.0) {
            if let Some(next) = row.get(self.pos.1) {
                let next_pos = (self.next_fn)(self.pos);
                self.pos = next_pos;
                Some(*next)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 18);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 9);
    }

    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
}
