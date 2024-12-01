use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| {
            line.splitn(2, r"   ")
                .collect_tuple::<(_, _)>()
                .unwrap_or_else(|| panic!("invalid input {}", line))
        })
        .map(|(x, y)| {
            (
                x.parse::<u64>().expect("Not a number"),
                y.parse::<u64>().expect("Not a number"),
            )
        })
        .collect::<Vec<_>>();

    let left = lines.iter().map(|(l, _)| l).sorted();
    let right = lines.iter().map(|(_, r)| r).sorted();

    left.zip(right).map(|(l, r)| l.abs_diff(*r)).sum::<u64>() as usize
}

pub fn solve_part_2(input: &str) -> usize {
    let lines = input
        .lines()
        .map(|line| {
            line.splitn(2, r"   ")
                .collect_tuple::<(_, _)>()
                .unwrap_or_else(|| panic!("invalid input {}", line))
        })
        .map(|(x, y)| {
            (
                x.parse::<u64>().expect("Not a number"),
                y.parse::<u64>().expect("Not a number"),
            )
        })
        .collect::<Vec<_>>();

    let left = lines.iter().map(|(l, _)| l);
    let right = lines.iter().map(|(_, r)| *r).counts();

    let mut sum = 0;
    for n in left {
        let num_times_in_right = right.get(n).unwrap_or(&0);
        sum += (*n as usize) * num_times_in_right;
    }
    sum
}

#[cfg(test)]
mod test {
    use crate::prob1::{solve_part_1, solve_part_2};

    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1() {
        assert_eq!(solve_part_1(INPUT), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part_2(INPUT), 31);
    }
}
