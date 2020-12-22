use std::collections::{HashSet, VecDeque};

pub fn solve_part_1(input: &str) -> u32 {
    let mut iter = input.split("\n\n");
    let mut p1 = iter.next().unwrap().lines().skip(1).map(|l| l.trim().parse::<u32>().unwrap()).collect::<VecDeque<_>>();
    let mut p2 = iter.next().unwrap().lines().skip(1).map(|l| l.trim().parse::<u32>().unwrap()).collect::<VecDeque<_>>();

    while !p1.is_empty() && !p2.is_empty() {
        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();

        if p1_card > p2_card {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }
    }

    let winner = if p1.is_empty() {
        p2
    } else {
        p1
    };

    let mut res = 0;
    let mut mul = winner.len() as u32;
    for x in winner {
        res = res + x*mul;
        mul -= 1;
    }
    res
}

pub fn solve_part_2(input: &str) -> u32 {
    let mut iter = input.split("\n\n");
    let mut p1 = iter.next().unwrap().lines().skip(1).map(|l| l.trim().parse::<u32>().unwrap()).collect::<VecDeque<_>>();
    let mut p2 = iter.next().unwrap().lines().skip(1).map(|l| l.trim().parse::<u32>().unwrap()).collect::<VecDeque<_>>();
    let winner = if recursive_comet(&mut p1, &mut p2) {
        p1
    } else {
        p2
    };

    let mut res = 0;
    let mut mul = winner.len() as u32;
    for x in winner {
        res = res + x*mul;
        mul -= 1;
    }
    res
}

fn recursive_comet(p1: &mut VecDeque<u32>, p2: &mut VecDeque<u32>) -> bool {
    let mut p1_states: HashSet<VecDeque<u32>> = HashSet::new();
    let mut p2_states: HashSet<VecDeque<u32>> = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        if p1_states.contains(&p1) || p2_states.contains(&p2) {
            return true;
        }
        p1_states.insert(p1.clone());
        p2_states.insert(p2.clone());

        let p1_card = p1.pop_front().unwrap();
        let p2_card = p2.pop_front().unwrap();

        let p1_winner = if p1_card as usize <= p1.len() && p2_card as usize <= p2.len() {
            recursive_comet(
                &mut p1.iter().take(p1_card as usize).cloned().collect(),
                &mut p2.iter().take(p2_card as usize).cloned().collect(),
            )
        } else {
            p1_card > p2_card
        };
        if p1_winner {
            p1.push_back(p1_card);
            p1.push_back(p2_card);
        } else {
            p2.push_back(p2_card);
            p2.push_back(p1_card);
        }
    }

    if p1.is_empty() {
        false
    } else {
        true
    }
}

#[cfg(test)]
mod test {
    use crate::prob22::solve_part_2;

    const TESTCASE: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TESTCASE), 291);
    }
}

