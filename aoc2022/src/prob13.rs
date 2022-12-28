use crate::prob13::Packet::{List, Literal};
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Clone)]
enum Packet {
    Literal(u32),
    List(Vec<Packet>),
}

impl Debug for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Literal(v) => write!(f, "{}", v),
            List(list) => write!(f, "{:?}", list),
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Literal(left), Literal(right)) => left.cmp(right),
            (Literal(left), right @ List(_)) => List(vec![Literal(*left)]).cmp(right),
            (left @ List(_), Literal(right)) => left.cmp(&List(vec![Literal(*right)])),
            (List(left), List(right)) => left.cmp(right),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Packet {
    fn parse(s: &str) -> IResult<&str, Packet> {
        let literal = nom::character::complete::u32;
        let list = terminated(
            preceded(tag("["), separated_list0(char(','), Packet::parse)),
            tag("]"),
        );

        alt((map(literal, Packet::Literal), map(list, Packet::List)))(s)
    }
}

pub fn solve_part_1(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|line| {
            let mut split = line.split("\n");
            (
                Packet::parse(split.next().unwrap()).unwrap(),
                Packet::parse(split.next().unwrap()).unwrap(),
            )
        })
        .enumerate()
        .filter(|(_, (p1, p2))| {
            println!("{:?} -- {:?} = {:?}", p1.1, p2.1, p1.cmp(p2));

            p1.cmp(p2) != Ordering::Greater
        })
        .map(|(i, _)| i + 1)
        .sum::<usize>()
}

pub fn solve_part_2(input: &str) -> usize {
    let mut packets: Vec<Packet> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Packet::parse(line).unwrap().1)
        .collect();

    let (_, divider_packet_1) = Packet::parse("[[2]]").unwrap();
    let (_, divider_packet_2) = Packet::parse("[[6]]").unwrap();
    packets.push(divider_packet_1.clone());
    packets.push(divider_packet_2.clone());

    packets
        .into_iter()
        .sorted()
        .enumerate()
        .filter(|(_, p)| *p == divider_packet_1 || *p == divider_packet_2)
        .map(|(index, _)| index + 1)
        .product::<usize>()
}

#[cfg(test)]
mod test {
    use crate::prob13::{solve_part_1, solve_part_2};

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 13);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(INPUT), 140);
    }

    const INPUT: &'static str = r"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";
}
