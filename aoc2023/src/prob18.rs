use std::{
    collections::{HashSet, VecDeque},
    ops::RangeInclusive,
};

use itertools::{all, Itertools};

pub fn solve_part_1(input: &str) -> usize {
    let mut borders = Vec::with_capacity(input.lines().count());
    let mut current = (0, 0);
    for line in input.lines() {
        let (dir, steps) = line
            .split_whitespace()
            .take(2)
            .collect_tuple()
            .expect(&format!("No parse {}", line));
        let steps = steps.parse::<i64>().unwrap();
        match dir {
            "U" => {
                borders.push(Line::Vertical(Vertical {
                    x: current.0,
                    y: current.1..=current.1 + steps - 1,
                }));
                current.1 += steps;
            }
            "R" => {
                borders.push(Line::Horizontal(Horizontal {
                    y: current.1,
                    x: current.0..=current.0 + steps - 1,
                }));
                current.0 += steps;
            }
            "D" => {
                borders.push(Line::Vertical(Vertical {
                    x: current.0,
                    y: current.1 - steps + 1..=current.1,
                }));
                current.1 -= steps;
            }
            "L" => {
                borders.push(Line::Horizontal(Horizontal {
                    y: current.1,
                    x: current.0 - steps + 1..=current.0,
                }));
                current.0 -= steps;
            }
            _ => unreachable!(),
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
    let min_x = borders
        .iter()
        .map(|line| match line {
            Line::Horizontal(h) => *h.x.start(),
            Line::Vertical(v) => v.x,
        })
        .min()
        .unwrap();
    let max_x = borders
        .iter()
        .map(|line| match line {
            Line::Horizontal(h) => *h.x.end(),
            Line::Vertical(v) => v.x,
        })
        .max()
        .unwrap();

    let mut grid =
        vec![vec!['.'; (max_x - min_x).abs() as usize + 1]; (max_y - min_y).abs() as usize + 1];

    let mut counter = 0;
    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if !borders.iter().any(|line| line.contains((x, y))) {
                //let hit_line_up = borders.iter().any(|line| line.strictly_above(x, y));
                //let hit_line_right = borders.iter().any(|line| line.strictly_right(x, y));
                //let hit_line_below = borders.iter().any(|line| line.strictly_below(x, y));
                //let hit_line_left = borders.iter().any(|line| line.strictly_left(x, y));

                let mut lines_crossed = 0;
                for b_index in 0..borders.len() {
                    match &borders[b_index] {
                        Line::Horizontal(h) => {
                            if x < *h.x.start() && h.y == y {
                                let prev_index = if b_index == 0 {
                                    borders.len() - 1
                                } else {
                                    b_index - 1
                                };
                                let next_index = if b_index == borders.len() - 1 {
                                    0
                                } else {
                                    b_index + 1
                                };
                                if let (Line::Vertical(v_prev), Line::Vertical(v_next)) =
                                    (&borders[prev_index], &borders[next_index])
                                {
                                    if *v_prev.y.start() < h.y && *v_next.y.start() < h.y {
                                        lines_crossed += 1;
                                    } else if *v_prev.y.end() > h.y && *v_next.y.end() > h.y {
                                        lines_crossed += 1;
                                    }
                                }
                            }
                        }
                        Line::Vertical(v) => {
                            if x < v.x && v.y.contains(&y) {
                                lines_crossed += 1
                            }
                        }
                    }
                }
                //if hit_line_up && hit_line_below && hit_line_right && hit_line_left {
                if lines_crossed % 2 == 1 {
                    grid[(y - min_y) as usize][(x - min_x) as usize] = '+';
                    counter += 1;
                } else {
                    grid[(y - min_y) as usize][(x - min_x) as usize] = '.';
                }
            } else {
                grid[(y - min_y) as usize][(x - min_x) as usize] = '#';
            }
        }
    }
    for row in grid.into_iter().rev() {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
    counter + borders.iter().map(|line| line.count()).sum::<usize>()
}
//if visited.contains(&(x, y)) {
//    continue;
//}
//let mut current_set = HashSet::new();
//current_set.insert((x, y));

//let mut queue = VecDeque::new();
//queue.push_back((x, y));

//while let Some((x, y)) = queue.pop_front() {
//    for (nx, ny) in [(x + 1, y), (x, y - 1), (x - 1, y), (x, y + 1)] {
//        if !(nx >= min_x && nx <= max_x && ny >= min_y && ny <= max_y) {
//            if !borders.iter().any(|line| line.contains((x, y))) {
//                visited.insert((nx, ny));
//                queue.push_back((nx, ny));
//            }
//        } else {
//            current_set.clear();
//            queue.clear();
//            break;
//        }
//    }
//}
//if !current_set.is_empty() {
//    counter += current_set.len();
//}

pub fn solve_part_2(_input: &str) -> usize {
    todo!()
}

#[derive(Debug)]
enum Line {
    Horizontal(Horizontal),
    Vertical(Vertical),
}

impl Line {
    fn contains(&self, point: (i64, i64)) -> bool {
        match self {
            Line::Horizontal(h) => h.y == point.1 && h.x.contains(&point.0),
            Line::Vertical(v) => v.x == point.0 && v.y.contains(&point.1),
        }
    }

    fn intersects_to_right(&self, (x, y): (i64, i64)) -> bool {
        match self {
            Line::Horizontal(_) => false,
            Line::Vertical(v) => x < v.x && v.y.contains(&y),
        }
    }

    fn count(&self) -> usize {
        match self {
            Line::Horizontal(h) => (h.x.end() - h.x.start()).abs() as usize + 1,
            Line::Vertical(v) => (v.y.end() - v.y.start()).abs() as usize + 1,
        }
    }

    fn strictly_above(&self, x: i64, y: i64) -> bool {
        match self {
            Line::Horizontal(h) => y < h.y && h.x.contains(&x),
            Line::Vertical(v) => x == v.x && y < *v.y.start(),
        }
    }
    fn strictly_right(&self, x: i64, y: i64) -> bool {
        match self {
            Line::Horizontal(h) => x < *h.x.start() && y == h.y,
            Line::Vertical(v) => x < v.x && v.y.contains(&y),
        }
    }
    fn strictly_below(&self, x: i64, y: i64) -> bool {
        match self {
            Line::Horizontal(h) => y > h.y && h.x.contains(&x),
            Line::Vertical(v) => x == v.x && y > *v.y.end(),
        }
    }
    fn strictly_left(&self, x: i64, y: i64) -> bool {
        match self {
            Line::Horizontal(h) => x > *h.x.end() && y == h.y,
            Line::Vertical(v) => x > v.x && v.y.contains(&y),
        }
    }
}

#[derive(Debug)]
struct Horizontal {
    y: i64,
    x: RangeInclusive<i64>,
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
        assert_eq!(super::solve_part_2(TEST_INPUT), 0);
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
