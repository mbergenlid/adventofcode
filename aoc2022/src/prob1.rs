use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {

    let mut max = 0;
    let mut current: u32 = 0;
    for line in input.lines() {
        if line.is_empty() {
            max = std::cmp::max(max, current);
            current = 0;
        } else {
            current += line.parse::<u32>().unwrap();
        }
    }

    max as usize
}

pub fn solve_part_2(input: &str) -> usize {
    let mut elves = Vec::new();
    let mut current: usize = 0;
    for line in input.lines() {
        if line.is_empty() {
            elves.push(current);
            current = 0;
        } else {
            current += line.parse::<usize>().unwrap();
        }
    }

    elves.iter().sorted().rev().take(3).sum()
}
