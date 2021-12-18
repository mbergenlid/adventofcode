use nom::error::Error;
use nom::IResult;

use crate::prob16::OperatorType::{GreaterThan, Max, Min, Product, Sum, LessThan, Equal};
use crate::prob16::PacketType::{LITERAL, OPERATOR};
use itertools::Itertools;
use nom::bits::{bits, streaming::take};
use nom::combinator::{map, map_res};
use std::iter::Iterator;
use std::panic::panic_any;
use nom::sequence::tuple;

pub fn solve_part_1(input: &str) -> usize {
    let bits = to_bits(input);
    let packet = packet(bits.as_slice())
        .map(|(_, p)| p)
        .unwrap_or_else(|_| panic!());

    packet.version_sum()
}

pub fn solve_part_2(input: &str) -> usize {
    let packet = packet(to_bits(input).as_slice())
        .map(|(_, p)| p)
        .unwrap_or_else(|_| panic!());
    packet.evaluate()
}

fn to_bits(input: &str) -> Vec<u8> {
    let input: Vec<_> = input
        .chars()
        .filter(|c| !c.is_whitespace())
        .map(|c| c.to_digit(16).unwrap())
        .collect();
    let mut bits = Vec::new();
    for chunk in input.chunks(2) {
        bits.push(((chunk[0] << 4) | chunk[1]) as u8);
    }
    bits
}

#[derive(Eq, PartialEq, Debug)]
struct Packet {
    version: u8,
    packet_type: PacketType,
}

impl Packet {
    fn version_sum(&self) -> usize {
        self.version as usize
            + match &self.packet_type {
                OPERATOR(_, sub_packets) => sub_packets.iter().map(Packet::version_sum).sum(),
                LITERAL(_) => 0,
            }
    }

