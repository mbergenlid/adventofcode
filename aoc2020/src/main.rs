#[macro_use]
extern crate lazy_static;
extern crate regex;

mod prob1;
mod prob2;
mod prob3;
mod prob4;
mod prob5;
mod prob6;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() == 0 {
        panic!("Need to specify a problem number");
    }

    let problem_num: u32 = args[1].parse::<u32>().unwrap();

    match problem_num {
        1 => {
            println!("Part 1: {}", prob1::solve_part_1());
            println!("Part 2: {}", prob1::solve_part_2());
        }
        2 => {
            println!("Part 1: {}", prob2::solve_part_1());
            println!("Part 2: {}", prob2::solve_part_2());
        }
        3 => {
            println!("Part 1: {}", prob3::solve_part_1());
            println!("Part 2: {}", prob3::solve_part_2());
        }
        4 => {
            println!("Part 1: {}", prob4::solve_part_1());
            println!("Part 2: {}", prob4::solve_part_2());
        }
        5 => {
            println!("Part 1: {}", prob5::solve_part_1());
            println!("Part 2: {}", prob5::solve_part_2());
        }
        6 => {
            println!("Part 1: {}", prob6::solve_part_1());
            println!("Part 2: {}", prob6::solve_part_2());
        }
        _ => panic!("Unknown problem number"),
    }
}
