pub fn solve_part_1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let all_seqs = extract_history(line);
            let mut next_num_in_seq = 0;
            for seq in all_seqs.into_iter().rev() {
                next_num_in_seq = seq.last().unwrap() + next_num_in_seq;
            }

            next_num_in_seq
        })
        .sum()
}

fn extract_history(line: &str) -> Vec<Vec<i64>> {
    let mut current_seq = line
        .split_whitespace()
        .map(|c| c.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let mut all_seqs = Vec::new();

    while !current_seq.iter().all(|&i| i == 0) {
        let other = current_seq.iter().copied().skip(1);
        let next_seq = current_seq
            .iter()
            .copied()
            .zip(other)
            .map(|(i1, i2)| i2 - i1)
            .collect::<Vec<_>>();

        all_seqs.push(current_seq);
        current_seq = next_seq;
    }
    all_seqs
}

pub fn solve_part_2(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let all_seqs = extract_history(line);
            let mut next_num_in_seq = 0;
            for seq in all_seqs.into_iter().rev() {
                next_num_in_seq = seq.first().unwrap() - next_num_in_seq;
            }

            next_num_in_seq
        })
        .sum()
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 114);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 2);
    }

    const TEST_INPUT: &'static str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
}
