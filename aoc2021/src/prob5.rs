use std::cmp::{max, min};
use std::collections::HashMap;
use std::io::repeat;
use std::iter;
use std::ptr::replace;

#[derive(Deserialize, Recap)]
#[recap(regex = r"(?P<start_x>\d+),(?P<start_y>\d+) -> (?P<end_x>\d+),(?P<end_y>\d+)")]
struct LineSegment {
    start_x: u32,
    start_y: u32,

    end_x: u32,
    end_y: u32,
}

pub fn solve_part_1(input: &str) -> usize {
    solve(input, true)
}

pub fn solve_part_2(input: &str) -> usize {
    solve(input, false)
}

fn solve(input: &str, only_horizontal_or_vertial: bool) -> usize {
    let lines: Vec<LineSegment> = input.lines().map(|line| line.parse().unwrap()).collect();

    let mut points = HashMap::new();
    for line in lines {
        if !only_horizontal_or_vertial || (line.start_y == line.end_y || line.start_x == line.end_x)
        {
            if let Some(range) = range(line) {
                for (x, y) in range {
                    if let Some(count) = points.get_mut(&(x, y)) {
                        *count += 1;
                    } else {
                        points.insert((x, y), 1u32);
                    }
                }
            }
        }
    }
    points.values().filter(|&&c| c > 1).count()
}

fn range(line: LineSegment) -> Option<Box<dyn Iterator<Item = (u32, u32)>>> {
    let x_range: Box<dyn Iterator<Item = u32>> = if line.start_x == line.end_x {
        Box::new(iter::repeat(line.start_x))
    } else if line.start_x < line.end_x {
        Box::new((line.start_x..=line.end_x))
    } else {
        Box::new((line.end_x..=line.start_x).rev())
    };
    let y_range: Box<dyn Iterator<Item = u32>> = if line.start_y == line.end_y {
        Box::new(iter::repeat(line.start_y))
    } else if line.start_y < line.end_y {
        Box::new((line.start_y..=line.end_y))
    } else {
        Box::new((line.end_y..=line.start_y).rev())
    };
    Some(Box::new(x_range.zip(y_range)))
}

#[cfg(test)]
mod test {

    #[test]
    fn test_1() {
        let res = super::solve_part_1(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        );
        assert_eq!(res, 5);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(
            "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2",
        );
        assert_eq!(res, 12);
    }
}
