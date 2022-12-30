use std::collections::{HashMap, HashSet};
use std::fmt::{Debug, Formatter};
use itertools::Itertools;

#[derive(Hash, Eq, PartialEq, Copy, Clone)]
struct Elf {
    x: i64,
    y: i64,
}

impl Elf {
    fn add(&self, direction: (i64, i64)) -> Elf {
        Elf {
            x: self.x + direction.0,
            y: self.y + direction.1
        }
    }
}

const N: (i64, i64) = (0, -1);
const NE: (i64, i64) = (1, -1);
const NW: (i64, i64) = (-1, -1);
const E: (i64, i64) = (1, 0);
const W: (i64, i64) = (-1, 0);
const S: (i64, i64) = (0, 1);
const SE: (i64, i64) = (1, 1);
const SW: (i64, i64) = (-1, 1);

const ALL_NEIGHBOURS: [(i64, i64); 8] = [N, NE, NW, E, W, S, SE, SW];

struct Map<'a> {
    elves: &'a HashSet<Elf>,
}

impl <'a> Debug for Map<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (min_x, max_x) = self.elves.iter().map(|e| e.x).minmax().into_option().unwrap();
        let (min_y, max_y) = self.elves.iter().map(|e| e.y).minmax().into_option().unwrap();

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if self.elves.contains(&Elf {x, y}) {
                    write!(f, "{}", '#').unwrap();
                } else {
                    write!(f, "{}", '.').unwrap();
                }
            }
            writeln!(f).unwrap();
        }
        Ok(())
    }
}

impl <'a> Map<'a> {

    fn has_neighbous(&self, elf: &Elf) -> bool {
        for neighbour in ALL_NEIGHBOURS {
            if self.elves.contains(&elf.add(neighbour)) {
                return true;
            }
        }
        return false;
    }

    fn has_neighbous_in(&self, elf: &Elf, directions: &[(i64, i64)]) -> bool {
        for neighbour in directions {
            if self.elves.contains(&elf.add(*neighbour)) {
                return true;
            }
        }
        return false;
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| Elf {
                    x: col as i64,
                    y: row as i64,
                })
        })
        .collect::<HashSet<_>>();
    let mut moves = vec![(N, [N, NE, NW]), (S, [S, SE, SW]), (W, [W,NW,SW]), (E, [E, NE, SE])];

    for _ in 0..10 {
        let mut proposed_new_positions: HashMap<Elf, Vec<Elf>> = HashMap::new();
        let map = Map { elves: &elves };

        for elf in elves.iter() {
            if map.has_neighbous(elf) {
                let mut found = false;
                for m in moves.iter() {
                    if !map.has_neighbous_in(elf, &m.1) {
                        let new_position = elf.add(m.0);
                        found = true;
                        if let Some(sources) = proposed_new_positions.get_mut(&new_position) {
                            sources.push(*elf);
                        } else {
                            proposed_new_positions.insert(new_position, vec![*elf]);
                        }
                        break;
                    }
                }
                if !found {
                    proposed_new_positions.insert(*elf, vec![*elf]);
                }
            } else {
                proposed_new_positions.insert(*elf, vec![*elf]);
            }
        }

        let mut new_positions = HashSet::new();
        for (pos, sources) in proposed_new_positions {
            if sources.len() == 1 {
                new_positions.insert(pos);
            } else {
                for elf in sources {
                    new_positions.insert(elf);
                }
            }
        }
        assert_eq!(new_positions.len(), elves.len());
        elves = new_positions;

        let first = moves.remove(0);
        moves.push(first);
    }

    let (min_x, max_x) = elves.iter().map(|e| e.x).minmax().into_option().unwrap();
    let (min_y, max_y) = elves.iter().map(|e| e.y).minmax().into_option().unwrap();

    (((max_x - min_x + 1)*(max_y-min_y+1)) as usize) - elves.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut elves = input
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(col, _)| Elf {
                    x: col as i64,
                    y: row as i64,
                })
        })
        .collect::<HashSet<_>>();
    let mut moves = vec![(N, [N, NE, NW]), (S, [S, SE, SW]), (W, [W,NW,SW]), (E, [E, NE, SE])];

    for round in 1.. {
        let mut proposed_new_positions: HashMap<Elf, Vec<Elf>> = HashMap::new();
        let map = Map { elves: &elves };

        for elf in elves.iter() {
            if map.has_neighbous(elf) {
                let mut found = false;
                for m in moves.iter() {
                    if !map.has_neighbous_in(elf, &m.1) {
                        let new_position = elf.add(m.0);
                        found = true;
                        if let Some(sources) = proposed_new_positions.get_mut(&new_position) {
                            sources.push(*elf);
                        } else {
                            proposed_new_positions.insert(new_position, vec![*elf]);
                        }
                        break;
                    }
                }
                if !found {
                    proposed_new_positions.insert(*elf, vec![*elf]);
                }
            } else {
                proposed_new_positions.insert(*elf, vec![*elf]);
            }
        }

        let mut num_elves_moved = 0;
        let mut new_positions = HashSet::new();
        for (pos, sources) in proposed_new_positions {
            if sources.len() == 1 {
                new_positions.insert(pos);
                if pos != sources[0] {
                    num_elves_moved += 1;
                }
            } else {
                for elf in sources {
                    new_positions.insert(elf);
                }
            }
        }
        if num_elves_moved == 0 {
            return round;
        }
        assert_eq!(new_positions.len(), elves.len());
        elves = new_positions;

        let first = moves.remove(0);
        moves.push(first);
    }

    println!("{:?}", Map { elves: &elves});
    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::prob23::{solve_part_1, solve_part_2};

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 110);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(INPUT), 20);
    }

    const INPUT: &'static str = r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";
}