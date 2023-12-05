use std::{collections::HashMap, str::FromStr};

pub fn solve_part_1(input: &str) -> usize {
    let engine = input.parse::<Engine>().unwrap();
    let mut sum = 0;
    for part in &engine.parts {
        if part
            .adjacent_points()
            .iter()
            .any(|p| engine.symbols.contains_key(p))
        {
            sum += dbg!(part.number);
        }
    }

    sum
}

pub fn solve_part_2(input: &str) -> usize {
    let engine = input.parse::<Engine>().unwrap();

    let mut result = 0;
    for (p, c) in &engine.symbols {
        if *c == '*' {
            let mut adjacent_parts = Vec::new();
            for part in &engine.parts {
                if part
                    .adjacent_points()
                    .iter()
                    .any(|part_point| p == part_point)
                {
                    adjacent_parts.push(part.number);
                }
            }
            if adjacent_parts.len() == 2 {
                result += adjacent_parts.into_iter().product::<usize>();
            }
        }
    }
    result
}

struct Engine {
    parts: Vec<PartNumber>,
    symbols: HashMap<Point, char>,
}

impl FromStr for Engine {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut symbols: HashMap<Point, char> = HashMap::new();
        let mut parts: Vec<PartNumber> = Vec::new();

        let mut row = 0;
        for line in s.lines() {
            let mut col = 0;
            let mut chars = line.chars().peekable();
            while let Some(c) = chars.next() {
                if c.is_digit(10) {
                    let point = Point { row, col };
                    let mut size = 1;
                    let mut number = c.to_digit(10).unwrap();
                    col += 1;
                    while let Some(c) = chars.peek() {
                        if c.is_digit(10) {
                            number = number * 10 + c.to_digit(10).unwrap();
                            col += 1;
                            size += 1;
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    parts.push(PartNumber {
                        number: number as usize,
                        point,
                        size,
                        adjacent_points: std::cell::RefCell::new(Vec::new()),
                    });
                } else {
                    if c != '.' {
                        symbols.insert(Point { row, col }, c);
                    }
                    col += 1;
                }
            }
            row += 1;
        }
        Ok(Engine { parts, symbols })
    }
}
#[derive(Debug)]
struct PartNumber {
    number: usize,
    point: Point,
    size: usize,
    adjacent_points: std::cell::RefCell<Vec<Point>>,
}

impl PartNumber {
    fn adjacent_points(&self) -> std::cell::Ref<Vec<Point>> {
        if self.adjacent_points.borrow().is_empty() {
            let p = &self.point;
            let mut points = Vec::new();
            points.push(Point {
                row: p.row,
                col: p.col - 1,
            });
            points.push(Point {
                row: p.row,
                col: p.col + self.size as i32,
            });

            let row = p.row - 1;
            for col in p.col - 1..=(p.col + self.size as i32) {
                points.push(Point { row, col });
            }
            let row = p.row + 1;
            for col in p.col - 1..=(p.col + self.size as i32) {
                points.push(Point { row, col });
            }
            self.adjacent_points.replace(points);
        }

        self.adjacent_points.borrow()
    }
}

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
struct Point {
    row: i32,
    col: i32,
}

#[cfg(test)]
mod test {

    #[test]
    fn solve_part_1() {
        assert_eq!(super::solve_part_1(TEST_INPUT), 4361);
    }

    #[test]
    fn solve_part_2() {
        assert_eq!(super::solve_part_2(TEST_INPUT), 467835);
    }

    const TEST_INPUT: &'static str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
}
