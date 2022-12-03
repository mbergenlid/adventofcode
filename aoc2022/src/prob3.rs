use std::collections::HashSet;
use std::ops::BitAnd;
use itertools::Itertools;

trait Item {
    fn priority(&self) -> u32;
}

impl Item for char {
    fn priority(&self) -> u32 {
        if self.is_lowercase() {
            (*self as u32) - ('a' as u32) + 1
        } else {
            (*self as u32) - ('A' as u32) + 27
        }
    }
}


pub fn solve_part_1(input: &str) -> usize {
    input.lines().map(|line| {
        let num_items = line.len();
        let first_compartment: HashSet<_> = line.chars().take(num_items/2).collect();
        let second_compartment: HashSet<_> = line.chars().skip(num_items/2).collect();

        let intersection = first_compartment.bitand(&second_compartment);

        intersection.iter().map(|&c| c.priority()).sum::<u32>()
    }).sum::<u32>() as usize
}

pub fn solve_part_2(input: &str) -> usize {
    input.lines().chunks(3).into_iter()
        .map(|lines| {
            let intersection = lines.into_iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|set1, set2| set1.bitand(&set2))
                .unwrap();
            assert!(intersection.len() == 1);
            intersection.iter().next().unwrap().priority()
        }).sum::<u32>() as usize
}
