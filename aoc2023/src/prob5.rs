use std::{ops::Range, str::FromStr};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let almanac = input.parse::<Almanac>().unwrap();

    almanac
        .initial_seeds
        .iter()
        .map(|&seed| {
            let mut result = seed;
            for map in &almanac.maps {
                result = map.lookup(result);
            }
            result
        })
        .min()
        .unwrap()
}

pub fn solve_part_2(input: &str) -> usize {
    let almanac = input.parse::<Almanac>().unwrap();

    let mut ranges = almanac
        .initial_seeds
        .chunks(2)
        .map(|chunk| {
            let (&start, &len) = chunk.into_iter().collect_tuple().unwrap();
            start..start + len
        })
        .collect::<Vec<_>>();

    for map in &almanac.maps {
        let mut ranges_at_this_level = Vec::new();
        for r in ranges {
            ranges_at_this_level.append(&mut map.lookup_range(r));
        }
        ranges = ranges_at_this_level;
    }
    ranges.into_iter().map(|r| r.start).min().unwrap()
}

struct Almanac {
    initial_seeds: Vec<usize>,
    maps: Vec<Map>,
}

impl FromStr for Almanac {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.split("\n\n");

        let initial_seeds = {
            let first_line = lines.next().unwrap();
            first_line["seeds: ".len()..]
                .split_whitespace()
                .map(|seed| seed.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        };

        let maps = lines
            .map(|line| line.parse::<Map>().unwrap())
            .collect::<Vec<_>>();
        Ok(Almanac {
            initial_seeds,
            maps,
        })
    }
}

struct Map {
    maps: Vec<(Range<usize>, Range<usize>)>,
}

impl Map {
    fn lookup(&self, source: usize) -> usize {
        for (src, dst) in &self.maps {
            if src.contains(&source) {
                return dst.start + (source - src.start);
            }
        }
        return source;
    }

    fn lookup_range(&self, source: Range<usize>) -> Vec<Range<usize>> {
        let mut ranges_left = vec![source];
        let mut result = Vec::new();
        for (src, dst) in &self.maps {
            let mut unresolved_ranges = Vec::new();
            for r in ranges_left {
                if r.start < src.start {
                    if r.end <= src.start {
                        //strictly before
                        unresolved_ranges.push(r);
                    } else if src.contains(&(r.end - 1)) {
                        //partly overlap left
                        unresolved_ranges.push(r.start..src.start);
                        result.push(dst.start..dst.start + (r.end - src.start));
                    } else {
                        //r fully contain src
                        unresolved_ranges.push(r.start..src.start);
                        result.push(dst.start..dst.end);
                        unresolved_ranges.push(src.end..r.end);
                    }
                } else if src.contains(&r.start) {
                    if src.contains(&(r.end - 1)) {
                        //src fully contain r
                        result.push(
                            dst.start + (r.start - src.start)..dst.start + (r.end - src.start),
                        )
                    } else {
                        //partially overlap right
                        result.push(dst.start + (r.start - src.start)..dst.end);
                        unresolved_ranges.push(src.end..r.end);
                    }
                } else {
                    unresolved_ranges.push(r)
                }
            }
            ranges_left = unresolved_ranges;
        }
        result.append(&mut ranges_left);
        result
    }
}

impl FromStr for Map {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let maps = s
            .lines()
            .skip(1)
            .map(|line| {
                let (dst_start, src_start, length) = line
                    .split_whitespace()
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect_tuple()
                    .unwrap();

                (
                    (src_start..(src_start + length)),
                    (dst_start..(dst_start + length)),
                )
            })
            .collect::<Vec<_>>();
        Ok(Map { maps })
    }
}

#[cfg(test)]
mod test {
    use crate::prob5::Map;

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 35);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 46);
    }

    #[test]
    fn test_lookup_range() {
        assert_eq!(
            Map {
                maps: vec![(10..15, 20..15),]
            }
            .lookup_range(0..10),
            vec![0..10]
        );

        assert_eq!(
            Map {
                maps: vec![(10..15, 20..15),]
            }
            .lookup_range(0..15),
            vec![20..25, 0..10,]
        );

        assert_eq!(
            Map {
                maps: vec![(10..15, 20..15),]
            }
            .lookup_range(10..15),
            vec![20..25]
        );
    }

    #[test]
    fn basic() {
        assert_eq!((10..15).contains(&15), false);
    }

    const TEST_INPUT: &'static str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
}
