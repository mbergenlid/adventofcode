use std::collections::HashMap;

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let input = input
        .trim()
        .split(" ")
        .map(|n| n.parse::<usize>().expect("Invalid number"))
        .collect_vec();

    let mut result = 0;
    let mut cache = Cache::new();
    for stone in input {
        result += blink(stone, 25, &mut cache);
    }
    result
}

pub fn solve_part_2(input: &str) -> usize {
    let input = input
        .trim()
        .split(" ")
        .map(|n| n.parse::<usize>().expect("Invalid number"))
        .collect_vec();

    let mut result = 0;
    let mut cache = Cache::new();
    for stone in input {
        result += blink(stone, 75, &mut cache);
    }
    result
}

type Cache = HashMap<(usize, usize), usize>;

fn blink(stone: usize, times: usize, cache: &mut Cache) -> usize {
    if times == 0 {
        return 1;
    }

    if let Some(result) = cache.get(&(stone, times)) {
        return *result;
    }
    let result = if stone == 0 {
        blink(1, times - 1, cache)
    } else if stone.digits() % 2 == 0 {
        let (s1, s2) = stone.split_half();
        blink(s1, times - 1, cache) + blink(s2, times - 1, cache)
    } else {
        blink(stone * 2024, times - 1, cache)
    };

    cache.insert((stone, times), result);
    result
}

trait Base10Number {
    fn digits(&self) -> u32;

    fn split_half(&self) -> (Self, Self)
    where
        Self: Sized;
}

impl Base10Number for usize {
    fn digits(&self) -> u32 {
        ((*self as f64).log10() as u32) + 1
    }

    fn split_half(&self) -> (Self, Self)
    where
        Self: Sized,
    {
        let digits = self.digits();

        let divider = 10_usize.pow(digits / 2);

        (*self / divider, *self % divider)
    }
}

#[cfg(test)]
mod test {
    use crate::prob11::Base10Number;

    #[test]
    fn base10_number() {
        assert_eq!(10_usize.digits(), 2);
        assert_eq!(2456_usize.split_half(), (24, 56));
    }

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 55312);
    }

    const INPUT: &str = "125 17";
}
