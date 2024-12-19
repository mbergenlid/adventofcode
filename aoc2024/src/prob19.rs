
use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();

    let towels = towels.split(",").map(|t| t.trim()).collect_vec();

    let mut result = 0;
    for pattern in patterns.lines() {
        if count_combinations(&towels, pattern) > 0 {
            result += 1;
        }
    }
    result
}

pub fn solve_part_2(input: &str) -> usize {
    let (towels, patterns) = input.split_once("\n\n").unwrap();

    let towels = towels.split(",").map(|t| t.trim()).collect_vec();

    let mut result = 0;
    for pattern in patterns.lines() {
        result += count_combinations(&towels, pattern);
    }
    result
}
fn count_combinations(towels: &[&str], pattern: &str) -> usize {

    //table[i] represents if it is possible to find a solution starting from 
    //pattern[i..]
    let mut table = vec![0; pattern.len()];
    for i in (0..pattern.len()).rev() {

        let sub_pattern = &pattern[i..];
        for towel in towels {
            if sub_pattern.starts_with(towel) {
                if *towel == sub_pattern {
                    table[i] += 1;
                } else {
                    table[i] += table[i+towel.len()]
                }
            }
        }
    }
    table[0]
}


#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), 6);
    }

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT), 16);
    }
    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
}
