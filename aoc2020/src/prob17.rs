use std::collections::{HashMap, HashSet};

pub fn solve_part_1(input: &str) -> usize {
    solve(input, &|p| p.neighbours())
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, &|p| p.neighbours_hypercube())
}

fn solve<I: Iterator<Item=Position>>(input: &str, f: &dyn Fn(Position) -> I) -> usize {
    let mut state = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Position(x as i64, y as i64, 0, 0))
        })
        .collect::<HashSet<_>>();

    for _ in 0..6 {
        let mut changes: HashMap<Position, usize> = HashMap::new();
        for p in state.iter() {
            for n in f(*p) {
                if let Some(p1) = changes.get_mut(&n) {
                    *p1 += 1;
                } else {
                    changes.insert(n, 1);
                }
            }
        }
        state = changes
            .into_iter()
            .filter(|(pos, count)| *count == 3 || (*count == 2 && state.contains(pos)))
            .map(|(pos, _)| pos)
            .collect::<HashSet<_>>();
    }

    state.len()
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
struct Position(i64, i64, i64, i64);

impl Position {
    fn neighbours(self) -> impl Iterator<Item = Position> {
        NEIGHBOURS
            .iter()
            .map(move |&p| Position(self.0 + p.0, self.1 + p.1, self.2 + p.2, 0))
    }


    fn neighbours_hypercube(self) -> impl Iterator<Item = Position> {
        NEIGHBOURS_2
            .iter()
            .map(move |&p| Position(self.0 + p.0, self.1 + p.1, self.2 + p.2, self.3 + p.3))
    }
}

#[cfg(test)]
mod test {
    use crate::prob17::{solve_part_1, solve_part_2};

    const TESTCASE_1: &'static str = ".#.
..#
###";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TESTCASE_1), 112);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TESTCASE_1), 848);
    }
}

const NEIGHBOURS: [Position; 26] = [
    Position(-1, -1, -1, 0),
    Position(-1, -1, 0, 0),
    Position(-1, -1, 1, 0),
    Position(-1, 0, -1, 0),
    Position(-1, 0, 0, 0),
    Position(-1, 0, 1, 0),
    Position(-1, 1, -1, 0),
    Position(-1, 1, 0, 0),
    Position(-1, 1, 1, 0),
    Position(0, -1, -1, 0),
    Position(0, -1, 0, 0),
    Position(0, -1, 1, 0),
    Position(0, 0, -1, 0),
    Position(0, 0, 1, 0),
    Position(0, 1, -1, 0),
    Position(0, 1, 0, 0),
    Position(0, 1, 1, 0),
    Position(1, -1, -1, 0),
    Position(1, -1, 0, 0),
    Position(1, -1, 1, 0),
    Position(1, 0, -1, 0),
    Position(1, 0, 0, 0),
    Position(1, 0, 1, 0),
    Position(1, 1, -1, 0),
    Position(1, 1, 0, 0),
    Position(1, 1, 1, 0),
];

const NEIGHBOURS_2: [Position; 80] = [
    Position(-1, -1, -1, -1),
    Position(-1, -1, -1, 0),
    Position(-1, -1, -1, 1),
    Position(-1, -1, 0, -1),
    Position(-1, -1, 0, 0),
    Position(-1, -1, 0, 1),
    Position(-1, -1, 1, -1),
    Position(-1, -1, 1, 0),
    Position(-1, -1, 1, 1),
    Position(-1, 0, -1, -1),
    Position(-1, 0, -1, 0),
    Position(-1, 0, -1, 1),
    Position(-1, 0, 0, -1),
    Position(-1, 0, 0, 0),
    Position(-1, 0, 0, 1),
    Position(-1, 0, 1, -1),
    Position(-1, 0, 1, 0),
    Position(-1, 0, 1, 1),
    Position(-1, 1, -1, -1),
    Position(-1, 1, -1, 0),
    Position(-1, 1, -1, 1),
    Position(-1, 1, 0, -1),
    Position(-1, 1, 0, 0),
    Position(-1, 1, 0, 1),
    Position(-1, 1, 1, -1),
    Position(-1, 1, 1, 0),
    Position(-1, 1, 1, 1),

    Position(0, -1, -1, -1),
    Position(0, -1, -1, 0),
    Position(0, -1, -1, 1),
    Position(0, -1, 0, -1),
    Position(0, -1, 0, 0),
    Position(0, -1, 0, 1),
    Position(0, -1, 1, -1),
    Position(0, -1, 1, 0),
    Position(0, -1, 1, 1),
    Position(0, 0, -1, -1),
    Position(0, 0, -1, 0),
    Position(0, 0, -1, 1),
    Position(0, 0, 0, -1),
    Position(0, 0, 0, 1),
    Position(0, 0, 1, -1),
    Position(0, 0, 1, 0),
    Position(0, 0, 1, 1),
    Position(0, 1, -1, -1),
    Position(0, 1, -1, 0),
    Position(0, 1, -1, 1),
    Position(0, 1, 0, -1),
    Position(0, 1, 0, 0),
    Position(0, 1, 0, 1),
    Position(0, 1, 1, -1),
    Position(0, 1, 1, 0),
    Position(0, 1, 1, 1),

    Position(1, -1, -1, -1),
    Position(1, -1, -1, 0),
    Position(1, -1, -1, 1),
    Position(1, -1, 0, -1),
    Position(1, -1, 0, 0),
    Position(1, -1, 0, 1),
    Position(1, -1, 1, -1),
    Position(1, -1, 1, 0),
    Position(1, -1, 1, 1),
    Position(1, 0, -1, -1),
    Position(1, 0, -1, 0),
    Position(1, 0, -1, 1),
    Position(1, 0, 0, -1),
    Position(1, 0, 0, 0),
    Position(1, 0, 0, 1),
    Position(1, 0, 1, -1),
    Position(1, 0, 1, 0),
    Position(1, 0, 1, 1),
    Position(1, 1, -1, -1),
    Position(1, 1, -1, 0),
    Position(1, 1, -1, 1),
    Position(1, 1, 0, -1),
    Position(1, 1, 0, 0),
    Position(1, 1, 0, 1),
    Position(1, 1, 1, -1),
    Position(1, 1, 1, 0),
    Position(1, 1, 1, 1),
];
