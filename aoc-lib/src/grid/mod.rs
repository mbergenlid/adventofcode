use std::str::FromStr;

use itertools::Itertools;


pub struct Grid {
    data: Vec<Vec<char>>,
}

impl Grid {
    pub fn iter(&self) -> RowWiseIter<'_> {
        RowWiseIter {
            data: &self.data,
            row: 0,
            col: 0,
        }
    }

    pub fn up(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up,
        }
    }
    pub fn down(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down,
        }
    }
    pub fn left(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::left,
        }
    }
    pub fn right(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::right,
        }
    }

    pub fn up_right(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up_right,
        }
    }

    pub fn down_right(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down_right, //Box::new(|r, c| (r+1, c+1)),
        }
    }

    pub fn up_left(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up_left,
        }
    }
    pub fn down_left(&self, pos: Pos) -> DiagonalIterator<'_> {
        DiagonalIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down_left, //Box::new(|r, c| (r+1, c+1)),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone)]
pub struct Pos(usize, usize);

impl Pos {
    pub fn up(self) -> Pos {
        Pos(self.0.wrapping_sub(1), self.1)
    }
    pub fn down(self) -> Pos {
        Pos(self.0 + 1, self.1)
    }

    pub fn left(self) -> Pos {
        Pos(self.0, self.1.wrapping_sub(1))
    }
    pub fn right(self) -> Pos {
        Pos(self.0, self.1 + 1)
    }

    pub fn down_right(self) -> Self {
        Pos(self.0 + 1, self.1 + 1)
    }

    pub fn up_right(self) -> Self {
        Pos(self.0.wrapping_sub(1), self.1 + 1)
    }

    pub fn up_left(self) -> Self {
        Pos(self.0.wrapping_sub(1), self.1.wrapping_sub(1))
    }

    pub fn down_left(self) -> Self {
        Self(self.0 + 1, self.1.wrapping_sub(1))
    }
}

pub struct RowWiseIter<'a> {
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

pub struct Point {
    pub pos: Pos,
    pub value: char,
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

pub struct DiagonalIterator<'a> {
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
