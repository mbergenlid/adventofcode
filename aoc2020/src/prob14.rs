use regex::Regex;
use serde::export::PhantomData;
use serde::Deserialize;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::fmt::Debug;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve_part_1(input: &str) -> u64 {
    solve::<Part1>(input)
}

pub fn solve_part_2(input: &str) -> u64 {
    solve::<Part2>(input)
}

fn solve<P>(input: &str) -> u64
where
    P: Part<P>,
    BitMask<P>: FromStr,
    <BitMask<P> as FromStr>::Err: Debug,
{
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut current_mask: BitMask<P> = input.lines().next().unwrap().parse().unwrap();
    for line in input.lines().skip(1) {
        if line.starts_with("mask") {
            current_mask = line.parse().unwrap();
        } else {
            let op = line.parse::<MemoryOperation>().unwrap();
            P::update_memory(&mut memory, &current_mask, &op);
        }
    }
    memory.values().sum()
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"mask = (\w+)").unwrap();
}

#[derive(Deserialize, Recap, Eq, PartialEq, Debug)]
#[recap(regex = r#"mem\[(?P<location>\d+)\] = (?P<value>\d+)"#)]
struct MemoryOperation {
    location: u64,
    value: u64,
}

trait Part<P> {
    fn update_memory(mem: &mut HashMap<u64, u64>, mask: &BitMask<P>, op: &MemoryOperation);
}
struct Part1;
struct Part2;

impl Part<Part1> for Part1 {
    fn update_memory(
        mem: &mut HashMap<u64, u64, RandomState>,
        mask: &BitMask<Part1>,
        op: &MemoryOperation,
    ) {
        for value in mask.apply(op.value) {
            mem.insert(op.location, value);
        }
    }
}

impl Part<Part2> for Part2 {
    fn update_memory(
        mem: &mut HashMap<u64, u64, RandomState>,
        mask: &BitMask<Part2>,
        op: &MemoryOperation,
    ) {
        for location in mask.apply(op.location) {
            mem.insert(location, op.value);
        }
    }
}

struct BitMask<P> {
    or_masks: u64,
    and_masks: Vec<u64>,
    phantom: PhantomData<P>,
}

impl<P> BitMask<P> {
    fn apply(&self, value: u64) -> impl Iterator<Item = u64> + '_ {
        let value = self.or_masks | value;
        self.and_masks.iter().map(move |&m| m & value)
    }
}

impl FromStr for BitMask<Part1> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = &REGEX.captures(s).unwrap()[1];

        Ok(BitMask {
            and_masks: vec![u64::from_str_radix(x.replace("X", "1").as_str(), 2)?],
            or_masks: u64::from_str_radix(x.replace("X", "0").as_str(), 2)?,
            phantom: PhantomData,
        })
    }
}

impl FromStr for BitMask<Part2> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x = &REGEX.captures(s).unwrap()[1];

        let init_mask = u64::from_str_radix(&x.replace("X", "1"), 2)?;
        let mut masks: Vec<u64> = vec![0];
        for c in x.chars() {
            if c == 'X' {
                for i in 0..masks.len() {
                    masks[i] = masks[i] * 2 + 0;
                    masks.push(masks[i] + 1);
                }
            } else {
                for mask in masks.iter_mut() {
                    *mask = *mask * 2 + 1;
                }
            }
        }

        Ok(BitMask {
            or_masks: init_mask,
            and_masks: masks,
            phantom: PhantomData,
        })
    }
}

#[cfg(test)]
mod test {
    use crate::prob14::{solve_part_1, solve_part_2, MemoryOperation};

    #[test]
    fn test_parse() {
        assert_eq!(
            "mem[3] = 11".parse::<MemoryOperation>().unwrap(),
            MemoryOperation {
                location: 3,
                value: 11
            }
        );
    }

    const TESTCASE: &'static str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(TESTCASE), 165);
        assert_eq!(
            solve_part_1(include_str!("../inputs/prob14")),
            8570568288597
        );
    }

    const TESTCASE_2: &'static str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(TESTCASE_2), 208);
        assert_eq!(
            solve_part_2(include_str!("../inputs/prob14")),
            3289441921203
        );
    }
}
