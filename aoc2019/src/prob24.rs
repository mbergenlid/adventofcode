use std::collections::{HashMap, HashSet};

pub fn solve_part_1() {
    /*
    #..#.
    ..#..
    ...##
    ...#.
    #.###
     */
    let initial_state = 0b_11101_01000_11000_00100_01001_u32;

    let mut all_states = HashSet::new();

    let mut state = initial_state;
    while !all_states.contains(&state) {
        all_states.insert(state);

        state = state.next_state();
    }
    println!("{}", state);
}

pub fn solve_part_2() {
    /*
       #..#.
       ..#..
       ...##
       ...#.
       #.###
    */
    let mut grid: HashSet<Position> = vec![
        Position::new(0, 0, 0),
        Position::new(0, 3, 0),
        Position::new(1, 2, 0),
        Position::new(2, 3, 0),
        Position::new(2, 4, 0),
        Position::new(3, 3, 0),
        Position::new(4, 0, 0),
        Position::new(4, 2, 0),
        Position::new(4, 3, 0),
        Position::new(4, 4, 0),
    ]
    .into_iter()
    .collect();

    for _ in 0..200 {
        let mut changes: HashMap<Position, u32> = HashMap::new();
        for tile in grid.iter() {
            for neighbour in tile.neighbours() {
                if let Some(count) = changes.get_mut(&neighbour) {
                    *count += 1;
                } else {
                    changes.insert(neighbour, 1);
                }
            }
        }
        grid = changes
            .into_iter()
            .filter(|(pos, count)| *count == 1 || (*count == 2 && !grid.contains(pos)))
            .map(|(pos, _)| pos)
            .collect();
    }

    println!("Part 2: {}", grid.len());
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Position {
    row: u8,
    col: u8,
    depth: i32,
}

impl Position {
    fn new(row: u8, col: u8, depth: i32) -> Position {
        Position { row, col, depth }
    }

    fn neighbours(&self) -> impl Iterator<Item = Position> {
        let mut neighbours = Vec::new();
        if self.row > 0 {
            if self.row == 3 && self.col == 2 {
                neighbours.push(Position::new(4, 0, self.depth + 1));
                neighbours.push(Position::new(4, 1, self.depth + 1));
                neighbours.push(Position::new(4, 2, self.depth + 1));
                neighbours.push(Position::new(4, 3, self.depth + 1));
                neighbours.push(Position::new(4, 4, self.depth + 1));
            } else {
                neighbours.push(Position::new(self.row - 1, self.col, self.depth));
            }
        } else {
            neighbours.push(Position::new(1, 2, self.depth - 1));
        }

        if self.col < 4 {
            if self.col == 1 && self.row == 2 {
                neighbours.push(Position::new(0, 0, self.depth + 1));
                neighbours.push(Position::new(1, 0, self.depth + 1));
                neighbours.push(Position::new(2, 0, self.depth + 1));
                neighbours.push(Position::new(3, 0, self.depth + 1));
                neighbours.push(Position::new(4, 0, self.depth + 1));
            } else {
                neighbours.push(Position::new(self.row, self.col + 1, self.depth));
            }
        } else {
            neighbours.push(Position::new(2, 3, self.depth - 1));
        }

        if self.row < 4 {
            if self.row == 1 && self.col == 2 {
                neighbours.push(Position::new(0, 0, self.depth + 1));
                neighbours.push(Position::new(0, 1, self.depth + 1));
                neighbours.push(Position::new(0, 2, self.depth + 1));
                neighbours.push(Position::new(0, 3, self.depth + 1));
                neighbours.push(Position::new(0, 4, self.depth + 1));
            } else {
                neighbours.push(Position::new(self.row + 1, self.col, self.depth));
            }
        } else {
            neighbours.push(Position::new(3, 2, self.depth - 1));
        }

        if self.col > 0 {
            if self.col == 3 && self.row == 2 {
                neighbours.push(Position::new(0, 4, self.depth + 1));
                neighbours.push(Position::new(1, 4, self.depth + 1));
                neighbours.push(Position::new(2, 4, self.depth + 1));
                neighbours.push(Position::new(3, 4, self.depth + 1));
                neighbours.push(Position::new(4, 4, self.depth + 1));
            } else {
                neighbours.push(Position::new(self.row, self.col - 1, self.depth));
            }
        } else {
            neighbours.push(Position::new(2, 1, self.depth - 1));
        }

        neighbours.into_iter()
    }
}

trait Bugs: Sized {
    fn next_state(&self) -> Self;
}

impl Bugs for u32 {
    fn next_state(&self) -> Self {
        let mut result = 0_u32;
        for i in 0..MASKS.len() {
            if self & (1 << i) != 0 {
                let pattern = self & MASKS[i];
                let set_bits = count_set_bits(pattern);
                if set_bits == 1 {
                    result |= 1 << i;
                }
            } else {
                let pattern = self & MASKS[i];
                let set_bits = count_set_bits(pattern);
                if set_bits == 1 || set_bits == 2 {
                    result |= 1 << i;
                }
            }
        }
        result
    }
}

fn count_set_bits(mut n: u32) -> u8 {
    let mut count = 0;
    while n > 0 {
        count += (n & 1) as u8;
        n >>= 1;
    }
    count
}

const MASKS: [u32; 25] = [
    0b_00000_00000_00000_00001_00010,
    0b_00000_00000_00000_00010_00101,
    0b_00000_00000_00000_00100_01010,
    0b_00000_00000_00000_01000_10100,
    0b_00000_00000_00000_10000_01000,
    0b_00000_00000_00001_00010_00001,
    0b_00000_00000_00010_00101_00010,
    0b_00000_00000_00100_01010_00100,
    0b_00000_00000_01000_10100_01000,
    0b_00000_00000_10000_01000_10000,
    0b_00000_00001_00010_00001_00000,
    0b_00000_00010_00101_00010_00000,
    0b_00000_00100_01010_00100_00000,
    0b_00000_01000_10100_01000_00000,
    0b_00000_10000_01000_10000_00000,
    0b_00001_00010_00001_00000_00000,
    0b_00010_00101_00010_00000_00000,
    0b_00100_01010_00100_00000_00000,
    0b_01000_10100_01000_00000_00000,
    0b_10000_01000_10000_00000_00000,
    0b_00010_00001_00000_00000_00000,
    0b_00101_00010_00000_00000_00000,
    0b_01010_00100_00000_00000_00000,
    0b_10100_01000_00000_00000_00000,
    0b_01000_10000_00000_00000_00000,
];

#[cfg(test)]
mod test {

    use super::Bugs;
    use crate::prob24::{solve_part_1, Position, solve_part_2};

    #[test]
    fn new_state() {
        let state = 0b00001_00100_11001_01001_10000_u32;
        println!("{:x}", state);
        assert_eq!(
            state.next_state(),
            0b00110_11011_10111_01111_01001_u32,
            "{:x}",
            state.next_state()
        );
    }

    #[test]
    fn part_1() {
        solve_part_1();
    }

    #[test]
    fn test_neighbours() {
        assert_eq!(
            Position::new(3, 3, 1).neighbours().collect::<Vec<_>>(),
            vec![
                Position::new(2, 3, 1),
                Position::new(3, 4, 1),
                Position::new(4, 3, 1),
                Position::new(3, 2, 1)
            ]
        );

        assert_eq!(
            Position::new(2, 3, 1).neighbours().collect::<Vec<_>>(),
            vec![
                Position::new(1, 3, 1),
                Position::new(2, 4, 1),
                Position::new(3, 3, 1),
                Position::new(0, 4, 2),
                Position::new(1, 4, 2),
                Position::new(2, 4, 2),
                Position::new(3, 4, 2),
                Position::new(4, 4, 2),
            ]
        );

        assert_eq!(
            Position::new(0, 4, 1).neighbours().collect::<Vec<_>>(),
            vec![
                Position::new(1, 2, 0),
                Position::new(2, 3, 0),
                Position::new(1, 4, 1),
                Position::new(0, 3, 1),
            ]
        );
    }

    #[test]
    fn part_2() {
        solve_part_2();
    }
}
