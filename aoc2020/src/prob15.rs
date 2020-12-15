use std::collections::HashMap;

pub fn solve_part_1(input: &str) -> u64 {
    solve(input, 2020)
}

pub fn solve_part_2(input: &str) -> u64 {
    solve(input, 30000000)
}

fn solve(input: &str, n: usize) -> u64 {
    let vec: Vec<u64> = input.trim().split(",").map(|s| s.parse::<u64>().unwrap()).collect();
    let length = vec.len();
    let mut memory = MemoryGame::new(vec);
    memory.nth(n-length-1).unwrap()
}

struct MemoryGame {
    numbers: HashMap<u64, u64>,
    last_element: (u64, u64),
}

impl MemoryGame {
    fn new(numbers: Vec<u64>) -> MemoryGame {
        MemoryGame {
            numbers: numbers.iter().enumerate().map(|(i,n)| (*n, i as u64)).collect(),
            last_element: numbers.last().map(|n| (*n, (numbers.len()-1) as u64)).unwrap(),
        }
    }
}

impl Iterator for MemoryGame {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let (last_number, last_index) = self.last_element;
        let next_number = if let Some(index) = self.numbers.get_mut(&last_number) {
            let i = *index;
            *index = last_index;
            last_index - i
        } else {
            self.numbers.insert(last_number, last_index);
            0
        };
        self.last_element = (next_number, last_index+1);
        Some(next_number)
    }
}

#[cfg(test)]
mod test {
    use crate::prob15::solve_part_1;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1("0,3,6"), 436);
    }
}
