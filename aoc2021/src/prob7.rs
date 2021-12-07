use std::convert::identity;

pub fn solve_part_1(input: &str) -> usize {
    solve(input, identity)
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, cost_of_moving)
}

fn solve<F>(input: &str, cost_func: F) -> usize
where
    F: Fn(u32) -> u32,
{
    let crab_positions: Vec<_> = input
        .split(",")
        .map(|n| n.trim().parse::<u32>().unwrap())
        .collect();

    let min_position = *crab_positions.iter().min().unwrap();
    let max_position = *crab_positions.iter().max().unwrap();

    let mut min_cost = u32::MAX;
    for pos in min_position..max_position {
        let cost = crab_positions
            .iter()
            .map(|&p| cost_func((pos as i64 - p as i64).abs() as u32))
            .sum();
        if cost < min_cost {
            min_cost = cost;
        }
    }

    min_cost as usize
}

fn cost_of_moving(steps: u32) -> u32 {
    steps * (steps + 1) / 2
}

#[cfg(test)]
mod test {
    #[test]
    fn test_1() {
        let res = super::solve_part_1("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(res, 37);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2("16,1,2,0,4,2,7,1,2,14");
        assert_eq!(res, 168);
    }
}
