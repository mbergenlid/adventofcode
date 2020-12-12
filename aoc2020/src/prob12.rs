use crate::prob12::Move::{East, Forward, Left, North, Right, South, West};
use std::str::FromStr;
use std::ops::{Add, Mul};

pub fn solve_part_1(input: &str) -> i64 {
    let mut ship = Ship {
        position: Vector(0, 0),
        waypoint: Vector(1, 0),
    };
    for m in input.lines().map(|line| Move::from_str(line).unwrap()) {
        ship.go_part1(m);
    }
    ship.position.0.abs() + ship.position.1.abs()
}

pub fn solve_part_2(input: &str) -> i64 {
    let mut ship = Ship {
        position: Vector(0, 0),
        waypoint: Vector(10, 1),
    };
    for m in input.lines().map(|line| Move::from_str(line).unwrap()) {
        ship.go_part2(m);
    }
    ship.position.0.abs() + ship.position.1.abs()
}

struct Ship {
    position: Vector,
    waypoint: Vector,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Vector(i64, i64);

impl Vector {
    fn rotate_left(&self) -> Vector {
        let cos = 0;
        let sin = 1;
        Vector(self.0 * cos - self.1 * sin, self.0 * sin + self.1 * cos)
    }

    fn rotate_right(&self) -> Vector {
        let cos = 0;
        let sin = -1;
        Vector(self.0 * cos - self.1 * sin, self.0 * sin + self.1 * cos)
    }
}

impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, rhs: Vector) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<i64> for Vector {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: i64) -> Self::Output {
        Vector(rhs*self.0, rhs*self.1)
    }
}

impl Mul<Vector> for i64 {
    type Output = Vector;

    #[inline]
    fn mul(self, rhs: Vector) -> Self::Output {
        rhs * self
    }
}

enum Move {
    North(u32),
    East(u32),
    South(u32),
    West(u32),
    Left(u32),
    Right(u32),
    Forward(u32),
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[0..1] {
            "N" => Ok(North(s[1..].parse::<u32>().unwrap())),
            "E" => Ok(East(s[1..].parse::<u32>().unwrap())),
            "S" => Ok(South(s[1..].parse::<u32>().unwrap())),
            "W" => Ok(West(s[1..].parse::<u32>().unwrap())),
            "L" => Ok(Left(s[1..].parse::<u32>().unwrap())),
            "R" => Ok(Right(s[1..].parse::<u32>().unwrap())),
            "F" => Ok(Forward(s[1..].parse::<u32>().unwrap())),
            _ => Err("Invalid".to_string()),
        }
    }
}

impl Ship {
    fn go_part1(&mut self, m: Move) {
        match m {
            Move::North(d) => self.position.1 += d as i64,
            Move::East(d) => self.position.0 += d as i64,
            Move::South(d) => self.position.1 -= d as i64,
            Move::West(d) => self.position.0 -= d as i64,
            Move::Left(d) => {
                for _ in 0..(d / 90) {
                    self.waypoint = self.waypoint.rotate_left()
                }
            }
            Move::Right(d) => {
                for _ in 0..(d / 90) {
                    self.waypoint = self.waypoint.rotate_right()
                }
            }
            Move::Forward(d) => {
                self.position = Vector(
                    self.position.0 + (d as i64) * self.waypoint.0,
                    self.position.1 + (d as i64) * self.waypoint.1,
                )
            }
        }
    }

    fn go_part2(&mut self, m: Move) {
        match m {
            Move::North(d) => self.waypoint.1 += d as i64,
            Move::East(d) => self.waypoint.0 += d as i64,
            Move::South(d) => self.waypoint.1 -= d as i64,
            Move::West(d) => self.waypoint.0 -= d as i64,
            Move::Left(d) => {
                for _ in 0..(d / 90) {
                    self.waypoint = self.waypoint.rotate_left()
                }
            }
            Move::Right(d) => {
                for _ in 0..(d / 90) {
                    self.waypoint = self.waypoint.rotate_right()
                }
            }
            Forward(d) => {
                self.position = self.position + (d as i64)*self.waypoint;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob12::{solve_part_1, solve_part_2, Vector};

    #[test]
    fn test_rotate() {
        assert_eq!(Vector(1, 0).rotate_right(), Vector(0, -1));
        assert_eq!(Vector(1, 0).rotate_left(), Vector(0, 1));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            solve_part_1(
                "F10
N3
F7
R90
F11"
            ),
            25
        );
        assert_eq!(solve_part_1(include_str!("../inputs/prob12")), 381);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            solve_part_2(
                "F10
N3
F7
R90
F11"
            ),
            286
        );
        assert_eq!(solve_part_2(include_str!("../inputs/prob12")), 28591);
    }
}
