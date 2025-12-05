use std::ops::RangeInclusive;

pub fn solve_part_1(input: &str) -> usize {
    let (ranges_in, ids_in) = input.split_once("\n\n").unwrap();

    let ranges: Vec<_> = ranges_in
        .lines()
        .map(|line| {
            let (low, high) = line.split_once("-").unwrap();
            let low = low.parse::<u64>().unwrap();
            let high = high.parse::<u64>().unwrap();
            low..=high
        })
        .collect();

    ids_in
        .lines()
        .map(|id| id.parse::<u64>().unwrap())
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

pub fn solve_part_2(input: &str) -> usize {
    let (ranges_in, _) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<_> = ranges_in
        .lines()
        .map(|line| {
            let (low, high) = line.split_once("-").unwrap();
            let low = low.parse::<u64>().unwrap();
            let high = high.parse::<u64>().unwrap();
            low..=high
        })
        .collect();

    let mut reduced_ranges = Vec::with_capacity(ranges.len());

    while let Some(current) = ranges.pop() {
        // Try merge it with any in the list and replace those
        if let Some((idx, merged)) = merge_ranges(&ranges, &current) {
            ranges[idx] = merged;
        } else {
            reduced_ranges.push(current);
        }
    }

    reduced_ranges.into_iter().map(|r| r.count()).sum()
}

fn merge_ranges(
    ranges: &[RangeInclusive<u64>],
    current: &RangeInclusive<u64>,
) -> Option<(usize, RangeInclusive<u64>)> {
    for (idx, r) in ranges.iter().enumerate() {
        if r.contains(current.start()) && r.contains(current.end()) {
            // current is completely inside r
            return Some((idx, r.clone()));
        }
        if current.contains(r.start()) && current.contains(r.end()) {
            return Some((idx, current.clone()));
        }
        if r.contains(current.start()) {
            return Some((idx, *r.start()..=*current.end()));
        }
        if r.contains(current.end()) {
            return Some((idx, *current.start()..=*r.end()));
        }
        // No match
    }
    None
}

#[cfg(test)]
mod test {
    use crate::prob5::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), 14);
    }

    const INPUT: &'static str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
}
