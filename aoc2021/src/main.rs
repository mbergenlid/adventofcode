
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

aoc! {
    1 => prob1,
    2 => prob2,
    3 => prob3
}
