use itertools::Itertools;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{Range, RangeInclusive};

#[derive(Deserialize, Recap, Debug)]
#[recap(
    regex = r"Sensor at x=(?P<x>-?\d+), y=(?P<y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"
)]
struct Sensor {
    x: i64,
    y: i64,
    beacon_x: i64,
    beacon_y: i64,
}

impl Sensor {
    fn positions_covered_on_line(&self, line: i64) -> RangeInclusive<i64> {
        let distance_to_beacon = (self.x - self.beacon_x).abs() + (self.y - self.beacon_y).abs();
        let y_distance_to_line = (self.y - line).abs();

        let start_x = self.x - (distance_to_beacon - y_distance_to_line);
        let end_x = self.x + (distance_to_beacon - y_distance_to_line);
        start_x..=end_x
    }

    fn distance_to_beacon(&self) -> i64 {
        (self.x - self.beacon_x).abs() + (self.y - self.beacon_y).abs()
    }

    // fn ranges_covered(&self) -> Vec<RangeInclusive<i64>> {
    //     let distance_to_beacon = (self.x-self.beacon_x).abs() + (self.y-self.beacon_y).abs();
    //     let y_distance_to_line = (self.y - line).abs();
    //
    //     let start_x = self.x - (distance_to_beacon - y_distance_to_line);
    //     let end_x = self.x + (distance_to_beacon - y_distance_to_line);
    //     //start_x..=end_x
    //     todo!()
    // }
}

pub fn solve_part_1(input: &str) -> usize {
    let sensonrs = input
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();

    let mut set = HashSet::new();
    let mut all_beacons = HashSet::new();
    for sensor in sensonrs {
        if sensor.beacon_y == 2000000 {
            all_beacons.insert(sensor.beacon_x);
        }
        let range = sensor.positions_covered_on_line(2000000);
        for i in range {
            set.insert(i);
        }
    }

    // println!("{:?}", set.iter().sorted());
    set.retain(|i| !all_beacons.contains(i));
    set.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let sensons = input
        .lines()
        .map(|line| line.parse::<Sensor>().unwrap())
        .collect::<Vec<_>>();

    let mut covered_ranges = vec![Vec::new(); 4000000 + 1];
    for sensor in sensons {
        let d = sensor.distance_to_beacon();

        for y in max(sensor.y - d, 0)..=min(sensor.y + d, 4000000) {
            // for range in covered_ranges.get_mut(y as usize).unwrap().iter_mut() {
            //
            // }
            covered_ranges[y as usize].push(sensor.positions_covered_on_line(y));
        }
    }

    find_tuning_frequency(covered_ranges)
}

fn find_tuning_frequency(mut covered_ranges: Vec<Vec<RangeInclusive<i64>>>) -> usize {
    for (i, ranges) in covered_ranges.iter_mut().enumerate() {
        while ranges.len() > 1 {
            let range = ranges.pop().unwrap();
            let merged = try_merge(&range, ranges);
            if !merged {
                println!("Found it on {} {:?} {:?}", i, range, ranges);
                println!("Result = ({:?} .. {:?}", ranges.iter().map(|r| *r.start()).min(), ranges.iter().map(|r| *r.end()).max());

                let min_x = ranges.iter().map(|r| *r.start()).min().unwrap();
                let max_x = ranges.iter().map(|r| *r.end()).max().unwrap();

                if *range.start() == max_x + 2 {
                    return (max_x as usize + 1) * 4000000 + i;
                } else if *range.end() == min_x - 2 {
                    return (min_x as usize - 1) * 4000000 + i;
                } else {
                    panic!("ahaaaha");
                }

            }
        }
    }
    todo!()
}

fn try_merge(range: &RangeInclusive<i64>, ranges: &mut Vec<RangeInclusive<i64>>) -> bool {
    for other_range in ranges.iter_mut() {
        if range.start() <= other_range.start() && range.end() >= other_range.start()
            ||
            range.start() > other_range.start() && *range.start() <= (*other_range.end() + 1)

        {
            //merge them
            *other_range =
                min(*range.start(), *other_range.start())..=max(*range.end(), *other_range.end());
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod test {
    use crate::prob15::{solve_part_1, solve_part_2};

    #[test]
    fn test_1() {
        assert_eq!(solve_part_1(INPUT), 26);
    }

    #[test]
    fn test_2() {
        assert_eq!(solve_part_2(INPUT), 56000011);
    }

    const INPUT: &'static str = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";
}
