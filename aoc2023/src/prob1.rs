pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = line
                .chars()
                .find(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .expect("No digit");
            let last = line
                .chars()
                .rev()
                .find(|c| c.is_digit(10))
                .map(|c| c.to_digit(10).unwrap())
                .expect("No digit");

            first * 10 + last
        })
        .sum::<u32>() as usize
}

const PATTERNS: [&'static str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

struct NumberPosition {
    number: usize,
    position: usize,
}

impl Eq for NumberPosition {}
impl Ord for NumberPosition {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.position.cmp(&other.position)
    }
}
impl PartialOrd for NumberPosition {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.position.partial_cmp(&other.position)
    }
}

impl PartialEq for NumberPosition {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let first = {
                let mut first_numer: Option<NumberPosition> = None;
                for (index, digit_text) in PATTERNS.iter().enumerate() {
                    if let Some(pos) = line.find(digit_text) {
                        let num_pos = NumberPosition {
                            number: index + 1,
                            position: pos,
                        };
                        first_numer = match first_numer {
                            Some(n) => Some(n.min(num_pos)),
                            None => Some(num_pos),
                        };
                    }
                }

                let first_digit =
                    line.chars()
                        .enumerate()
                        .find(|(_, c)| c.is_digit(10))
                        .map(|(i, c)| NumberPosition {
                            number: c.to_digit(10).unwrap() as usize,
                            position: i,
                        });

                match (first_numer, first_digit) {
                    (Some(ft), Some(fd)) => ft.min(fd),
                    (Some(ft), None) => ft,
                    (None, Some(fd)) => fd,
                    _ => panic!(),
                }
            };
            let last = {
                let mut number: Option<NumberPosition> = None;
                for (index, digit_text) in PATTERNS.iter().enumerate() {
                    if let Some(pos) = line.rfind(digit_text) {
                        let num_pos = NumberPosition {
                            number: index + 1,
                            position: pos,
                        };
                        number = match number {
                            Some(n) => Some(n.max(num_pos)),
                            None => Some(num_pos),
                        };
                    }
                }

                let digit = line
                    .chars()
                    .rev()
                    .enumerate()
                    .find(|(_, c)| c.is_digit(10))
                    .map(|(i, c)| NumberPosition {
                        number: c.to_digit(10).unwrap() as usize,
                        position: line.len() - (i + 1),
                    });

                match (number, digit) {
                    (Some(ft), Some(fd)) => ft.min(fd),
                    (Some(ft), None) => ft,
                    (None, Some(fd)) => fd,
                    _ => panic!(),
                }
            };

            first.number * 10 + last.number
        })
        .sum::<usize>()
}

#[cfg(test)]
mod test {
    use super::solve_part_2;

    #[test]
    fn part2() {
        assert_eq!(solve_part_2(TEST_INPUT), 281);
    }

    const TEST_INPUT: &'static str = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";
}
