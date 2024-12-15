use std::{fmt::Debug, str::FromStr};

use aoc_lib::grid::Pos;
use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    solve_1::<103, 101>(input)
}

#[allow(clippy::comparison_chain)]
fn solve_1<const ROWS: usize, const COLS: usize>(input: &str) -> usize {
    let mut robots = input
        .lines()
        .map(|line| {
            line.parse::<Robot<ROWS, COLS>>()
                .unwrap_or_else(|e| panic!("Invalid line {}\n{}", line, e))
        })
        //.filter(|r| r.pos == Pos::new(4, 2))
        .collect_vec();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.step();
        }
    }

    print(&robots);

    let mut quadrant_1 = 0;
    let mut quadrant_2 = 0;
    let mut quadrant_3 = 0;
    let mut quadrant_4 = 0;
    // (2n+1)/2 =  n + 1/2 = n
    for robot in robots {
        if robot.pos.row() < ROWS / 2 {
            match robot.pos.col() {
                col if col < COLS / 2 => quadrant_1 += 1,
                col if col > COLS / 2 => quadrant_2 += 1,
                _ => {}
            }
        } else if robot.pos.row() > ROWS / 2 {
            match robot.pos.col() {
                col if col < COLS / 2 => quadrant_3 += 1,
                col if col > COLS / 2 => quadrant_4 += 1,
                _ => {}
            }
        }
    }
    dbg!(quadrant_1) * dbg!(quadrant_2) * dbg!(quadrant_3) * dbg!(quadrant_4)
}

pub fn solve_part_2(input: &str) -> usize {
    let mut robots = input
        .lines()
        .map(|line| {
            line.parse::<Robot<103, 101>>()
                .unwrap_or_else(|e| panic!("Invalid line {}\n{}", line, e))
        })
        //.filter(|r| r.pos == Pos::new(4, 2))
        .collect_vec();

    for i in 1..10000 {
        for robot in robots.iter_mut() {
            robot.step();
        }
        let mut total_dist = 0;
        for (index, r1) in robots.iter().enumerate() {
            for r2 in robots.iter().skip(index + 1) {
                total_dist += r1.pos.distance_to(&r2.pos);
            }
        }
        if total_dist < 5000000 {
            print(&robots);
            return i;
        }
    }
    panic!("No christmas tree found");
}

fn print<const R: usize, const C: usize>(robots: &[Robot<R, C>]) {
    for row in 0..R {
        for col in 0..C {
            let r = robots
                .iter()
                .filter(|r| r.pos == Pos::new(row, col))
                .count();
            if r > 0 {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[derive(Debug)]
struct Robot<const ROWS: usize, const COLS: usize> {
    pos: Pos,
    velocity: (isize, isize),
}
impl<const ROWS: usize, const COLS: usize> Robot<ROWS, COLS> {
    fn step(&mut self) {
        let new_row = (self.pos.row() as isize) + self.velocity.0;
        let new_row = if new_row < 0 {
            (ROWS as isize + new_row) as usize
        } else {
            new_row as usize % ROWS
        };
        let new_col = (self.pos.col() as isize) + self.velocity.1;
        let new_col = if new_col < 0 {
            (COLS as isize + new_col) as usize
        } else {
            new_col as usize % COLS
        };

        self.pos = Pos::new(new_row, new_col)
    }
}

impl<const ROWS: usize, const COLS: usize> FromStr for Robot<ROWS, COLS> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos_str, vel_str) = s
            .splitn(2, " ")
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| format!("Invalid {}", s))?;
        let pos_comma = pos_str.find(',').expect("");
        let pos = Pos::new(
            parse_number(&pos_str[pos_comma + 1..])?,
            parse_number(&pos_str[2..pos_comma])?,
        );
        let vel_comma = vel_str.find(',').expect("");
        let velocity = (
            parse_number(&vel_str[vel_comma + 1..])?,
            parse_number(&vel_str[2..vel_comma])?,
        );

        Ok(Robot { pos, velocity })
    }
}

fn parse_number<T>(s: &str) -> Result<T, String>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    s.parse::<T>()
        .map_err(|e| format!("Not a number: {}\n{:?}", s, e))
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_1::<7, 11>(INPUT), 12);
    }

    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
}
