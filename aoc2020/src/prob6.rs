use std::ops::{BitAnd, BitOr};

pub fn solve_part_1(_: &str) -> u32 {
    include_str!("../inputs/prob6")
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|line| CustomsDeclaration::from(line))
                .fold(CustomsDeclaration(u32::MIN), |x, y| x | y)
                .len()
        })
        .sum()
}

pub fn solve_part_2(_: &str) -> u32 {
    include_str!("../inputs/prob6")
        .split("\n\n")
        .map(|s| s.lines()
            .map(|line| CustomsDeclaration::from(line))
            .fold(CustomsDeclaration(u32::MAX), |x, y| x & y)
            .len())
        .sum()
}

struct CustomsDeclaration(u32);

impl BitAnd for CustomsDeclaration {
    type Output = CustomsDeclaration;

    fn bitand(self, rhs: Self) -> Self::Output {
        CustomsDeclaration(self.0 & rhs.0)
    }
}

impl BitOr for CustomsDeclaration {
    type Output = CustomsDeclaration;

    fn bitor(self, rhs: Self) -> Self::Output {
        CustomsDeclaration(self.0 | rhs.0)
    }
}

impl From<&str> for CustomsDeclaration {
    fn from(s: &str) -> Self {
        let mut result: u32 = 0;
        for c in s.chars() {
            result = result | (1 << (c as u8 - 'a' as u8) as u32);
        }
        CustomsDeclaration(result)
    }
}

impl CustomsDeclaration {
    fn len(&self) -> u32 {
        let mut count = 0;
        let mut x = self.0;
        while x > 0 {
            count += 1;
            x &= x - 1;
        }
        count
    }
}
