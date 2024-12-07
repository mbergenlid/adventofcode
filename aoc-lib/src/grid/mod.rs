use std::{iter::Map, str::FromStr};

use itertools::Itertools;


pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl <T> Clone for Grid<T> where T: Clone {
    fn clone(&self) -> Self {
        Self { data: self.data.clone() }
    }
}

impl<T> Grid<T> {

    pub fn insert(&mut self, pos: Pos, value: T) {
        self.data[pos.0][pos.1] = value;
    }

    pub fn iter(&self) -> RowWiseIter<'_, T> {
        RowWiseIter {
            data: &self.data,
            row: 0,
            col: 0,
        }
    }

    pub fn up(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up,
        }
    }
    pub fn down(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down,
        }
    }
    pub fn left(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::left,
        }
    }
    pub fn right(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::right,
        }
    }

    pub fn up_right(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up_right,
        }
    }

    pub fn down_right(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down_right, //Box::new(|r, c| (r+1, c+1)),
        }
    }

    pub fn up_left(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::up_left,
        }
    }
    pub fn down_left(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            next_fn: &Pos::down_left, //Box::new(|r, c| (r+1, c+1)),
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash)]
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

pub struct RowWiseIter<'a, T> {
    data: &'a Vec<Vec<T>>,
    row: usize,
    col: usize,
}

impl<'a, T> Iterator for RowWiseIter<'a, T> where T: Clone {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(grid_row) = self.data.get(self.row) {
            if let Some(col_value) = grid_row.get(self.col) {
                let res = Some(Point {
                    pos: Pos(self.row, self.col),
                    value: col_value.clone(),
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

pub struct Point<T> {
    pub pos: Pos,
    pub value: T,
}

impl<T> FromStr for Grid<T> where T: From<char> {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let puzzle = input
            .lines()
            .map(|line| line.chars().map(|c| c.into()).collect_vec())
            .collect_vec();
        Ok(Self { data: puzzle })
    }
}

pub struct PathIterator<'a, T> {
    data: &'a Vec<Vec<T>>,
    pos: Pos,
    next_fn: &'static dyn Fn(Pos) -> Pos,
}


impl<'a, T> Iterator for PathIterator<'a, T> where T: Clone {
    type Item = Point<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row) = self.data.get(self.pos.0) {
            if let Some(next) = row.get(self.pos.1) {
                let next_pos = (self.next_fn)(self.pos);
                let point = Point { pos: self.pos, value: next.clone() }; 
                self.pos = next_pos;
                Some(point)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a, T> PathIterator<'a, T> {
    pub fn values(self) -> Map<PathIterator<'a, T>, impl FnMut(Point<T>) -> T> where T: Clone {
        self.map(|p| p.value)
    }
}
