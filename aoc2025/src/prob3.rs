pub fn solve_part_1(input: &str) -> usize {
    input
        .lines()
        .map(|line| part1::best_joltage(&parse(line)))
        .sum::<u32>() as usize
}

pub fn solve_part_2(input: &str) -> usize {
    input
        .lines()
        .map(|line| -> u64 { part2::best_joltage(&parse(line)) })
        .sum::<u64>() as usize
}

fn parse(input: &str) -> Vec<u8> {
    input
        .chars()
        .map(|c| c.to_digit(10).expect("Invalid digit") as u8)
        .collect::<Vec<_>>()
}

mod part1 {
    pub fn best_joltage(batteries: &[u8]) -> u32 {
        let mut best_joltage_from = vec![0u8; batteries.len()];
        *best_joltage_from.last_mut().unwrap() = *batteries.last().unwrap();

        for (index, &b) in batteries.iter().enumerate().rev().skip(1) {
            best_joltage_from[index] = b.max(best_joltage_from[index + 1]);
        }

        let mut best: u32 = 0;
        for (index, &b) in batteries[0..batteries.len() - 1].iter().enumerate() {
            best = best.max(b as u32 * 10 + best_joltage_from[index + 1] as u32);
        }
        best
    }
}

mod part2 {
    pub fn best_joltage(batteries: &[u8]) -> u64 {
        let mut best_joltage_from = vec![vec![0u64; batteries.len()]; 12];
        *best_joltage_from[0].last_mut().unwrap() = *batteries.last().unwrap() as u64;

        for (index, &b) in batteries.iter().enumerate().rev().skip(1) {
            best_joltage_from[0][index] = (b as u64).max(best_joltage_from[0][index + 1]);
        }

        for level in 1..12 {
            for (index, &b) in batteries.iter().enumerate().rev().skip(level) {
                let temp = best_joltage_from[level - 1][index + 1];
                let current_row = &mut best_joltage_from[level];
                current_row[index] =
                    current_row[index + 1].max(b as u64 * 10_u64.pow(level as u32) + temp);
            }
        }

        *best_joltage_from
            .last()
            .unwrap()
            .iter()
            .max()
            .expect("Should have at least one element")
    }
}

#[cfg(test)]
mod test {
    use crate::prob3::{part2, solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 357);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part2::best_joltage(&[2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 3, 4, 2, 7, 8]),
            434234234278
        );
        assert_eq!(solve_part_2(INPUT), 3121910778619);
    }

    const INPUT: &'static str = "987654321111111
811111111111119
234234234234278
818181911112111
";
}
