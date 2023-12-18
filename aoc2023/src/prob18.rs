use std::ops::RangeInclusive;

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let mut borders = Vec::with_capacity(input.lines().count());
    let mut current = (0, 0);
    let mut directions = Vec::with_capacity(borders.capacity());
    for line in input.lines() {
        let (dir, steps) = line
            .split_whitespace()
            .take(2)
            .collect_tuple()
            .expect(&format!("No parse {}", line));
        let steps = steps.parse::<i64>().unwrap();
        match dir {
            "U" => {
                directions.push('U');
                borders.push(Line::Vertical(Vertical {
                    x: current.0,
                    y: current.1..=current.1 + steps - 1,
                }));
                current.1 += steps;
            }
            "R" => {
                directions.push('R');
                borders.push(Line::Horizontal(Horizontal {
                    y: current.1,
                    x: current.0..=current.0 + steps - 1,
                    should_count: false,
                }));
                current.0 += steps;
            }
            "D" => {
                directions.push('D');
                borders.push(Line::Vertical(Vertical {
                    x: current.0,
                    y: current.1 - steps + 1..=current.1,
                }));
                current.1 -= steps;
            }
            "L" => {
                directions.push('L');
                borders.push(Line::Horizontal(Horizontal {
                    y: current.1,
                    x: current.0 - steps + 1..=current.0,
                    should_count: false,
                }));
                current.0 -= steps;
            }
            _ => unreachable!(),
        }
    }

    solve(borders, directions)
}

pub fn solve_part_2(input: &str) -> usize {
    let mut borders = Vec::with_capacity(input.lines().count());
    let mut current = (0, 0);
    let mut directions = Vec::with_capacity(borders.capacity());
    for line in input.lines() {
        let hex = line
            .split_whitespace()
            .last()
            .expect(&format!("No parse {}", line));

        let (dir, steps) = (hex.chars().nth(7).unwrap(), &hex[2..7]);
        let steps = i64::from_str_radix(steps, 16).unwrap();
        match dir {
            '0' => {
                directions.push('R');
                borders.push(Line::Horizontal(Horizontal {
                    y: current.1,
                    x: current.0..=current.0 + steps - 1,
                    should_count: false,
                }));
                current.0 += steps;
            }
            '1' => {
                directions.push('D');
                borders.push(Line::Vertical(Vertical {
                    x: current.0,
                    y: current.1 - steps + 1..=current.1,
                }));
                current.1 -= steps;
            }
            '2' => {
                directions.push('L');
                borders.push(Line::Horizontal(Horizontal {
                    y: current.1,
                    x: current.0 - steps + 1..=current.0,
                    should_count: false,
                }));
                current.0 -= steps;
            }
            '3' => {
                directions.push('U');
                borders.push(Line::Vertical(Vertical {
                    x: current.0,
                    y: current.1..=current.1 + steps - 1,
                }));
                current.1 += steps;
            }
            _ => unreachable!(),
        }
    }
    solve(borders, directions)
}

fn solve(mut borders: Vec<Line>, directions: Vec<char>) -> usize {
    for (index, line) in borders.iter_mut().enumerate() {
        match line {
            Line::Horizontal(h) => {
                let prev_index = if index == 0 {
                    directions.len() - 1
                } else {
                    index - 1
                };
                let next_index = if index == directions.len() - 1 {
                    0
                } else {
                    index + 1
                };
                if directions[prev_index] != directions[next_index] {
                    h.should_count = true;
                }
            }
            Line::Vertical(_) => {}
        }
    }

    let min_y = borders
        .iter()
        .map(|line| match line {
            Line::Horizontal(h) => h.y,
            Line::Vertical(v) => *v.y.start(),
        })
        .min()
        .unwrap();
    let max_y = borders
        .iter()
        .map(|line| match line {
            Line::Horizontal(h) => h.y,
            Line::Vertical(v) => *v.y.end(),
        })
        .max()
        .unwrap();

    let sorted_by_x = borders
        .iter()
        .sorted_by_key(|line| match line {
            Line::Horizontal(h) => *h.x.start(),
            Line::Vertical(v) => v.x,
        })
        .collect::<Vec<_>>();

    let mut verticals = borders
        .iter()
        .filter_map(|line| match line {
            Line::Vertical(v) => Some((*v.y.start(), *v.y.end())),
            Line::Horizontal(h) => Some((h.y, h.y + 1)),
        })
        .flat_map(move |(start, end)| [start, end])
        .sorted()
        .unique()
        .collect::<Vec<_>>();
    verticals.push(max_y + 1);
    let ranges = verticals
        .iter()
        .zip(verticals.iter().skip(1))
        .map(|(s, e)| s..e)
        .collect::<Vec<_>>();

    let mut counter = 0;
    for range in ranges {
        let mut lines_crossed = 0;
        let mut prev_x = 0;
        for &line in sorted_by_x.iter() {
            match line {
                Line::Horizontal(h) => {
                    if h.y == *range.start {
                        assert!(
                            *range.end == *range.start + 1,
                            "Range {:?} {:?}",
                            *range.start - min_y,
                            *range.end - min_y
                        );

                        counter += (h.x.end() - h.x.start() + 1) as usize;
                        if lines_crossed % 2 == 1 {
                            counter += (h.x.start() - prev_x) as usize;
                        }
                        prev_x = h.x.end() + 1;
                        if h.should_count {
                            lines_crossed += 1;
                        }
                    }
                }
                Line::Vertical(v) => {
                    if v.y.contains(range.start) {
                        if lines_crossed % 2 == 1 {
                            counter += ((range.end - range.start) * (v.x - prev_x + 1)) as usize;
                        }
                        lines_crossed += 1;
                        prev_x = v.x;
                    }
                }
            }
        }
    }
    counter
}

#[derive(Debug)]
enum Line {
    Horizontal(Horizontal),
    Vertical(Vertical),
}

#[derive(Debug)]
struct Horizontal {
    y: i64,
    x: RangeInclusive<i64>,
    should_count: bool,
}

#[derive(Debug)]
struct Vertical {
    x: i64,
    y: RangeInclusive<i64>,
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 62);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 952408144115);
    }

    const TEST_INPUT: &'static str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
}
