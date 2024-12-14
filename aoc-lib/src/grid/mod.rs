use std::{fmt::Debug, iter::Map, str::FromStr};

use itertools::Itertools;

pub struct Grid<T> {
    data: Vec<Vec<T>>,
}

impl<T> Debug for Grid<T> where T: Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.data {
            for col in row {
                write!(f, "{:?}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T> Clone for Grid<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
        }
    }
}

impl<T> Grid<T> {
    pub fn insert(&mut self, pos: Pos, value: T) {
        self.data[pos.0][pos.1] = value;
    }

    pub fn get(&self, pos: Pos) -> Option<&T> {
        if let Some(row) = self.data.get(pos.row()) {
            row.get(pos.col())
        } else {
            None
        }
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
            row_step: -1,
            col_step: 0,
        }
    }
    pub fn down(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            row_step: 1,
            col_step: 0,
        }
    }
    pub fn left(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            row_step: 0,
            col_step: -1,
        }
    }
    pub fn right(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            row_step: 0,
            col_step: 1,
        }
    }

    pub fn up_right(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            row_step: -1,
            col_step: 1,
        }
    }

    pub fn down_right(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            row_step: 1,
            col_step: 1,
        }
    }

    pub fn up_left(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            row_step: -1,
            col_step: -1,
        }
    }
    pub fn down_left(&self, pos: Pos) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            row_step: 1,
            col_step: -1,
        }
    }

    pub fn step(&self, pos: Pos, step_row: isize, step_col: isize) -> PathIterator<'_, T> {
        PathIterator {
            data: &self.data,
            pos,
            row_step: step_row,
            col_step: step_col,
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Hash, Debug)]
pub struct Pos(usize, usize);

impl Pos {
    pub fn new(row: usize, col: usize) -> Self {
        Self(row, col)
    }
    pub fn row(&self) -> usize {
        self.0
    }
    pub fn col(&self) -> usize {
        self.1
    }
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

    pub fn distance_to(&self, other: &Pos) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

pub struct RowWiseIter<'a, T> {
    data: &'a Vec<Vec<T>>,
    row: usize,
    col: usize,
}

impl<'a, T> Iterator for RowWiseIter<'a, T>
where
    T: Clone,
{
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

trait ParseChar {
    fn parse(c: char) -> Self;
}

impl ParseChar for char {
    fn parse(c: char) -> Self {
        c
    }
}

impl ParseChar for u32 {
    fn parse(c: char) -> Self {
        c as u32 - '0' as u32
    }
}

impl<T> FromStr for Grid<T>
where
    T: ParseChar,
{
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let puzzle = input
            .lines()
            .map(|line| line.chars().map(|c| ParseChar::parse(c)).collect_vec())
            .collect_vec();
        Ok(Self { data: puzzle })
    }
}

pub struct PathIterator<'a, T> {
    data: &'a Vec<Vec<T>>,
    pos: Pos,
    row_step: isize,
    col_step: isize,
}

impl<'a, T> Iterator for PathIterator<'a, T>
where
    T: Clone,
{
    type Item = Point< T>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(row) = self.data.get(self.pos.0) {
            if let Some(next) = row.get(self.pos.1) {
                let next_pos = Pos(
                    self.pos.0.wrapping_add(self.row_step as usize),
                    self.pos.1.wrapping_add(self.col_step as usize),
                );
                let point = Point {
                    pos: self.pos,
                    value: next.clone(),
                };
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
    pub fn values(self) -> Map<PathIterator<'a, T>, impl FnMut(Point<T>) -> T>
    where
        T: Clone,
    {
        self.map(|p| p.value)
    }
}
