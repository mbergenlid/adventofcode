use crate::prob18::SnailFishNumber::{Literal, Pair};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, digit1};
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::fmt::{Debug, Formatter, Write};
use std::ops::Add;

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| *SnailFishNumber::parse(line))
        .reduce(|n1, n2| n1 + n2)
        .map(|s| s.magnitude())
        .unwrap()
}

pub fn solve_part_2(input: &str) -> usize {
    let numbers: Vec<_> = input
        .lines()
        .map(|line| *SnailFishNumber::parse(line))
        .collect();
    numbers
        .iter()
        .permutations(2)
        .map(|perms| perms[0].clone() + perms[1].clone())
        .map(|number| number.magnitude())
        .max()
        .unwrap()
}

#[derive(Eq, PartialEq, Clone)]
enum SnailFishNumber {
    Literal(i32),
    Pair((Box<SnailFishNumber>, Box<SnailFishNumber>)),
}

impl SnailFishNumber {
    fn magnitude(&self) -> usize {
        match self {
            Literal(value) => *value as usize,
            Pair((left, right)) => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn add_left(&mut self, value: i32) {
        match self {
            Literal(v) => *v += value,
            Pair((left, _)) => left.add_left(value),
        }
    }

    fn add_right(&mut self, value: i32) {
        match self {
            Literal(v) => *v += value,
            Pair((_, right)) => right.add_right(value),
        }
    }

    fn explode(&mut self) -> bool {
        self._explode(0).is_some()
    }

    fn _explode(&mut self, depth: u8) -> Option<(i32, i32)> {
        match self {
            Literal(_) => None,
            Pair((ref mut left, right)) => {
                if let (Literal(l_value), Literal(r_value)) = (left.as_ref(), right.as_ref()) {
                    if depth >= 4 {
                        let result = (*l_value, *r_value);
                        *self = Literal(0);
                        Some(result)
                    } else {
                        None
                    }
                } else if let Some((l_explode, r_explode)) = left._explode(depth + 1) {
                    right.add_left(r_explode);
                    Some((l_explode, 0))
                } else if let Some((l_explode, r_explode)) = right._explode(depth + 1) {
                    left.add_right(l_explode);
                    Some((0, r_explode))
                } else {
                    None
                }
            }
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Literal(value) => {
                if *value >= 10 {
                    let left = *value / 2;
                    let right = ((*value as f64) / 2f64).ceil() as i32;
                    *self = Pair((Box::new(Literal(left)), Box::new(Literal(right))));
                    true
                } else {
                    false
                }
            }
            Pair((left, right)) => {
                left.split() || right.split()
            }
        }
    }

    fn parse(s: &str) -> Box<SnailFishNumber> {
        let (_, number) = _parse(s).unwrap();

        fn _parse(input: &str) -> IResult<&str, Box<SnailFishNumber>> {
            alt((
                map(digit1, |s: &str| {
                    Box::new(Literal(s.parse::<i32>().unwrap()))
                }),
                map(
                    separated_pair(
                        preceded(tag("["), _parse),
                        char(','),
                        terminated(_parse, tag("]")),
                    ),
                    |(s1, s2)| Box::new(Pair((s1, s2))),
                ),
            ))(input)
        }
        number
    }
}

impl Add for SnailFishNumber {
    type Output = SnailFishNumber;

    fn add(self, other: Self) -> Self::Output {
        let mut result = Pair((Box::new(self), Box::new(other)));
        while result.explode() || result.split() {}
        result
    }
}

impl Debug for SnailFishNumber {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal(value) => f.write_fmt(format_args!("{}", value)),
            Pair((left, right)) => {
                f.write_char('[')?;
                left.fmt(f)?;
                f.write_char(',')?;
                right.fmt(f)?;
                f.write_char(']')
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob18::SnailFishNumber;
    use std::ops::Add;

    #[test]
    fn test_explode() {
        assert_eq!(
            {
                let mut number = SnailFishNumber::parse("[[[[[9,8],1],2],3],4]");
                number.explode();
                number
            },
            SnailFishNumber::parse("[[[[0,9],2],3],4]")
        );
        assert_eq!(
            {
                let mut number = SnailFishNumber::parse("[7,[6,[5,[4,[3,2]]]]]");
                number.explode();
                number
            },
            SnailFishNumber::parse("[7,[6,[5,[7,0]]]]")
        );

        assert_eq!(
            {
                let mut number = SnailFishNumber::parse("[[6,[5,[4,[3,2]]]],1]");
                number.explode();
                number
            },
            SnailFishNumber::parse("[[6,[5,[7,0]]],3]")
        );
    }

    #[test]
    fn test_split() {
        assert_eq!(
            {
                let mut number = SnailFishNumber::parse("[10,4]");
                number.split();
                number
            },
            SnailFishNumber::parse("[[5,5],4]")
        );
        assert_eq!(
            {
                let mut number = SnailFishNumber::parse("[11,4]");
                number.split();
                number
            },
            SnailFishNumber::parse("[[5,6],4]")
        );
    }

    #[test]
    fn test_add() {
        // assert_eq!(
        //     SnailFishNumer::parse("[[[[4,3],4],4],[7,[[8,4],9]]]").add(*SnailFishNumer::parse("[1,1]")),
        //     *SnailFishNumer::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        // );

        assert_eq!(
            SnailFishNumber::parse("[[[[4,3],4],4],[7,[[8,4],9]]]")
                .add(*SnailFishNumber::parse("[1,1]")),
            *SnailFishNumber::parse("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
        );
        assert_eq!(
            SnailFishNumber::parse("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]")
                .add(*SnailFishNumber::parse("[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]")),
            *SnailFishNumber::parse("[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
        )
    }

    #[test]
    fn test_1() {
        //         super::solve_part_1("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
        // [7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
        // [[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
        // [[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
        // [7,[5,[[3,8],[1,4]]]]
        // [[2,[2,2]],[8,[8,1]]]
        // [2,9]
        // [1,[[[9,3],9],[[9,0],[0,7]]]]
        // [[[5,[7,4]],7],1]
        // [[[[4,2],2],6],[8,7]]");
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 4140);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 3993);
    }

    const TESTCASE: &'static str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]";
}
