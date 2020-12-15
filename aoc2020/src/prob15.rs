
pub fn solve_part_1(input: &str) -> usize {
    solve(input, 2020)
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, 30000000)
}

fn solve(input: &str, n: usize) -> usize {
    let vec: Vec<_> = input.trim().split(",").map(|s| s.parse::<usize>().unwrap()).collect();
    let length = vec.len();
    let mut memory = MemoryGame::new(vec, n);
    memory.nth(n-length-1).unwrap()
}

struct MemoryGame {
    numbers: Vec<usize>,
    last_element: (usize, usize),
}

impl MemoryGame {
    fn new(numbers: Vec<usize>, capacity: usize) -> MemoryGame {
        let mut vec = vec![0; capacity];
        for (i, &n) in numbers.iter().enumerate() {
            vec[n] = i+1;
        }
        MemoryGame {
            numbers: vec,
            last_element: numbers.last().map(|n| (*n, numbers.len())).unwrap(),
        }
    }
}

impl Iterator for MemoryGame {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (last_number, last_index) = self.last_element;
        let index = self.numbers[last_number as usize];
        let next_number = if index > 0 {
            last_index - index
        } else {
            0
        };
        if next_number >= self.numbers.len() {
            None
        } else {
            self.numbers[last_number as usize] = last_index;
            self.last_element = (next_number, last_index+1);
            Some(next_number)
        }
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
