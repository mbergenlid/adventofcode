use std::collections::HashSet;

pub fn solve_part_1(input: &str) -> usize {
    let ranges: Vec<_> = input
        .split(",")
        .map(|r| {
            let (lower, upper) = r.trim().split_once("-").expect("Invalid input");

            lower.parse::<u64>().expect(lower)..=(upper.parse::<u64>().expect(upper))
        })
        .collect();

    let max = ranges
        .iter()
        .map(|r| r.end())
        .max()
        .expect("Must contain at least one element");

    let mut sum = 0;
    for invalid in InvalidIdIterator::new().take_while(|n| n <= max) {
        let is_in_range = ranges.iter().any(|r| r.contains(&invalid));
        if is_in_range {
            sum += invalid;
        }
    }
    sum as usize
}

pub fn solve_part_2(input: &str) -> usize {
    let ranges: Vec<_> = input
        .split(",")
        .map(|r| {
            let (lower, upper) = r.trim().split_once("-").expect("Invalid input");

            lower.parse::<u64>().expect(lower)..=(upper.parse::<u64>().expect(upper))
        })
        .collect();

    let max = *ranges
        .iter()
        .map(|r| r.end())
        .max()
        .expect("Must contain at least one element");

    let mut sum = 0;
    for invalid in InvalidIdIteratorPart2::new(max) {
        let is_in_range = ranges.iter().any(|r| r.contains(&invalid));
        if is_in_range {
            sum += invalid;
        }
    }
    sum as usize
}

struct InvalidIdIterator {
    current: u64,
}

impl InvalidIdIterator {
    fn new() -> Self {
        Self { current: 0 }
    }
}

impl Iterator for InvalidIdIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;

        Some(self.current * (10_u32.pow(self.current.ilog10() + 1) as u64) + self.current)
    }
}

struct InvalidIdIteratorPart2 {
    current: u64,
    current_log: u32,
    next: u64,
    upper_bound: u64,
    visited: HashSet<u64>,
}

impl InvalidIdIteratorPart2 {
    fn new(upper_bound: u64) -> Self {
        Self {
            current: 1,
            current_log: 10,
            next: 1,
            upper_bound,
            visited: HashSet::default(),
        }
    }
}

impl Iterator for InvalidIdIteratorPart2 {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next * self.current_log as u64 + self.current;

        if next > self.upper_bound {
            self.current += 1;
            self.next = self.current;
            self.current_log = 10_u32.pow(self.current.ilog10() + 1);
            if self.next * self.current_log as u64 + self.current > self.upper_bound {
                return None;
            }
            return self.next();
        }

        self.next = next;
        if self.visited.contains(&next) {
            return self.next();
        }
        self.visited.insert(next);

        Some(next)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashSet;

    use crate::prob2::{solve_part_1, solve_part_2, InvalidIdIterator, InvalidIdIteratorPart2};

    #[test]
    fn test_iterator() {
        let mut it = InvalidIdIterator::new();
        let res = it.take(10).collect::<Vec<_>>();
        assert_eq!(res, vec![11, 22, 33, 44, 55, 66, 77, 88, 99, 1010]);

        let mut it = InvalidIdIterator::new();
        let res = it.skip_while(|n| *n < 100000).take(3).collect::<Vec<_>>();
        assert_eq!(res, vec![100100, 101101, 102102]);
    }

    #[test]
    fn test_iterator_part2() {
        let mut it = InvalidIdIteratorPart2::new(10_000);
        let res = it.take(10).collect::<Vec<_>>();
        assert_eq!(res, vec![11, 111, 1111, 22, 222, 2222, 33, 333, 3333, 44]);

        let x = InvalidIdIteratorPart2::new(100).take(1000).count();
        assert!(x < 1000);

        let mut it = InvalidIdIteratorPart2::new(100_000);
        let res = it.collect::<Vec<_>>();
        assert_eq!(
            HashSet::<u64>::from_iter(res.iter().copied()).len(),
            res.len(),
            "{res:?}",
        );
        //        let mut it = InvalidIdIterator::new();
        //        let res = it.skip_while(|n| *n < 100000).take(3).collect::<Vec<_>>();
        //        assert_eq!(res, vec![100100, 101101, 102102]);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 1227775554);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), 4174379265);
    }
    const INPUT: &'static str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
}
