use std::str::FromStr;

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Equation>().expect("Invalid input"))
        .filter(|e| e.is_valid())
        .map(|e| e.result())
        .sum::<usize>()
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line.parse::<Equation>().expect("Invalid input"))
        .filter(|e| e.is_valid_2())
        .map(|e| e.result())
        .sum::<usize>()
}

struct Equation {
    result: usize,
    parts: Vec<usize>,
}

impl Equation {
    fn is_valid(&self) -> bool {
        Equation::_is_valid(self.result, self.parts[0], &self.parts[1..])
    }

    fn _is_valid(result: usize, partial: usize, parts: &[usize]) -> bool {
        if parts.is_empty() {
            return partial == result;
        }

        Equation::_is_valid(result, partial + parts[0], &parts[1..])
            || Equation::_is_valid(result, partial * parts[0], &parts[1..])
    }

    fn is_valid_2(&self) -> bool {
        Equation::_is_valid_2(self.result, self.parts[0], &self.parts[1..])
    }

    fn _is_valid_2(result: usize, partial: usize, parts: &[usize]) -> bool {
        if parts.is_empty() {
            return partial == result;
        }

        fn concat(n1: usize, n2: usize) -> usize {
            let log = (n2 as f64).log10() as u32;
            n1 * (10_usize.pow(log + 1)) + n2
        }

        Equation::_is_valid_2(result, partial + parts[0], &parts[1..])
            || Equation::_is_valid_2(result, partial * parts[0], &parts[1..])
            || Equation::_is_valid_2(result, concat(partial, parts[0]), &parts[1..])
    }

    fn result(&self) -> usize {
        self.result
    }
}

impl FromStr for Equation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (res, parts) = s
            .split(":")
            .collect_tuple::<(_, _)>()
            .ok_or_else(|| "Invalid".to_string())?;

        return Ok(Self {
            result: res.parse::<usize>().map_err(|_| format!("Not a number {}", res))?,
            parts: parts.trim().split(" ").map(|p| p.parse::<usize>().expect("Not a number")).collect_vec()
        });
    }
}


#[cfg(test)]
mod test {

    #[test]
    fn test_is_valid() {
        assert!(super::Equation::_is_valid(1, 1, &[]));
        assert!(!super::Equation::_is_valid(2, 1, &[]));
        assert!(super::Equation::_is_valid(2, 1, &[1]));
        assert!(super::Equation::_is_valid(2, 2, &[1]));

        assert!(super::Equation::_is_valid(4, 1, &[1, 2]));

        assert!(super::Equation::_is_valid_2(156, 15, &[6]));
    }

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 3749);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 11387);
    }

    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
}
