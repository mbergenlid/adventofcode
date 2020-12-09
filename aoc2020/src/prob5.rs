use std::collections::BTreeSet;

pub fn solve_part_1(_: &str) -> u32 {
    include_str!("../inputs/prob5")
        .lines()
        .map(|s| seat_id_of(s))
        .max()
        .unwrap()
}

pub fn solve_part_2(_: &str) -> u32 {
    let seats_taken: BTreeSet<_> = include_str!("../inputs/prob5")
        .lines()
        .map(|s| seat_id_of(s)).collect();
    for seat in *seats_taken.iter().min().unwrap()..*seats_taken.iter().max().unwrap() {
        if !seats_taken.contains(&seat) && seats_taken.contains(&(seat-1)) && seats_taken.contains(&(seat+1)) {
            return seat;
        }
    }
    panic!("Not found")
}

fn seat_id_of(s: &str) -> u32 {
    let mut result = 0;
    for c in s.chars() {
        match c {
            'B' | 'R' => result = (result << 1) | 1,
            'F' | 'L' => result = (result << 1) | 0,
            _ => panic!("Not a valid seat")
        }
    }
    result
}

#[cfg(test)]
mod test {
    use crate::prob5::seat_id_of;

    #[test]
    fn test_part_1() {
        assert_eq!(seat_id_of("BFFFBBFRRR"), 567);
    }
}
