use nom::FindSubstring;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::ops::Add;

const LEFT: (i64, i64) = (-1, 0);
const UP: (i64, i64) = (0, -1);
const RIGHT: (i64, i64) = (1, 0);
const DOWN: (i64, i64) = (0, 1);

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new(x: usize, y: usize) -> Self {
        Pos {
            x: x as i64,
            y: y as i64,
        }
    }
}

impl Add<(i64, i64)> for Pos {
    type Output = Self;

    fn add(self, rhs: (i64, i64)) -> Self::Output {
        Pos {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct Blizzard {
    direction: (i64, i64),
}

impl Blizzard {
    fn left() -> Self {
        Blizzard { direction: (-1, 0) }
    }

    fn right() -> Self {
        Blizzard { direction: (1, 0) }
    }
    fn down() -> Self {
        Blizzard { direction: (0, 1) }
    }
    fn up() -> Self {
        Blizzard { direction: (0, -1) }
    }
}

struct BlizzardState {
    minute: u32,
    states: VecDeque<HashMap<Pos, Vec<Blizzard>>>,
    // next_minute: HashMap<Pos, Blizzard>,
    // current_minute: Option<HashMap<Pos, Blizzard>>,
    map: Map,
}

impl BlizzardState {
    fn new(map: Map, state: HashMap<Pos, Blizzard>) -> Self {
        let current_minute = state.into_iter().map(|(p, b)| (p, vec![b])).collect();
        let next_minute = BlizzardState::generate_new_state(&current_minute, &map);
        BlizzardState {
            minute: 0,
            states: vec![current_minute, next_minute].into(),
            // current_minute: state,
            // next_minute,
            map,
        }
    }

    fn generate_new_state(
        state: &HashMap<Pos, Vec<Blizzard>>,
        map: &Map,
    ) -> HashMap<Pos, Vec<Blizzard>> {
        let mut new_state: HashMap<Pos, Vec<Blizzard>> = HashMap::new();
        for (p, blizzard) in state.iter().flat_map(|(p, blizzards)| {
            blizzards.iter().map(move |b| {
                let new_pos = *p + b.direction;
                if map.in_bounds(&new_pos) {
                    (new_pos, b.clone())
                } else {
                    //wrap around
                    match b {
                        Blizzard { direction: DOWN } => (Pos { x: new_pos.x, y: 1 }, b.clone()),
                        Blizzard { direction: UP } => (
                            Pos {
                                x: new_pos.x,
                                y: (map.height as i64) - 2,
                            },
                            b.clone(),
                        ),
                        Blizzard { direction: LEFT } => (
                            Pos {
                                x: (map.width as i64) - 2,
                                y: new_pos.y,
                            },
                            b.clone(),
                        ),
                        Blizzard { direction: RIGHT } => (Pos { x: 1, y: new_pos.y }, b.clone()),
                        _ => unreachable!(),
                    }
                }
            })
        }) {
            if let Some(blizzards) = new_state.get_mut(&p) {
                blizzards.push(blizzard);
            } else {
                new_state.insert(p, vec![blizzard]);
            }
        }

        new_state
    }

    fn get_state(&mut self, minute: u32) -> &HashMap<Pos, Vec<Blizzard>> {
        if minute == self.minute {
            &self.states[0]
        } else if minute == self.minute + 1 {
            &self.states[1]
        } else if minute == self.minute + 2 {
            //generate new state
            self.states.pop_front();
            self.states.push_back(BlizzardState::generate_new_state(
                &self.states[0],
                &self.map,
            ));
            self.minute = self.minute + 1;
            &self.states[1]
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone)]
struct Map {
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut lines = input.lines();
        let first_line = lines.next().unwrap();
        let start = first_line.find_substring(".").expect("No start position");
        let width = first_line.len();
        let height = input.lines().count();

        let end = lines
            .last()
            .unwrap()
            .find_substring(".")
            .expect("No end position");

        Map {
            width,
            height,
            start: Pos {
                x: start as i64,
                y: 0,
            },
            end: Pos {
                x: end as i64,
                y: (height - 1) as i64,
            },
        }
    }

    fn in_bounds(&self, pos: &Pos) -> bool {
        pos.y > 0
            && pos.y < (self.height as i64) - 1
            && pos.x > 0
            && pos.x < (self.width as i64) - 1
            || *pos == self.start
            || *pos == self.end
    }
}

pub fn solve_part_1(input: &str) -> usize {
    let map = Map::parse(input);

    let lines = input.lines();
    let blizzards = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '>' => Some((Pos::new(x, y), Blizzard::right())),
                '^' => Some((Pos::new(x, y), Blizzard::up())),
                '<' => Some((Pos::new(x, y), Blizzard::left())),
                'v' => Some((Pos::new(x, y), Blizzard::down())),
                _ => None,
            })
        })
        .collect::<HashMap<_, _>>();
    let mut blizzard_state = BlizzardState::new(map.clone(), blizzards);

    bfs(&map, &mut blizzard_state, 0, map.start.clone(), map.end.clone())
}

pub fn solve_part_2(input: &str) -> usize {
    let map = Map::parse(input);

    let lines = input.lines();
    let blizzards = lines
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().filter_map(move |(x, c)| match c {
                '>' => Some((Pos::new(x, y), Blizzard { direction: RIGHT })),
                '^' => Some((Pos::new(x, y), Blizzard::up())),
                '<' => Some((Pos::new(x, y), Blizzard::left())),
                'v' => Some((Pos::new(x, y), Blizzard::down())),
                _ => None,
            })
        })
        .collect::<HashMap<_, _>>();
    let mut blizzard_state = BlizzardState::new(map.clone(), blizzards);

    let first = bfs(&map, &mut blizzard_state, 0, map.start.clone(), map.end.clone());
    let second = bfs(&map, &mut blizzard_state, first as u32, map.end.clone(), map.start.clone());
    let third = bfs(&map, &mut blizzard_state, second as u32, map.start.clone(), map.end.clone());

    third
}

fn bfs(map: &Map, blizzard_state: &mut BlizzardState, initial_minute: u32, start: Pos, end: Pos) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start, initial_minute));


    while let Some((pos, minute)) = queue.pop_front() {
        if pos == end {
            return minute as usize;
        }

        let next_state = blizzard_state.get_state(minute + 1);

        for n in [(0, 0), LEFT, RIGHT, UP, DOWN].map(|d| pos + d) {
            if map.in_bounds(&n) && !next_state.contains_key(&n) {
                if !queue.contains(&(n, minute + 1)) {
                    queue.push_back((n, minute + 1));
                }
            }
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use crate::prob24::{solve_part_1, solve_part_2};

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 18);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(INPUT), 54);
    }

    const INPUT: &'static str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";
}
