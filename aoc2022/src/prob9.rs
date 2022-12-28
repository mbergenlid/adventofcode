use std::collections::HashSet;

use crate::prob9::Direction::{Down, Left, Right, Up};

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Pos((i32, i32));

impl Pos {
    fn x(&self) -> i32 {
        self.0 .0
    }

    fn y(&self) -> i32 {
        self.0 .1
    }

    fn step(&self, dir: Direction) -> Pos {
        match dir {
            Up => Pos((self.x(), self.y() + 1)),
            Down => Pos((self.x(), self.y() - 1)),
            Left => Pos((self.x() - 1, self.y())),
            Right => Pos((self.x() + 1, self.y())),
        }
    }

    fn direction_to(&self, to: Pos) -> (Option<Direction>, Option<Direction>) {
        let x_diff = (self.x() - to.x()).abs();
        let y_diff = (self.y() - to.y()).abs();
        if x_diff > 1 || y_diff > 1 {
            let x_dir = if self.x() > to.x() {
                Some(Left)
            } else if self.x() < to.x() {
                Some(Right)
            } else {
                None
            };

            let y_dir = if self.y() > to.y() {
                Some(Down)
            } else if self.y() < to.y() {
                Some(Up)
            } else {
                None
            };

            (x_dir, y_dir)
        } else {
            (None, None)
        }
    }
}

struct Rope {
    knots: Vec<Pos>,
}

impl Rope {
    fn step(&mut self, dir: Direction) {
        self.knots[0] = self.knots[0].step(dir);

        let mut prev = self.knots[0];
        for knot in self.knots.iter_mut() {
            *knot = Rope::follow(&prev, knot);
            prev = *knot
        }
    }

    fn tail(&self) -> &Pos {
        self.knots.last().unwrap()
    }

    fn follow(head: &Pos, tail: &Pos) -> Pos {
        let (x_dir, y_dir) = tail.direction_to(*head);

        let mut res = *tail;
        if let Some(x_dir) = x_dir {
            res = res.step(x_dir);
        }

        if let Some(y_dir) = y_dir {
            res = res.step(y_dir);
        }
        res
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn solve(mut rope: Rope, input: &str) -> usize {
    let mut visited = HashSet::new();
    for line in input.lines() {
        let mut split = line.split(" ");

        let dir = match split.next().unwrap() {
            "U" => Up,
            "D" => Down,
            "L" => Left,
            "R" => Right,
            _ => panic!(),
        };
        let steps = split.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..steps {
            rope.step(dir);
            // println!("{:?}", rope);
            visited.insert(rope.tail().clone());
        }
    }

    visited.len()
}

pub fn solve_part_1(input: &str) -> usize {
    solve(
        Rope {
            knots: vec![Pos((0, 0)); 2],
        },
        input,
    )
}

pub fn solve_part_2(input: &str) -> usize {
    solve(
        Rope {
            knots: vec![Pos((0, 0)); 10],
        },
        input,
    )
}

#[cfg(test)]
mod test {
    use crate::prob9::solve_part_1;
    use crate::prob9::solve_part_2;

    #[test]
    fn test_part_1() {
        let input = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(solve_part_1(&input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(solve_part_2(&input), 1);
    }
}
