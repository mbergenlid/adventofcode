extern crate lazy_static;
extern crate regex;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate recap;
#[macro_use] extern crate lalrpop_util;

macro_rules! aoc {
    ( $($m:expr => $module:ident),* ) => {

        fn main() {
            let args: Vec<_> = std::env::args().collect();
            if args.len() == 0 {
                panic!("Need to specify a problem number");
            }

            let problem_num: u32 = args[1].parse::<u32>().unwrap();

            use std::time::Instant;
            match problem_num {
                $(
                    $m => {
                        let start = Instant::now();
                        let part1 = $module::solve_part_1(include_str!(concat!("../inputs/prob", $m)));
                        println!("Part 1: {} ({}µs)", part1, start.elapsed().as_micros());
                        let start = Instant::now();
                        let part2 = $module::solve_part_2(include_str!(concat!("../inputs/prob", $m)));
                        println!("Part 2: {} ({}µs)", part2, start.elapsed().as_micros());
                    }
                )*
                _ => panic!("Unknown problem number")
            }
        }
    }
}

mod prob1;
mod prob2;
mod prob3;
mod prob4;
mod prob5;
mod prob6;
mod prob7;
mod prob8;
mod prob9;
mod prob10;
mod prob11;
mod prob12;
mod prob13;
mod prob14;
mod prob15;
mod prob16;
mod prob17;
mod prob18;
mod prob19;
mod prob20;
mod prob21;

aoc! {
    1 => prob1,
    2 => prob2,
    3 => prob3,
    4 => prob4,
    5 => prob5,
    6 => prob6,
    7 => prob7,
    8 => prob8,
    9 => prob9,
    10 => prob10,
    11 => prob11,
    12 => prob12,
    13 => prob13,
    14 => prob14,
    15 => prob15,
    16 => prob16,
    17 => prob17,
    18 => prob18,
    19 => prob19,
    20 => prob20,
    21 => prob21
}
