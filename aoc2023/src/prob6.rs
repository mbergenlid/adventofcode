pub fn solve_part_1(input: &str) -> usize {
    let mut lines = input.lines();
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|n| n.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut result = 1;
    for (t, d) in times.into_iter().zip(distances.into_iter()) {
        let (low, high) = solve(t, d);
        result *= high - low + 1;
    }
    result as usize
}

fn solve(time: i64, distance: i64) -> (i64, i64) {
    let time = time as f64;
    let distance = distance as f64;
    let lower = -time / 2.0 - ((time / 2.0) * (time / 2.0) - distance).sqrt();
    let upper = -time / 2.0 + ((time / 2.0) * (time / 2.0) - distance).sqrt();

    ((lower + 1.0).floor() as i64, (upper - 1.0).ceil() as i64)
}

pub fn solve_part_2(input: &str) -> usize {
    let mut lines = input.lines();
    let time = lines.next().unwrap()["Time:".len()..]
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();
    let distance = lines.next().unwrap()["Distance:".len()..]
        .replace(" ", "")
        .parse::<i64>()
        .unwrap();

    let (low, high) = solve(time, distance);
    (high - low + 1) as usize
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 288);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 71503);
    }

    const TEST_INPUT: &'static str = "Time:      7  15   30
Distance:  9  40  200";
}