    fn evaluate(&self) -> usize {
        match &self.packet_type {
            LITERAL(value) => *value,
            OPERATOR(tpe, packets) => match tpe {
                Sum => packets.iter().map(Packet::evaluate).sum(),
                Product => packets.iter().map(Packet::evaluate).product(),
                Min => packets
                    .iter()
                    .map(Packet::evaluate)
                    .min()
                    .expect("Yo dude, no sub packets."),
                Max => packets
                    .iter()
                    .map(Packet::evaluate)
                    .max()
                    .expect("Yo dude, no sub packets."),
                GreaterThan => {
                    if packets[0].evaluate() > packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                },
                LessThan => {
                    if packets[0].evaluate() < packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                },
                Equal => {
                    if packets[0].evaluate() == packets[1].evaluate() {
                        1
                    } else {
                        0
                    }
                }
            },
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum PacketType {
    LITERAL(usize),
    OPERATOR(OperatorType, Vec<Packet>),
}

#[derive(Eq, PartialEq, Debug)]
enum OperatorType {
    Sum,
    Product,
    Min,
    Max,
    GreaterThan,
    LessThan,
    Equal,
}

fn packet(input: &[u8]) -> IResult<&[u8], Packet> {
    let (rest, version): (_, Packet) =
        bits::<_, _, Error<(&[u8], usize)>, _, _>(parse_packet)(input)?;
    Ok((rest, version))
}

fn parse_packet(input: (&[u8], usize)) -> IResult<(&[u8], usize), Packet> {
    let (rest, (version, type_id)): (_, (u8, u8)) = tuple((take(3usize), take(3usize)))(input)?;

    let (rest, packet_type): (_, PacketType) = match type_id {
        4 => {
            let mut number: usize = 0;
            let mut mut_rest = rest;
            loop {
                let (rest, keep_going): (_, u8) = take(1usize)(mut_rest)?;
                let (rest, next): (_, usize) = take(4usize)(rest)?;
                number = (number << 4) | next;
                if keep_going == 0 {
                    break (rest, LITERAL(number));
                }
                mut_rest = rest
            }
        }
        operator_type => {
            let (rest, type_id): (_, u8) = take(1usize)(rest)?;
            let (rest, sub_packets) = if type_id == 0 {
                let (rest, mut length): (_, usize) = take(15usize)(rest)?;
                let mut sub_packets = Vec::new();
                let mut mut_rest = rest;
                while length > 0 {
                    let start = mut_rest.0.len() * 8 - mut_rest.1;
                    let (rest, packet) = parse_packet(mut_rest)?;
                    let bits_consumed = start - (rest.0.len() * 8 - rest.1);
                    length -= bits_consumed;
                    sub_packets.push(packet);
                    mut_rest = rest;
                }
                (mut_rest, sub_packets)
            } else {
                let (rest, number_of_subpackets): (_, u16) = take(11usize)(rest)?;
                let mut sub_packets = Vec::new();
                let mut mut_rest = rest;
                for _ in 0..number_of_subpackets {
                    let (rest, packet) = parse_packet(mut_rest)?;
                    sub_packets.push(packet);
                    mut_rest = rest;
                }
                (mut_rest, sub_packets)
            };
            let operator = match operator_type {
                0 => Sum,
                1 => Product,
                2 => Min,
                3 => Max,
                5 => GreaterThan,
                6 => LessThan,
                7 => Equal,
                _ => panic!("Unknown operator type: {}", operator_type),
            };
            (rest, OPERATOR(operator, sub_packets))
        }
    };
    // println!("{:?}, {:X?}", packet_type, rest);
    Ok((
        rest,
        Packet {
            version,
            packet_type,
        },
    ))
}

#[cfg(test)]
mod test {
    use crate::prob16::OperatorType::{Sum, Max, LessThan};
    use crate::prob16::Packet;
    use crate::prob16::PacketType::{LITERAL, OPERATOR};

    #[test]
    fn test_literal() {
        assert_eq!(
            super::packet(vec![0xD2, 0xFE, 0x28].as_slice()),
            Ok((
                vec![].as_slice(),
                Packet {
                    version: 6,
                    packet_type: LITERAL(2021)
                }
            ))
        );
    }

    #[test]
    fn test_op_1() {
        assert_eq!(
            super::packet(vec![0x38, 0x00, 0x6F, 0x45, 0x29, 0x12, 0x00].as_slice()),
            Ok((
                vec![].as_slice(),
                Packet {
                    version: 1,
                    packet_type: OPERATOR(
                        LessThan,
                        vec![
                            Packet {
                                version: 6,
                                packet_type: LITERAL(10)
                            },
                            Packet {
                                version: 2,
                                packet_type: LITERAL(20)
                            }
                        ]
                    )
                }
            ))
        );
    }

    #[test]
    fn test_op_2() {
        assert_eq!(
            super::packet(vec![0xEE, 0x00, 0xD4, 0x0C, 0x82, 0x30, 0x60].as_slice()),
            Ok((
                vec![].as_slice(),
                Packet {
                    version: 7,
                    packet_type: OPERATOR(
                        Max,
                        vec![
                            Packet {
                                version: 2,
                                packet_type: LITERAL(1)
                            },
                            Packet {
                                version: 4,
                                packet_type: LITERAL(2)
                            },
                            Packet {
                                version: 1,
                                packet_type: LITERAL(3)
                            }
                        ]
                    )
                }
            ))
        );
    }

    #[test]
    fn test_1() {
        assert_eq!(super::solve_part_1("8A004A801A8002F478"), 16);
        assert_eq!(super::solve_part_1("620080001611562C8802118E34"), 12);
        assert_eq!(super::solve_part_1("C0015000016115A2E0802F182340"), 23);
        assert_eq!(super::solve_part_1("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_2() {
        assert_eq!(super::solve_part_2("C200B40A82"), 3);
        assert_eq!(super::solve_part_2("04005AC33890"), 54);
        assert_eq!(super::solve_part_2("880086C3E88112"), 7);
        assert_eq!(super::solve_part_2("CE00C43D881120"), 9);
        assert_eq!(super::solve_part_2("D8005AC2A8F0"), 1);
        assert_eq!(super::solve_part_2("F600BC2D8F"), 0);
        assert_eq!(super::solve_part_2("9C005AC2F8F0"), 0);
        assert_eq!(super::solve_part_2("9C0141080250320F1802104A08"), 1);
    }

    const TESTCASE: &'static str = "";
}
