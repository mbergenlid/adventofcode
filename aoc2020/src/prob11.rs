use crate::prob11::Seat::{Available, Taken};
use serde::export::Formatter;
use std::fmt::Debug;

pub fn solve_part_1(input: &str) -> usize {
    solve(Seats::part_1(input))
}

pub fn solve_part_2(input: &str) -> usize {
    solve(Seats::part_2(input))
}

fn solve(seats: Seats) -> usize {
    let mut current_state = seats;
    let mut next_state = current_state.step();

    while current_state != next_state {
        current_state = next_state;
        next_state = current_state.step();
    }
    current_state
        .seats
        .iter()
        .filter(|s| s.as_ref().map(|seat| *seat == Taken).unwrap_or(false))
        .count()
}

#[derive(Hash, Eq, PartialEq, Clone)]
struct Seats {
    seats: Vec<Option<Seat>>,
    width: usize,
    height: usize,
    max_depth: u32,
    occupied_seats_needed: usize,
}

impl Seats {
    fn part_1(input: &str) -> Seats {
        let seats = input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| match c {
                'L' => Some(Available),
                '#' => Some(Taken),
                _ => None,
            })
            .collect();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Seats {
            seats,
            width,
            height,
            max_depth: 1,
            occupied_seats_needed: 4,
        }
    }

    fn part_2(input: &str) -> Seats {
        let seats = input
            .chars()
            .filter(|&c| c != '\n')
            .map(|c| match c {
                'L' => Some(Available),
                '#' => Some(Taken),
                _ => None,
            })
            .collect();
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        Seats {
            seats,
            width,
            height,
            max_depth: u32::MAX,
            occupied_seats_needed: 5,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<Seat> {
        if row >= self.height || col >= self.width {
            None
        } else if let Some(seat) = &self.seats[row * self.width + col] {
            Some(*seat)
        } else {
            None
        }
    }

    fn neighbours(&self, row: usize, col: usize) -> Neighbours {
        Neighbours {
            seats: self,
            position: (row, col),
            n: 0,
            depth: self.max_depth,
        }
    }

    fn step(&self) -> Seats {
        let mut copy = self.clone();
        for row in 0..self.height {
            for col in 0..self.width {
                let current = self.get(row, col);
                if current.is_some() {
                    let occupied_neighbours =
                        self.neighbours(row, col).filter(|&s| s == Taken).count();
                    if let Some(Available) = current {
                        if occupied_neighbours == 0 {
                            copy.seats[row * self.width + col] = Some(Taken);
                        }
                    } else if let Some(Taken) = current {
                        if occupied_neighbours >= self.occupied_seats_needed {
                            copy.seats[row * self.width + col] = Some(Available);
                        }
                    }
                }
            }
        }
        copy
    }
}

impl Debug for Seats {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                match self.get(row, col) {
                    Some(Available) => write!(f, "{}", "L")?,
                    Some(Taken) => write!(f, "{}", "#")?,
                    None => write!(f, "{}", ".")?,
                };
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Seat {
    Available,
    Taken,
}

struct Neighbours<'a> {
    seats: &'a Seats,
    position: (usize, usize),
    n: u8,
    depth: u32,
}

impl<'a> Neighbours<'a> {
    fn search_for_seat(&self, row_delta: isize, col_delta: isize) -> Option<Seat> {
        let mut row = self.position.0 as isize + row_delta;
        let mut col = self.position.1 as isize + col_delta;
        let mut depth = 0;
        while depth < self.depth
            && row >= 0
            && (row as usize) < self.seats.height
            && col >= 0
            && (col as usize) <= self.seats.width
        {
            if let Some(seat) = self.seats.get(row as usize, col as usize) {
                return Some(seat);
            }
            row = row + row_delta;
            col = col + col_delta;
            depth += 1;
        }
        None
    }
}

impl<'a> Iterator for Neighbours<'a> {
    type Item = Seat;

    fn next(&mut self) -> Option<Self::Item> {
        let mut x: Option<Seat> = None;
        while self.n <= 7 && x.is_none() {
            x = match self.n {
                0 => self.search_for_seat(-1, -1),
                1 => self.search_for_seat(-1, 0),
                2 => self.search_for_seat(-1, 1),
                3 => self.search_for_seat(0, -1),
                4 => self.search_for_seat(0, 1),
                5 => self.search_for_seat(1, -1),
                6 => self.search_for_seat(1, 0),
                7 => self.search_for_seat(1, 1),
                _ => None,
            };
            self.n += 1;
        }
        x
    }
}

#[cfg(test)]
mod test {
    use crate::prob11::Seat::Available;
    use crate::prob11::{solve_part_1, solve_part_2, Seats};

    const TESTCASE: &'static str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_neighbours() {
        let seats = Seats::part_1(TESTCASE);

        assert_eq!(
            seats.neighbours(1, 1).collect::<Vec<_>>(),
            vec![Available; 6]
        );
        assert_eq!(
            seats.neighbours(0, 0).collect::<Vec<_>>(),
            vec![Available; 2]
        );
    }

    #[test]
    fn test_part_1() {
        println!("{:?}", Seats::part_1(TESTCASE).step());
        assert_eq!(solve_part_1(TESTCASE), 37);
    }

    #[test]
    fn test_part_2() {
        // println!("{:?}", Seats::new(TESTCASE).step());
        assert_eq!(solve_part_2(TESTCASE), 26);
    }
}
