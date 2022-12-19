use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::{max, min};
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn down(&self) -> Self {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left_down(&self) -> Self {
        Point {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    fn right_down(&self) -> Self {
        Point {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

#[derive(Debug)]
struct Path(Vec<Point>);

impl Path {
    fn parse(s: &str) -> IResult<&str, Self> {
        map(
            separated_list1(
                tag(" -> "),
                map(
                    separated_pair(complete::u32, char(','), complete::u32),
                    |(x, y)| Point { x, y },
                ),
            ),
            Path,
        )(s)
    }

    fn contains(&self, point: &Point) -> bool {
        self.0.iter().zip(self.0.iter().skip(1)).any(|(p1, p2)| {
            if p1.x == p2.x {
                p1.x == point.x && point.y >= min(p1.y, p2.y) && point.y <= max(p1.y, p2.y)
            } else if p1.y == p2.y {
                p1.y == point.y && point.x >= min(p1.x, p2.x) && point.x <= max(p1.x, p2.x)
            } else {
                panic!("Illegal path")
            }
        })
    }
}

struct Grid {
    rocks: Vec<Path>,
    sand_units: HashSet<Point>,
    floor: Option<u32>,
    x_bounds: (u32, u32),
    y_bounds: (u32, u32),
}

impl Grid {
    fn from_rocks(rocks: Vec<Path>, floor: Option<u32>) -> Self {
        let x_bounds = rocks
            .iter()
            .flat_map(|p| p.0.iter())
            .map(|p| p.x)
            .minmax()
            .into_option()
            .unwrap();
        let y_bounds = rocks
            .iter()
            .flat_map(|p| p.0.iter())
            .map(|p| p.y)
            .minmax()
            .into_option()
            .unwrap();

        Grid {
            rocks,
            sand_units: HashSet::default(),
            x_bounds,
            y_bounds,
            floor,
        }
    }

    fn from_rocks_with_floor(rocks: Vec<Path>) -> Self {
        let x_bounds = rocks
            .iter()
            .flat_map(|p| p.0.iter())
            .map(|p| p.x)
            .minmax()
            .into_option()
            .unwrap();
        let y_bounds = rocks
            .iter()
            .flat_map(|p| p.0.iter())
            .map(|p| p.y)
            .minmax()
            .into_option()
            .unwrap();

        Grid {
            rocks,
            sand_units: HashSet::default(),
            x_bounds,
            y_bounds,
            floor: Some(y_bounds.1 + 2),
        }
    }

    fn add_sand_unit(&mut self) -> bool {
        let mut sand = Point { x: 500, y: 0 };

        while !self.out_of_bounds(&sand) {
            let mut next = sand.down();

            if self.is_blocked(&next) {
                next = sand.left_down();
                if self.is_blocked(&next) {
                    next = sand.right_down();
                    if self.is_blocked(&next) {
                        self.sand_units.insert(sand);
                        return true;
                    }
                }
            }

            sand = next;
        }

        return false;
    }

    fn is_blocked(&self, point: &Point) -> bool {
        self.rocks.iter().any(|p| p.contains(point))
            || self.sand_units.contains(point)
            || self.floor.map(|floor| point.y == floor).unwrap_or(false)
    }

    fn out_of_bounds(&self, point: &Point) -> bool {
        if self.floor.is_some() {
            false
        } else {
            !(point.x >= self.x_bounds.0 && point.x <= self.x_bounds.1 && point.y <= self.y_bounds.1)
        }
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let paths: Vec<_> = input
        .lines()
        .map(|line| Path::parse(line).unwrap().1)
        .collect();
    let mut grid = Grid::from_rocks(paths, None);

    let mut count = 0;
    while grid.add_sand_unit() {
        count += 1;
    }
    count
}

pub fn solve_part_2(input: &str) -> usize {
    let paths: Vec<_> = input
        .lines()
        .map(|line| Path::parse(line).unwrap().1)
        .collect();
    let mut grid = Grid::from_rocks_with_floor(paths);

    let mut count = 0;
    while !grid.is_blocked(&Point { x: 500, y: 0 }) {
        grid.add_sand_unit();
        count += 1;
    }
    count
}

#[cfg(test)]
mod test {
    use crate::prob14::{solve_part_1, solve_part_2};

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 24);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(INPUT), 93);
    }

    const INPUT: &'static str = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
}
