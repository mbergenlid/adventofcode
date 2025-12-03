pub fn solve_part_1(input: &str) -> usize {
    let mut zeroes = 0;
    let mut current: u8 = 50;
    for line in input.lines() {
        match &line[0..1] {
            "L" => current = current.left(line[1..].parse::<u32>().unwrap()).0,
            "R" => current = current.right(line[1..].parse::<u32>().unwrap()).0,
            _ => panic!("Invalid line {line}"),
        }

        if current == 0 {
            zeroes += 1;
        }
    }
    zeroes
}

pub fn solve_part_2(input: &str) -> usize {
    let mut zeroes = 0;
    let mut current: u8 = 50;
    for line in input.lines() {
        let (dial, zs) = match &line[0..1] {
            "L" => current.left(line[1..].parse::<u32>().unwrap()),
            "R" => current.right(line[1..].parse::<u32>().unwrap()),
            _ => panic!("Invalid line {line}"),
        };

        current = dial;
        zeroes += zs;
    }
    zeroes as usize
}

trait Dial: Sized {
    fn left(self, steps: u32) -> (Self, u32);
    fn right(self, steps: u32) -> (Self, u32);
}

impl Dial for u8 {
    fn left(self, steps: u32) -> (Self, u32) {
        let zeroes_passed = steps / 100;
        let steps = steps % 100;
        let me = self as i16;
        let me = me - steps as i16;

        if me < 0 {
            if self == 0 {
                ((100 + me) as u8, zeroes_passed)
            } else {
                ((100 + me) as u8, zeroes_passed + 1)
            }
        } else if me == 0 {
            (0, zeroes_passed + 1)
        } else {
            (me as u8, zeroes_passed)
        }
    }

    fn right(self, steps: u32) -> (Self, u32) {
        let zeroes_passed = steps / 100;
        let temp = self as u32 + (steps % 100);

        if temp > 99 {
            ((temp % 100) as u8, zeroes_passed + 1)
        } else {
            (temp as u8, zeroes_passed)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::prob1::{solve_part_1, solve_part_2, Dial};

    #[test]
    fn test_dial() {
        assert_eq!(5_u8.left(10), (95, 1));
        assert_eq!(5_u8.left(100), (5, 1));
        assert_eq!(99_u8.right(1), (0, 1));

        assert_eq!(5_u8.right(300), (5, 3));
        assert_eq!(5_u8.left(300), (5, 3));
    }

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 3);
        assert_eq!(solve_part_1("L50\nR10"), 1);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(INPUT), 6);
        assert_eq!(solve_part_2("L50\nR10"), 1);
    }

    const INPUT: &'static str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";
}
