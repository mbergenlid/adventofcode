
pub fn solve_part_1(input: &str) -> usize {
    solve(input, 80)
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, 256)
}

fn solve(input: &str, days: u32) -> usize {
    let mut fishes = [0usize; 9];
    for fish in input.split(",").map(|s| s.trim().parse::<usize>().expect(&format!("Parse error {}", s))) {
        fishes[fish] += 1;
    }

    for day in 0..days {
        let zeroes = fishes[0];
        for index in 0..(fishes.len()-1) {
            fishes[index] = fishes[index+1];
        }
        fishes[8] = zeroes;
        fishes[6] += zeroes;
    }

    fishes.into_iter().sum()
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(
            "3,4,3,1,2"
        );

        assert_eq!(res, 5934)
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(
            "3,4,3,1,2"
        );

        assert_eq!(res, 26984457539)
    }
}
