use itertools::Itertools;

struct Mod {
    m: usize,
    value: usize,
}

impl Mod {
    fn new(value: usize, m: usize) -> Self {
        Mod { value, m }
    }

    fn inc(&mut self) -> usize {
        self.value = (self.value + 1) % self.m;
        self.value
    }

    fn dec(&mut self) -> usize {
        if self.value == 0 {
            self.value = self.m - 1;
        } else {
            self.value = self.value - 1;
        }
        self.value
    }
}

pub fn solve_part_1(input: &str) -> i64 {
    let original = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    solve(original, 1)
}

pub fn solve_part_2(input: &str) -> i64 {
    let original = input
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .map(|n| n * 811589153)
        .collect::<Vec<_>>();
    solve(original, 10)
}

fn solve(original: Vec<i64>, rounds: usize) -> i64 {
    let modulo = original.len();

    let mut copy = original.iter().enumerate().collect::<Vec<_>>();

    // println!("{:?}", original);
    for _ in 0..rounds {
        for original_index in 0..original.len() {
            let (index, &(_, &n)) = copy
                .iter()
                .enumerate()
                .find(|(_, (i, _))| *i == original_index)
                .expect("Error");

            if n >= 0 {
                let mut i = Mod::new(index, modulo);
                for _ in 0..(n % ((modulo as i64) - 1)) {
                    let current = i.value;
                    copy.swap(current, i.inc());
                }
            } else {
                let mut i = Mod::new(index, modulo);
                for _ in 0..(n.abs() % ((modulo as i64) - 1)) {
                    let current = i.value;
                    copy.swap(current, i.dec());
                }
            }
        }
    }

    let (index, _) = copy.iter().find_position(|(_, &x)| x == 0).unwrap();

    copy[(index + 1000) % modulo].1
        + copy[(index + 2000) % modulo].1
        + copy[(index + 3000) % modulo].1
}

#[cfg(test)]
mod test {
    use crate::prob20::{solve_part_1, solve_part_2};

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 3);
    }

    #[test]
    fn test_1_2() {
        assert_eq!(solve_part_1(INPUT_2), 0);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(INPUT), 1623178306);
    }

    const INPUT: &'static str = r"1
2
-3
3
-2
0
4
";

    const INPUT_2: &'static str = r"0
-1
-1
1";
}
