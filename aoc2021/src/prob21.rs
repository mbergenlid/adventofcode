use std::{ops::Sub, usize};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let mut players = input
        .lines()
        .map(|p| {
            let (_, pos) = p.split(":").collect_tuple().unwrap();
            Player {
                current: pos.trim().parse::<usize>().unwrap() - 1,
                score: 0,
            }
        })
        .collect::<Vec<_>>();

    let mut die = (1..=100).cycle();
    let mut throw_count = 0;

    'outer: loop {
        for p in players.iter_mut() {
            let (d1, d2, d3) = die.next_tuple().unwrap();

            throw_count += 3;
            p.step(d1 + d2 + d3);
            if p.score >= 1000 {
                break 'outer;
            }
        }
    }

    return dbg!(players.into_iter().map(|p| p.score).min().unwrap()) * dbg!(throw_count);
}

// 1. p1: 1+1+1, p2: 1+1+1, p1: 1+1+1, ..., p1:1+1+1 => p1 wins
// 2. p1: 1+1+1,                       ..., p1:1+1+2 => p1 wins
// n. p1: 3+3+3, p2: 3+3+3,

// 1+1+1 = 3
// 1+1+2 = 4
// 1+1+3 = 5
// 1+2+1 = 4
// 1+3+1 = 5

// 3: 1
// 4: 3
// 5: 1+1+3=>3 1+2+2=>3, Total: 6
// 6: 1+2+3=>6, 2+2+2=1, Total: 7
// 7: 1+3+3=>3, 2+2+3=>3, Total: 6
// 8: 2+3+3=>3, Total: 3
// 9: 3+3+3=>1, Total: 1

// p1: p[ 21 ] = p[21-3] + p[21-4]*3 + p[21-5]*6 ...
// p1: p[ N ] //In how many universes can p1 have N points and it being p1's turn.
// p1_t1[ 0 ] = 0
// p2_t2[ 0 ] = 27

// p2_t2[ 20 ] = p2_t1[20] - p1 wins when p2 has 20 points
// p2_t1[ 18 ] = p2_t1[20-3] + p2_t1[20-4]*3

// p1_t2[ 1 ] = 0
// p2_t1[ 3 ] = p2[]

// s1[x][y] = How many universes does p1 have x points and p2 have y points and it is p1's turn
// s2[x][y] = How many universes does p1 have x points and p2 have y points and it is p2's turn

// s1[0][0] = 1
// s1[0][1] = 0
// s1[1][0] = 0

// s2[0][0] = 0
// s2[3][0] = s1[3-3]*1
// s1[20][18] = s2[20][18-3] * 1 + s2[20][18-4]*3 + ... + s2[20][18-7] * 1
// s1[20][21] =
pub fn solve_part_2(input: &str) -> usize {
    let mut s1 = [[[[0_u64; 10]; 10]; 31]; 31];
    let mut s2 = [[[[0_u64; 10]; 10]; 31]; 31];
    let players = input
        .lines()
        .map(|p| {
            let (_, pos) = p.split(":").collect_tuple().unwrap();
            Player {
                current: pos.trim().parse::<usize>().unwrap() - 1,
                score: 0,
            }
        })
        .collect::<Vec<_>>();

    s1[0][0][dbg!(players[0].current)][dbg!(players[1].current)] = 1;

    //s2[7][0][6][7] = 1 (s1[7-7][0][7-(1+3)][7]*1)
    //s2[1][0][0][7] = s1[1-1][0][7][7]*1 + s1[1-1][0][6][7]*3 + s1[1-1][0][5][7]*6 + s1[1-1][0][4][7]*7 + s1[1-1][0][3][7]*6

    //s2[1][0][1][7] = s1[1-2][0][]

    let multiplier = [1, 3, 6, 7, 6, 3, 1];

    for x in 0..31 {
        for y in 0..31 {
            for p1_pos in 0..10 {
                for p2_pos in 0..10 {
                    if p2_pos + 1 <= y && y - (p2_pos + 1) < 21 {
                        let score = p2_pos + 1;
                        for (i, mul) in multiplier.iter().enumerate() {
                            s1[x][y][p1_pos][p2_pos] +=
                                s2[x][y - score][p1_pos][sub_mod10(score - 1, i + 3)] * mul
                        }
                    }

                    if p1_pos + 1 <= x && (p1_pos + 1) < 21 {
                        let score = p1_pos + 1;
                        for (i, mul) in multiplier.iter().enumerate() {
                            s2[x][y][p1_pos][p2_pos] +=
                                s1[x - score][y][sub_mod10(score - 1, i + 3)][p2_pos] * mul
                        }
                    }
                }
            }
            //println!("x,y = ({},{}) :: s1 = {:?}", x, y, s1[x][y]);
            //println!("x,y = ({},{}) :: s2 = {:?}", x, y, s2[x][y]);
        }
    }
    let mut p1_wins: u64 = 0;
    for p1_win_score in 21..31 {
        for p2_score in 0..21 {
            for p1_end_pos in (p1_win_score - 21)..10 {
                p1_wins += s2[p1_win_score][p2_score][p1_end_pos].iter().sum::<u64>();
            }
        }
    }
    let mut p2_wins: u64 = 0;
    for p2_win_score in 21..31 {
        for p1_score in 0..21 {
            for p2_end_pos in (p2_win_score - 21)..10 {
                for p1_end_pos in 0..10 {
                    p2_wins += s1[p1_score][p2_win_score][p1_end_pos][p2_end_pos];
                }
            }
        }
    }

    dbg!(p1_wins).max(dbg!(p2_wins)) as usize
}

fn sub_mod10(a: usize, b: usize) -> usize {
    if b > a {
        let rest = b - a;
        return 10 - rest;
    } else {
        a - b
    }
}

struct Player {
    current: usize,
    score: usize,
}

impl Player {
    fn step(&mut self, steps: usize) {
        self.current += steps;
        self.current = self.current % 10;
        self.score += self.current + 1;
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 739785);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 444356092776315);
    }

    const TESTCASE: &'static str = "Player 1 starting position: 4
Player 2 starting position: 8";
}
