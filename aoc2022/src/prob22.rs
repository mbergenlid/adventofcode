use crate::prob22::Direction::{Down, Left, Right, Up};
use crate::prob22::Instruction::{TurnLeft, TurnRight, Walk};
use crate::prob22::Tile::{Empty, Open, Wall};
use itertools::Itertools;
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::char;
use nom::combinator::map;

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Eq, PartialEq, Hash)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down,
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }

    fn score(&self) -> usize {
        match self {
            Up => 3,
            Right => 0,
            Down => 1,
            Left => 2,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Walk(u32),
    TurnLeft,
    TurnRight,
}

#[derive(Eq, PartialEq)]
enum Tile {
    Open,
    Wall,
    Empty,
}

struct Map {
    data: Vec<Vec<Tile>>,
    wrap_around: Option<Vec<(Bounds, Box<dyn WrapAround>)>>,
}

impl Map {
    fn parse(s: &str, wrap_around: Option<Vec<(Bounds, Box<dyn WrapAround>)>>) -> Self {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        if c == ' ' {
                            Empty
                        } else if c == '#' {
                            Wall
                        } else {
                            Open
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        Map { data, wrap_around }
    }

    fn top_left(&self) -> Point {
        let (x, _) = self.data[0].iter().find_position(|&p| p == &Open).unwrap();
        Point { x, y: 0 }
    }

    fn get(&self, x: usize, y: usize) -> &Tile {
        self.data
            .get(y)
            .and_then(|row| row.get(x))
            .unwrap_or(&Empty)
    }

    fn step(&self, point: &Point, direction: &Direction) -> Option<(Point, Direction)> {
        match direction {
            Up => {
                match self.get(point.x, point.y.wrapping_sub(1)) {
                    Open => Some((
                        Point {
                            y: point.y - 1,
                            ..*point
                        },
                        direction.clone(),
                    )),
                    Wall => None,
                    Empty => {
                        //wrap around
                        let (next_point, next_dir) =
                            self.wrap_around(&point, &direction).unwrap_or_else(|| {
                                let (index, _tile) = self
                                    .data
                                    .iter()
                                    .enumerate()
                                    .rev()
                                    .map(|(i, row)| (i, row.get(point.x).unwrap_or(&Empty)))
                                    .find(|(_i, tile)| **tile != Empty)
                                    .unwrap();

                                (Point { y: index, ..*point }, direction.clone())
                            });
                        let tile = self.get(next_point.x, next_point.y);
                        match tile {
                            Open => Some((next_point, next_dir)),
                            Wall => None,
                            Empty => unreachable!(),
                        }
                    }
                }
            }
            Right => match self.get(point.x + 1, point.y) {
                Open => Some((
                    Point {
                        x: point.x + 1,
                        ..*point
                    },
                    direction.clone(),
                )),
                Wall => None,
                Empty => {
                    //wrap around
                    let (next_point, next_dir) =
                        self.wrap_around(&point, &direction).unwrap_or_else(|| {
                            let (index, _tile) = self
                                .data
                                .get(point.y)
                                .unwrap()
                                .iter()
                                .find_position(|&tile| *tile != Empty)
                                .unwrap();

                            (Point { x: index, ..*point }, direction.clone())
                        });
                    let tile = self.get(next_point.x, next_point.y);
                    match tile {
                        Open => Some((next_point, next_dir)),
                        Wall => None,
                        Empty => unreachable!(),
                    }
                }
            },
            Down => match self.get(point.x, point.y + 1) {
                Open => Some((
                    Point {
                        y: point.y + 1,
                        ..*point
                    },
                    direction.clone(),
                )),
                Wall => None,
                Empty => {
                    //wrap around
                    let (next_point, next_dir) =
                        self.wrap_around(&point, &direction).unwrap_or_else(|| {
                            let (index, _tile) = self
                                .data
                                .iter()
                                .map(|row| row.get(point.x).unwrap_or(&Empty))
                                .find_position(|&tile| *tile != Empty)
                                .unwrap();

                            (Point { y: index, ..*point }, direction.clone())
                        });
                    let tile = self.get(next_point.x, next_point.y);
                    match tile {
                        Open => Some((next_point, next_dir)),
                        Wall => None,
                        Empty => unreachable!(),
                    }
                }
            },
            Left => match self.get(point.x.wrapping_sub(1), point.y) {
                Open => Some((
                    Point {
                        x: point.x - 1,
                        ..*point
                    },
                    direction.clone(),
                )),
                Wall => None,
                Empty => {
                    //wrap around
                    let (next_point, next_dir) =
                        self.wrap_around(&point, &direction).unwrap_or_else(|| {
                            let (index, _tile) = self
                                .data
                                .get(point.y)
                                .unwrap()
                                .iter()
                                .enumerate()
                                .rev()
                                .find(|(_i, tile)| **tile != Empty)
                                .unwrap();

                            (Point { x: index, ..*point }, direction.clone())
                        });
                    let tile = self.get(next_point.x, next_point.y);
                    match tile {
                        Open => Some((next_point, next_dir)),
                        Wall => None,
                        Empty => unreachable!(),
                    }
                }
            },
        }
    }

    fn wrap_around(&self, point: &Point, direction: &Direction) -> Option<(Point, Direction)> {
        self.wrap_around.as_ref().map(|wrap_around_rules| {
            let (_, wrap_rule) = wrap_around_rules
                .iter()
                .find(|(bounds, _)| bounds.contains(point))
                .expect("No wrap rule");
            let next = wrap_rule.next(point, direction);
            next
        })
    }
}

pub fn solve_part_1(input: &str) -> usize {
    solve(input, None)
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, Some(wrap_around()))
}

fn solve(input: &str, wrap_around: Option<Vec<(Bounds, Box<dyn WrapAround>)>>) -> usize {
    let mut split = input.split("\n\n");
    let map_data = Map::parse(split.next().unwrap(), wrap_around);

    let mut direction = Right;
    let mut point = map_data.top_left();
    let mut instructions = split.next().unwrap();

    while let Ok((rest, instr)) = alt::<_, _, (), _>((
        map(complete::u32, Walk),
        map(char('L'), |_| TurnLeft),
        map(char('R'), |_| TurnRight),
    ))(instructions)
    {
        match instr {
            Walk(n) => {
                for _ in 0..n {
                    if let Some((new_point, new_dir)) = map_data.step(&point, &direction) {
                        point = new_point;
                        direction = new_dir;
                    } else {
                        break;
                    }
                }
            }
            TurnLeft => direction = direction.turn_left(),
            TurnRight => direction = direction.turn_right(),
        }
        instructions = rest
    }
    1000 * (point.y + 1) + 4 * (point.x + 1) + direction.score()
}

struct Bounds((Point, Point));

impl Bounds {
    fn contains(&self, point: &Point) -> bool {
        (self.0 .0.x..self.0 .1.x).contains(&point.x)
            && (self.0 .0.y..self.0 .1.y).contains(&point.y)
    }
}

trait WrapAround {
    fn next(&self, point: &Point, direction: &Direction) -> (Point, Direction);
}

fn wrap_around() -> Vec<(Bounds, Box<dyn WrapAround>)> {
    vec![
        (
            Bounds((Point { x: 50, y: 0 }, Point { x: 100, y: 50 })),
            Box::new(Square1),
        ),
        (
            Bounds((Point { x: 100, y: 0 }, Point { x: 150, y: 50 })),
            Box::new(Square2),
        ),
        (
            Bounds((Point { x: 50, y: 50 }, Point { x: 100, y: 100 })),
            Box::new(Square3),
        ),
        (
            Bounds((Point { x: 0, y: 100 }, Point { x: 50, y: 150 })),
            Box::new(Square4),
        ),
        (
            Bounds((Point { x: 50, y: 100 }, Point { x: 50+50, y: 100+50 })),
            Box::new(Square5),
        ),
        (
            Bounds((Point { x: 0, y: 150 }, Point { x: 0+50, y: 150+50 })),
            Box::new(Square6),
        ),
    ]
}

struct Square1;
impl WrapAround for Square1 {
    fn next(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
        match direction {
            Up => (
                Point {
                    x: 0,
                    y: 150 + (point.x-50),
                },
                Right,
            ),
            Right => unreachable!(),
            Down => unreachable!(),
            Left => (
                Point {
                    x: 0,
                    y: 149 - point.y,
                },
                Right,
            ),
        }
    }
}
struct Square2;
impl WrapAround for Square2 {
    fn next(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
        match direction {
            Up => (
                Point {
                    x: point.x - 100,
                    y: 199,
                },
                Up,
            ),
            Right => (
                Point {
                    x: 99,
                    y: 149 - point.y,
                },
                Left,
            ),
            Down => (
                Point {
                    x: 99,
                    y: (point.x - 100) + 50,
                },
                Left,
            ),
            Left => unreachable!(),
        }
    }
}
struct Square3;
impl WrapAround for Square3 {
    fn next(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
        match direction {
            Up => unreachable!(),
            Right => (
                Point {
                    x: (point.y - 50) + 100,
                    y: 49,
                },
                Up,
            ),
            Down => unreachable!(),
            Left => (
                Point {
                    x: (point.y - 50),
                    y: 100,
                },
                Down,
            ),
        }
    }
}
struct Square4;
impl WrapAround for Square4 {
    fn next(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
        match direction {
            Up => (
                Point {
                    x: 50,
                    y: point.x + 50,
                },
                Right,
            ),
            Right => unreachable!(),
            Down => unreachable!(),
            Left => (
                Point {
                    x: 50,
                    y: 49 - (point.y - 100),
                },
                Right,
            ),
        }
    }
}
struct Square5;
impl WrapAround for Square5 {
    fn next(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
        match direction {
            Up => unreachable!(),
            Right => (
                Point {
                    x: 149,
                    y: 49 - (point.y - 100),
                },
                Left,
            ),
            Down => (
                Point {
                    x: 49,
                    y: (point.x - 50) + 150,
                },
                Left,
            ),
            Left => unreachable!(),
        }
    }
}
struct Square6;
impl WrapAround for Square6 {
    fn next(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
        match direction {
            Up => unreachable!(),
            Right => (
                Point {
                    x: (point.y - 150) + 50,
                    y: 149,
                },
                Up,
            ),
            Down => (
                Point {
                    x: point.x + 100,
                    y: 0,
                },
                Down,
            ),
            Left => (
                Point {
                    x: (point.y - 150) + 50,
                    y: 0,
                },
                Down,
            ),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob22::Direction::{Down, Left, Right, Up};
    use crate::prob22::{solve, solve_part_1, Bounds, Direction, Point, WrapAround, solve_part_2};

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(include_str!("../inputs/prob22")), 123046);
    }

    #[test]
    fn test_1_2() {
        assert_eq!(solve_part_1(INPUT_2), 1000 * 1 + 4 * 9 + 2);
    }
    #[test]
    fn test_1_3() {
        assert_eq!(solve_part_1(INPUT_3), 1000 * 4 + 4 * 9 + 3);
    }

    #[test]
    fn test_2_2() {
        assert_eq!(
            solve(INPUT_2, Some(wrap_around_test())),
            1000 * 8 + 4 * 5 + 1
        );
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(include_str!("../inputs/prob22")), 123046);
    }

    fn wrap_around_test() -> Vec<(Bounds, Box<dyn WrapAround>)> {
        vec![(
            Bounds((Point { x: 8, y: 0 }, Point { x: 8 + 4, y: 0 + 3 })),
            Box::new(Square1),
        )]
    }

    struct Square1;

    impl WrapAround for Square1 {
        fn next(&self, point: &Point, direction: &Direction) -> (Point, Direction) {
            match direction {
                Up => todo!(),
                Right => todo!(),
                Down => todo!(),
                Left => (
                    Point {
                        x: point.y + 4,
                        y: 4,
                    },
                    Down,
                ),
            }
        }
    }

    const INPUT: &'static str = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    const INPUT_2: &'static str = r"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

LL4";

    const INPUT_3: &'static str = r"        ...#
        .#..
        #...
        ....
...#....
........
..#....#
........

L5";
}
