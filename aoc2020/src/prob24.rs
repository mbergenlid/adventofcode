use std::collections::{HashMap, HashSet};

pub fn solve_part_1(input: &str) -> usize {
    initial_state(input).len()
}

fn initial_state(input: &str) -> HashSet<(i32, i32)> {
    let mut black_tiles = HashSet::new();
    for line in input.lines() {
        let mut chars = line.chars();
        let mut position: (i32, i32) = (0, 0);
        while let Some(c) = chars.next() {
            match c {
                'e' => position.0 += 1,
                'w' => position.0 -= 1,
                'n' => {
                    let c2 = chars.next().expect("Malformed input");
                    if c2 == 'e' && position.1.abs() % 2 == 0 {
                        position.0 += 1;
                    } else if c2 == 'w' && position.1.abs() % 2 == 1 {
                        position.0 -= 1;
                    }
                    position.1 += 1;
                }
                's' => {
                    let c2 = chars.next().expect("Malformed input");
                    if c2 == 'e' && position.1.abs() % 2 == 0 {
                        position.0 += 1;
                    } else if c2 == 'w' && position.1.abs() % 2 == 1 {
                        position.0 -= 1;
                    }
                    position.1 -= 1;
                }
                _ => panic!("Malformed input"),
            }
        }
        if black_tiles.contains(&position) {
            black_tiles.remove(&position);
        } else {
            black_tiles.insert(position);
        }
    }
    black_tiles
}

pub fn solve_part_2(input: &str) -> usize {
    let mut state = initial_state(input);

    for _ in 0..100 {
        let mut changes: HashMap<(i32, i32), u32> = HashMap::new();
        for p in state.iter() {
            for n in p.neighbours() {
                if let Some(p2) = changes.get_mut(&n) {
                    *p2 += 1;
                } else {
                    changes.insert(n, 1);
                }
            }
        }
        state = changes
            .into_iter()
            .filter(|(p, c)| {
                if state.contains(p) {
                    *c == 1 || *c == 2
                } else {
                    *c == 2
                }
            })
            .map(|(p, _)| p)
            .collect();
    }
    state.len()
}

trait Node {
    fn neighbours(&self) -> Vec<Self> where Self: Sized;
}

impl Node for (i32, i32) {
    fn neighbours(&self) -> Vec<Self> {
        vec![
            (self.0 + 1, self.1),
            (self.0 - 1, self.1),
            if self.1.abs() % 2 == 0 {
                (self.0 + 1, self.1 + 1)
            } else {
                (self.0, self.1 + 1)
            },
            if self.1.abs() % 2 == 1 {
                (self.0 - 1, self.1 + 1)
            } else {
                (self.0, self.1 + 1)
            },
            if self.1.abs() % 2 == 0 {
                (self.0 + 1, self.1 - 1)
            } else {
                (self.0, self.1 - 1)
            },
            if self.1.abs() % 2 == 1 {
                (self.0 - 1, self.1 - 1)
            } else {
                (self.0, self.1 - 1)
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use crate::prob24::{solve_part_1, solve_part_2};

    const TESTCASE: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1("esewnw"), 1);
        assert_eq!(solve_part_1(TESTCASE), 10);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TESTCASE), 2208);
    }
}
