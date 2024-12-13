use std::str::FromStr;

use itertools::Itertools;
use nalgebra::{Matrix2, Matrix2x1};

pub fn solve_part_1(input: &str) -> usize {
    solve::<0>(input)
}

pub fn solve_part_2(input: &str) -> usize {
    solve::<10000000000000>(input)
}

fn solve<const N: u64>(input: &str) -> usize {
    let machines = input
        .split("\n\n")
        .map(|i| i.parse::<ClawMachine<N>>().unwrap())
        .collect_vec();

    let mut result = 0;
    for m in machines {
        if let Some(cost) = m.solve() {
            result += cost;
        }
    }
    result as usize
}

#[derive(Debug)]
struct ClawMachine<const N: u64 = 0> {
    button_a: (u64, u64),
    button_b: (u64, u64),
    prize: (u64, u64),
}

impl<const N: u64> ClawMachine<N> {
    fn solve(&self) -> Option<u64> {
        let m1 = Matrix2::new(
            self.button_a.0 as f64,
            self.button_b.0 as f64,
            self.button_a.1 as f64,
            self.button_b.1 as f64,
        );
        let solutions = Matrix2x1::new(self.prize.0 as f64, self.prize.1 as f64);
        match m1.try_inverse() {
            Some(inv) => {
                let res = inv * solutions;
                let a = res.get(0).expect("").round() as u64;
                let b = res.get(1).expect("").round() as u64;

                if a * self.button_a.0 + b * self.button_b.0 == self.prize.0
                    && a * self.button_a.1 + b * self.button_b.1 == self.prize.1
                {
                    return Some(a * 3 + b);
                }
                None
            }
            None => None,
        }
    }
}

impl<const N: u64> FromStr for ClawMachine<N> {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (line1, line2, line3) = s
            .lines()
            .collect_tuple::<(_, _, _)>()
            .ok_or_else(|| "ClawMachine has to contain 3 lines".to_string())?;
        let line1_comma = line1
            .find(',')
            .ok_or_else(|| format!("Invalid line {}", line1))?;
        let m11: u64 = line1["Button A: X+".len()..line1_comma].parse().unwrap();
        let m21: u64 = line1[(line1_comma + 4)..].trim().parse().unwrap();

        let line2_comma = line2
            .find(',')
            .ok_or_else(|| format!("Invalid line {}", line2))?;
        let m12: u64 = line2["Button B: X+".len()..line2_comma].parse().unwrap();
        let m22: u64 = line2[(line2_comma + 4)..].trim().parse().unwrap();

        let line3_comma = line3
            .find(',')
            .ok_or_else(|| format!("Invalid line {}", line3))?;
        let b1: u64 = line3["Prize: X=".len()..line3_comma].parse().unwrap();
        let b2: u64 = line3[(line3_comma + 4)..].trim().parse().unwrap();

        Ok(Self {
            button_a: (m11, m21),
            button_b: (m12, m22),
            prize: (b1 + N, b2 + N),
        })
    }
}

#[cfg(test)]
mod test {

    use super::ClawMachine;

    #[test]
    fn test_solve() {
        assert_eq!(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400"
                .parse::<ClawMachine>()
                .unwrap()
                .solve(),
            Some(80 * 3 + 40)
        );
    }

    #[test]
    fn test_solve_part_2() {
        assert_eq!(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=10000000008400, Y=10000000005400"
                .parse::<ClawMachine>()
                .unwrap()
                .solve(),
            None
        );

        println!(
            "{:?}",
            "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176"
                .parse::<ClawMachine<10000000000000>>()
                .unwrap()
        );
        assert_eq!(
            "Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176"
                .parse::<ClawMachine<10000000000000>>()
                .unwrap()
                .solve(),
            Some(459236326669)
        );
    }

    #[test]
    fn test_solve_3() {
        assert_eq!(
            "Button A: X+13, Y+99
Button B: X+84, Y+41
Prize: X=3193, Y=10546"
                .parse::<ClawMachine>()
                .unwrap()
                .solve(),
            Some(314)
        );
    }

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 480);
    }

    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
}
