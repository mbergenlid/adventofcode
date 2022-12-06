use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    for index in 0..input.len()-4 {
        let range = &input[index..index+4];
        if is_unique(range) {
            return index+4;
        }
    }
    panic!("Not found")
}

fn is_unique(input: &str) -> bool {
    !input.is_empty() && input.chars().all_unique()
}

pub fn solve_part_2(input: &str) -> usize {
    for index in 0..input.len()-14 {
        let range = &input[index..index+14];
        if is_unique(range) {
            return index+14;
        }
    }
    panic!("Not found")
}
