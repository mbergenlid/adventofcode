use crate::prob2::Command::{Foreward, Down, Up};

enum Command {
    Foreward(usize),
    Down(usize),
    Up(usize)
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        if s.starts_with("forward ") {
            Foreward(s["forward ".len()..].parse().unwrap())
        } else if s.starts_with("down ") {
            Down(s["down ".len()..].parse().unwrap())
        } else if s.starts_with("up ") {
            Up(s["up ".len()..].parse().unwrap())
        } else {
            panic!();
        }

    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth = 0;
    for cmd in input.lines().map(|l| Command::from(l)) {
        match cmd {
            Foreward(x) => {
                horizontal += x;
            }
            Down(x) => {
                depth += x;
            }
            Up(x) => {
                depth -= x;
            }
        }
    }
    horizontal * depth
}

pub fn solve_part_2(input: &str) -> usize {
    let mut horizontal = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    for cmd in input.lines().map(|l| Command::from(l)) {
        match cmd {
            Foreward(x) => {
                horizontal += x;
                depth += aim * (x as i32);
            }
            Down(x) => {
                aim += x as i32;
            }
            Up(x) => {
                aim -= x as i32;
            }
        }
    }
    horizontal * (depth as usize)
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {
        let res = super::solve_part_1(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2"
        );
        assert_eq!(res, 150);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2(
            "forward 5
down 5
forward 8
up 3
down 8
forward 2"
        );
        assert_eq!(res, 900);
    }

}
