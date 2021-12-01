
pub fn solve_part_1(input: &str) -> usize {

    let mut depths = input.lines().filter(|l| !l.is_empty()).map(|l| l.parse::<u32>().expect(&format!("Parse error: {}", l)));

    let mut prev_depth = depths.next().unwrap();
    let mut count = 0;
    for depth in depths {
        if depth > prev_depth {
            count += 1;
        }
        prev_depth = depth;
    }

    count
}

pub fn solve_part_2(input: &str) -> usize {
    let depths: Vec<_> = input.lines().filter(|l| !l.is_empty()).map(|l| l.parse::<u32>().expect(&format!("Parse error: {}", l))).collect();

    let mut prev_sum: u32 = depths.iter().take(3).sum();
    let mut count = 0;
    for index in 1..(depths.len()-2) {
        let depth_sum = depths.iter().skip(index).take(3).sum();
        if depth_sum > prev_sum {
            count += 1;
        }
        prev_sum = depth_sum;
    }

    count
}

#[cfg(test)]
mod test {

    #[test]
    fn test() {
        let res = super::solve_part_1("199
200
208
210
200
207
240
269
260
263");

        assert_eq!(res, 7);
    }

    #[test]
    fn test2() {
        let res = super::solve_part_2("199
200
208
210
200
207
240
269
260
263");

        assert_eq!(res, 5);
    }
}
