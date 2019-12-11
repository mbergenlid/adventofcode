pub fn solve_part_1() {
    let result = solve_for_part_1(input().as_slice());
    println!("Part 1: {}", result);
}

pub fn solve_part_2() {
    solve_for_part_2(input().as_slice());
    //println!("Part 1: {}", result);
}
#[derive(Eq, PartialEq, Debug, Clone)]
struct Point {
    x: u32,
    y: u32,
}

#[derive(Debug, Eq)]
struct Fraction {
    nominator: i32,
    denominator: i32,
}

impl Fraction {
    fn new(nominator: i32, denominator: i32) -> Fraction {
        Fraction {
            nominator: nominator,
            denominator: denominator,
        }
    }

    fn eval(&self) -> f64 {
        (self.nominator as f64) / (self.denominator as f64)
    }
}

impl PartialEq for Fraction {
    fn eq(&self, rhs: &Self) -> bool {
        self.nominator * rhs.denominator == rhs.nominator * self.denominator
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Vector {
    direction: Fraction,
    length: u32,
    dir: bool,
    point: Point,
}
use std::cmp::Ordering;

impl Ord for Vector {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Vector {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let my_quadrant = if !self.dir {
            if self.direction.eval() <= 0.0 {
                0
            } else {
                1
            }
        } else {
            if self.direction.eval() <= 0.0 {
                2
            } else {
                3
            }
        };
        let other_quadrant = if !other.dir {
            if other.direction.eval() <= 0.0 {
                0
            } else {
                1
            }
        } else {
            if other.direction.eval() <= 0.0 {
                2
            } else {
                3
            }
        };

        if my_quadrant != other_quadrant {
            return my_quadrant.partial_cmp(&other_quadrant);
        } else {
            if self.direction == other.direction {
                return self.length.partial_cmp(&other.length);
            } else {
                match my_quadrant {
                    0 => self
                        .direction
                        .eval()
                        .abs()
                        .partial_cmp(&other.direction.eval().abs()),
                    1 => other
                        .direction
                        .eval()
                        .abs()
                        .partial_cmp(&self.direction.eval().abs()),
                    2 => self
                        .direction
                        .eval()
                        .abs()
                        .partial_cmp(&other.direction.eval().abs()),
                    3 => other
                        .direction
                        .eval()
                        .abs()
                        .partial_cmp(&self.direction.eval().abs()),
                    _ => panic!(""),
                }
            }
        }
    }
}

impl Vector {
    fn new(p1: &Point, p2: &Point) -> Vector {
        Vector {
            direction: Fraction::new(p2.x as i32 - p1.x as i32, p2.y as i32 - p1.y as i32),
            length: ((p2.x as i32 - p1.x as i32) * (p2.x as i32 - p1.x as i32)) as u32
                + ((p2.y as i32 - p1.y as i32) * (p2.y as i32 - p1.y as i32)) as u32,
            dir: if p2.x == p1.x {
                p2.y > p1.y
            } else {
                p2.x < p1.x
            },
            point: p2.clone(),
        }
    }
}

impl Point {
    fn new(x: u32, y: u32) -> Point {
        Point { x: x, y: y }
    }

    fn count_visible_points(&self, points: &[Point]) -> u32 {
        let vectors: Vec<Vector> = points
            .iter()
            .filter(|p| p != &self)
            .map(|p| Vector::new(self, p))
            .collect();

        let mut all_visible = Vec::new();
        for v in vectors.iter() {
            let all_with_same_direction = vectors
                .iter()
                .filter(|other| other.direction == v.direction && other.dir == v.dir)
                .collect::<Vec<_>>();

            let min = all_with_same_direction
                .iter()
                .map(|other| other.length)
                .min();

            if min.is_none() || min.unwrap() == v.length {
                all_visible.push(v);
            }
        }

        all_visible.len() as u32
    }
}

fn solve_for_part_1(points: &[Point]) -> u32 {
    let (_, res) = find_center_point(points);
    return res;
}

fn find_center_point(points: &[Point]) -> (Point, u32) {
    let mut max = 0;
    let mut max_point: Option<Point> = None;
    for p in points {
        let visible = p.count_visible_points(points);
        if visible > max {
            max_point = Some(p.clone());
            max = visible;
        }
    }
    (max_point.unwrap(), max)
}

fn solve_for_part_2(points: &[Point]) -> Point {
    let (center, _) = find_center_point(points);
    solve_for_part_2_new(&center, points)
}
fn solve_for_part_2_new(center: &Point, points: &[Point]) -> Point {
    let mut vectors: Vec<Vector> = points
        .iter()
        .filter(|&p| p != center)
        .map(|p| Vector::new(&center, p))
        .collect();
    vectors.sort();

    let mut bla: Vec<Vec<(Point, bool)>> = Vec::new();
    let first = vectors.first().unwrap();
    let mut prev_direction = &first.direction;
    let mut current_bucket = vec![(first.point.clone(), false)];
    for v in vectors.iter().skip(1) {
        if v.direction == *prev_direction {
            current_bucket.push((v.point.clone(), false));
        } else {
            bla.push(current_bucket);
            prev_direction = &v.direction;
            current_bucket = vec![(v.point.clone(), false)];
        }
    }

    //println!("{:?}", vectors);
    let mut vaporizing_order = Vec::new();
    let mut idx = 0;
    for i in 0..201 {
        let mut found_empty = false;
        while !found_empty {
            for (point, visited) in bla.get_mut(idx).unwrap().iter_mut() {
                if !*visited {
                    *visited = true;
                    found_empty = true;
                    println!("Vaporizing {:?} -> {}", point, idx);
                    vaporizing_order.push(point.clone());
                    break;
                }
            }

            idx += 1;

            if idx == bla.len() {
                idx = 0;
            }
        }
    }
    //println!("{:?}", center);
    //println!("{:?}", vectors[0].direction.eval());
    //println!("{:?}", vectors[1].direction.eval());
    //println!("{:?}", vectors[2].direction.eval());
    //println!("{:?}", vectors[199].point);
    println!("50th {:?}", vaporizing_order[49]);
    println!("100th {:?}", vaporizing_order[99]);
    println!("199th {:?}", vaporizing_order[198]);
    println!("200th {:?}", vaporizing_order[199]);
    println!("201th {:?}", vaporizing_order[200]);
    return vaporizing_order[199].clone();
}

#[cfg(test)]
mod test {

    use super::Point;

    #[test]
    fn basic_example() {
        assert_eq!(
            Point::new(1, 0).count_visible_points(test_input().as_slice()),
            7
        );
    }

    #[test]
    fn basic_example_2() {
        assert_eq!(super::solve_for_part_1(test_input().as_slice()), 8);
    }

    //#[test]
    //fn test_input_3() {
    //    assert_eq!(
    //        super::solve_for_part_1(
    //            super::parse(vec![
    //                "......#.#.",
    //                "#..#.#....",
    //                "..#######.",
    //                ".#.#.###..",
    //                ".#..#.....",
    //                "..#....#.#",
    //                "#..#....#.",
    //                ".##.#..###",
    //                "##...#..#.",
    //                ".#....####",
    //            ])
    //            .as_slice()
    //        )
    //        33
    //    );
    //}

    #[test]
    fn test_input_4() {
        assert_eq!(
            super::solve_for_part_1(
                super::parse(vec!(
                    "#.#...#.#.",
                    ".###....#.",
                    ".#....#...",
                    "##.#.#.#.#",
                    "....#.#.#.",
                    ".##..###.#",
                    "..#...##..",
                    "..##....##",
                    "......#...",
                    ".####.###.",
                ))
                .as_slice()
            ),
            35
        );
    }
    #[test]
    fn test_input_5() {
        assert_eq!(
            super::solve_for_part_1(
                super::parse(vec!(
                    ".#..#..###",
                    "####.###.#",
                    "....###.#.",
                    "..###.##.#",
                    "##.##.#.#.",
                    "....###..#",
                    "..#.#..#.#",
                    "#..#.#.###",
                    ".##...##.#",
                    ".....#.#..",
                ))
                .as_slice()
            ),
            41
        );
    }

    #[test]
    fn test_input_6() {
        assert_eq!(
            super::solve_for_part_1(
                super::parse(vec!(
                    ".#..##.###...#######",
                    "##.############..##.",
                    ".#.######.########.#",
                    ".###.#######.####.#.",
                    "#####.##.#.##.###.##",
                    "..#####..#.#########",
                    "####################",
                    "#.####....###.#.#.##",
                    "##.#################",
                    "#####.##.###..####..",
                    "..######..##.#######",
                    "####.##.####...##..#",
                    ".#####..#.######.###",
                    "##...#.##########...",
                    "#.##########.#######",
                    ".####.#.###.###.#.##",
                    "....##.##.###..#####",
                    ".#.#.###########.###",
                    "#.#.#.#####.####.###",
                    "###.##.####.##.#..##",
                ))
                .as_slice()
            ),
            210
        );
    }

    #[test]
    fn test_input_6_part_2() {
        assert_eq!(
            super::solve_for_part_2(
                super::parse(vec!(
                    ".#..##.###...#######",
                    "##.############..##.",
                    ".#.######.########.#",
                    ".###.#######.####.#.",
                    "#####.##.#.##.###.##",
                    "..#####..#.#########",
                    "####################",
                    "#.####....###.#.#.##",
                    "##.#################",
                    "#####.##.###..####..",
                    "..######..##.#######",
                    "####.##.####...##..#",
                    ".#####..#.######.###",
                    "##...#.##########...",
                    "#.##########.#######",
                    ".####.#.###.###.#.##",
                    "....##.##.###..#####",
                    ".#.#.###########.###",
                    "#.#.#.#####.####.###",
                    "###.##.####.##.#..##",
                ))
                .as_slice()
            ),
            Point::new(8, 2)
        );
    }

    //#[test]
    fn test_sample_part_2() {
        assert_eq!(
            super::solve_for_part_2_new(
                &Point::new(8, 3),
                super::parse(vec!(
                    ".#....#####...#..",
                    "##...##.#####..##",
                    "##...#...#.#####.",
                    "..#.....#...###..",
                    "..#.#.....#....##",
                ))
                .as_slice()
            ),
            Point::new(8, 2)
        );
    }

    #[test]
    fn partial_eq() {
        assert_eq!(
            super::Vector::new(&Point::new(20, 20), &Point::new(20, 18)).cmp(&super::Vector::new(
                &Point::new(20, 20),
                &Point::new(33, 19)
            )),
            super::Ordering::Less,
        );
        assert_eq!(
            super::Vector::new(&Point::new(20, 20), &Point::new(22, 20)).cmp(&super::Vector::new(
                &Point::new(20, 20),
                &Point::new(21, 19)
            )),
            super::Ordering::Greater,
        );
    }

    fn test_input() -> Vec<Point> {
        let lines = vec![".#..#", ".....", "#####", "....#", "...##"];

        let mut result: Vec<Point> = Vec::new();
        for (line_no, line) in lines.iter().enumerate() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    result.push(Point::new(i as u32, line_no as u32));
                }
            }
        }
        result
    }
}

fn parse(lines: Vec<&str>) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for (line_no, line) in lines.iter().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c == '#' {
                result.push(Point::new(i as u32, line_no as u32));
            }
        }
    }
    result
}

fn input() -> Vec<Point> {
    let lines = vec![
        "#.#....#.#......#.....#......####.",
        "#....#....##...#..#..##....#.##..#",
        "#.#..#....#..#....##...###......##",
        "...........##..##..##.####.#......",
        "...##..##....##.#.....#.##....#..#",
        "..##.....#..#.......#.#.........##",
        "...###..##.###.#..................",
        ".##...###.#.#.......#.#...##..#.#.",
        "...#...##....#....##.#.....#...#.#",
        "..##........#.#...#..#...##...##..",
        "..#.##.......#..#......#.....##..#",
        "....###..#..#...###...#.###...#.##",
        "..#........#....#.....##.....#.#.#",
        "...#....#.....#..#...###........#.",
        ".##...#........#.#...#...##.......",
        ".#....#.#.#.#.....#...........#...",
        ".......###.##...#..#.#....#..##..#",
        "#..#..###.#.......##....##.#..#...",
        "..##...#.#.#........##..#..#.#..#.",
        ".#.##..#.......#.#.#.........##.##",
        "...#.#.....#.#....###.#.........#.",
        ".#..#.##...#......#......#..##....",
        ".##....#.#......##...#....#.##..#.",
        "#..#..#..#...........#......##...#",
        "#....##...#......#.###.#..#.#...#.",
        "#......#.#.#.#....###..##.##...##.",
        "......#.......#.#.#.#...#...##....",
        "....##..#.....#.......#....#...#..",
        ".#........#....#...#.#..#....#....",
        ".#.##.##..##.#.#####..........##..",
        "..####...##.#.....##.............#",
        "....##......#.#..#....###....##...",
        "......#..#.#####.#................",
        ".#....#.#..#.###....##.......##.#.",
    ];

    parse(lines)
}

/*
    "#.#....#.#......#.....#......####.",
    "#....#....##...#..#..##....#.##..#",
    "#.#..#....#..#....##...###......##",
    "...........##..##..##.####.#......",
    "...##..##....##.#.....#.##....#..#",
    "..##.....#..#.......#.#.........##",
    "...###..##.###.#..................",
    ".##...###.#.#.......#.#...##..#.#.",
    "...#...##....#....##.#.....#...#.#",
    "..##........#.#...#..#...##...##..",
    "..#.##.......#..#......#.....##..#",
    "....###..#..#...###...#.###...#.##",
    "..#........#....#.....##.....#.#.#",
    "...#....#.....#..#...###........#.",
    ".##...#........#.#...#...##.......",
    ".#....#.#.#.#.....#...........#...",
    ".......###.##...#..#.#....#..##..#",
    "#..#..###.#.......##....##.#..#...",
    "..##...#.#.#........##..#..#.#..#.",
    ".#.##..#.......#.#.#.........##.##",
    "...#.#.....#.#....##X.#.........#.",
    ".#..#.##...#......#......#..##....",
    ".##....#.#......##...#....#.##..#.",
    "#..#..#..#...........#......##...#",
    "#....##...#......#.###.#..#.#...#.",
    "#......#.#.#.#....###..##.##...##.",
    "......#.......#.#.#.#...#...##....",
    "....##..#.....#.......#....#...#..",
    ".#........#....#...#.#..#....#....",
    ".#.##.##..##.#.#####..........##..",
    "..####...##.#.....##.............#",
    "....##......#.#..#....###....##...",
    "......#..#.#####.#................",
    ".#....#.#..#.###....##.......##.#.",
*/

/*
   Vaporizing Point { x: 20, y: 18 }
Vaporizing Point { x: 21, y: 1 }
Vaporizing Point { x: 21, y: 8 }
Vaporizing Point { x: 21, y: 9 }
Vaporizing Point { x: 22, y: 0 }
Vaporizing Point { x: 22, y: 1 }
Vaporizing Point { x: 22, y: 3 }
Vaporizing Point { x: 22, y: 4 }
Vaporizing Point { x: 22, y: 5 }
Vaporizing Point { x: 21, y: 13 }
Vaporizing Point { x: 22, y: 7 }
Vaporizing Point { x: 21, y: 14 }
Vaporizing Point { x: 23, y: 3 }
Vaporizing Point { x: 22, y: 11 }
Vaporizing Point { x: 24, y: 3 }
Vaporizing Point { x: 21, y: 16 }
Vaporizing Point { x: 25, y: 2 }
Vaporizing Point { x: 22, y: 13 }
Vaporizing Point { x: 25, y: 3 }
Vaporizing Point { x: 23, y: 10 }
Vaporizing Point { x: 25, y: 4 }
Vaporizing Point { x: 27, y: 1 }
Vaporizing Point { x: 23, y: 12 }
Vaporizing Point { x: 27, y: 3 }
Vaporizing Point { x: 23, y: 13 }
Vaporizing Point { x: 24, y: 11 }
Vaporizing Point { x: 29, y: 0 }
Vaporizing Point { x: 25, y: 9 }
Vaporizing Point { x: 26, y: 7 }
Vaporizing Point { x: 29, y: 1 }
Vaporizing Point { x: 21, y: 18 }
Vaporizing Point { x: 30, y: 1 }
Vaporizing Point { x: 27, y: 7 }
Vaporizing Point { x: 26, y: 9 }
Vaporizing Point { x: 31, y: 0 }
Vaporizing Point { x: 25, y: 11 }
Vaporizing Point { x: 27, y: 8 }
Vaporizing Point { x: 32, y: 0 }
Vaporizing Point { x: 30, y: 4 }
Vaporizing Point { x: 26, y: 11 }
Vaporizing Point { x: 33, y: 1 }
Vaporizing Point { x: 33, y: 2 }
Vaporizing Point { x: 30, y: 7 }
Vaporizing Point { x: 32, y: 5 }
Vaporizing Point { x: 33, y: 4 }
Vaporizing Point { x: 25, y: 14 }
Vaporizing Point { x: 33, y: 5 }
Vaporizing Point { x: 29, y: 10 }
Vaporizing Point { x: 30, y: 9 }
Vaporizing Point { x: 31, y: 8 }
Vaporizing Point { x: 32, y: 7 }
Vaporizing Point { x: 26, y: 14 }
Vaporizing Point { x: 33, y: 8 }
Vaporizing Point { x: 30, y: 11 }
Vaporizing Point { x: 29, y: 12 }
Vaporizing Point { x: 33, y: 10 }
Vaporizing Point { x: 24, y: 17 }
Vaporizing Point { x: 31, y: 12 }
Vaporizing Point { x: 33, y: 11 }
Vaporizing Point { x: 26, y: 16 }
Vaporizing Point { x: 33, y: 12 }
Vaporizing Point { x: 25, y: 17 }
Vaporizing Point { x: 32, y: 13 }
Vaporizing Point { x: 24, y: 18 }
Vaporizing Point { x: 29, y: 16 }
Vaporizing Point { x: 27, y: 17 }
Vaporizing Point { x: 30, y: 16 }
Vaporizing Point { x: 33, y: 16 }
Vaporizing Point { x: 30, y: 17 }
Vaporizing Point { x: 27, y: 18 }
Vaporizing Point { x: 29, y: 18 }
Vaporizing Point { x: 32, y: 18 }
Vaporizing Point { x: 29, y: 19 }
Vaporizing Point { x: 30, y: 19 }
Vaporizing Point { x: 32, y: 19 }
Vaporizing Point { x: 33, y: 19 }
Vaporizing Point { x: 21, y: 33 }
Vaporizing Point { x: 21, y: 28 }
Vaporizing Point { x: 22, y: 31 }
Vaporizing Point { x: 21, y: 24 }
Vaporizing Point { x: 23, y: 31 }
Vaporizing Point { x: 22, y: 27 }
Vaporizing Point { x: 21, y: 23 }
Vaporizing Point { x: 24, y: 31 }
Vaporizing Point { x: 21, y: 22 }
Vaporizing Point { x: 23, y: 25 }
Vaporizing Point { x: 24, y: 26 }
Vaporizing Point { x: 29, y: 33 }
Vaporizing Point { x: 23, y: 24 }
Vaporizing Point { x: 30, y: 33 }
Vaporizing Point { x: 24, y: 25 }
Vaporizing Point { x: 29, y: 31 }
Vaporizing Point { x: 30, y: 31 }
Vaporizing Point { x: 32, y: 33 }
Vaporizing Point { x: 27, y: 27 }
Vaporizing Point { x: 30, y: 29 }
Vaporizing Point { x: 29, y: 28 }
Vaporizing Point { x: 26, y: 25 }
Vaporizing Point { x: 31, y: 29 }
Vaporizing Point { x: 33, y: 30 }
Vaporizing Point { x: 28, y: 26 }
Vaporizing Point { x: 27, y: 25 }
Vaporizing Point { x: 26, y: 24 }
Vaporizing Point { x: 31, y: 27 }
Vaporizing Point { x: 28, y: 24 }
Vaporizing Point { x: 31, y: 25 }
Vaporizing Point { x: 32, y: 25 }
Vaporizing Point { x: 28, y: 23 }
Vaporizing Point { x: 26, y: 22 }
Vaporizing Point { x: 28, y: 22 }
Vaporizing Point { x: 33, y: 23 }
Vaporizing Point { x: 29, y: 22 }
Vaporizing Point { x: 25, y: 21 }
Vaporizing Point { x: 32, y: 22 }
Vaporizing Point { x: 28, y: 21 }
Vaporizing Point { x: 29, y: 21 }
Vaporizing Point { x: 22, y: 20 }
Vaporizing Point { x: 20, y: 24 }
Vaporizing Point { x: 19, y: 2 }
Vaporizing Point { x: 19, y: 3 }
Vaporizing Point { x: 19, y: 8 }
Vaporizing Point { x: 18, y: 1 }
Vaporizing Point { x: 18, y: 2 }
Vaporizing Point { x: 18, y: 8 }
Vaporizing Point { x: 18, y: 9 }
Vaporizing Point { x: 16, y: 0 }
Vaporizing Point { x: 18, y: 11 }
Vaporizing Point { x: 16, y: 3 }
Vaporizing Point { x: 19, y: 16 }
Vaporizing Point { x: 15, y: 1 }
Vaporizing Point { x: 15, y: 3 }
Vaporizing Point { x: 19, y: 17 }
Vaporizing Point { x: 15, y: 6 }
Vaporizing Point { x: 14, y: 4 }
Vaporizing Point { x: 13, y: 2 }
Vaporizing Point { x: 18, y: 15 }
Vaporizing Point { x: 17, y: 13 }
Vaporizing Point { x: 13, y: 4 }
Vaporizing Point { x: 16, y: 11 }
Vaporizing Point { x: 12, y: 3 }
Vaporizing Point { x: 11, y: 1 }
Vaporizing Point { x: 17, y: 14 }
Vaporizing Point { x: 10, y: 1 }
Vaporizing Point { x: 11, y: 3 }
Vaporizing Point { x: 12, y: 5 }
Vaporizing Point { x: 14, y: 9 }
Vaporizing Point { x: 9, y: 0 }
Vaporizing Point { x: 10, y: 2 }
Vaporizing Point { x: 12, y: 6 }
Vaporizing Point { x: 13, y: 8 }
Vaporizing Point { x: 12, y: 7 }
Vaporizing Point { x: 11, y: 6 }
Vaporizing Point { x: 7, y: 0 }
Vaporizing Point { x: 18, y: 17 }
Vaporizing Point { x: 13, y: 10 }
Vaporizing Point { x: 12, y: 9 }
Vaporizing Point { x: 9, y: 5 }
Vaporizing Point { x: 8, y: 4 }
Vaporizing Point { x: 10, y: 7 }
Vaporizing Point { x: 9, y: 6 }
Vaporizing Point { x: 5, y: 1 }
Vaporizing Point { x: 7, y: 4 }
Vaporizing Point { x: 15, y: 14 }
Vaporizing Point { x: 14, y: 13 }
Vaporizing Point { x: 12, y: 11 }
Vaporizing Point { x: 2, y: 0 }
Vaporizing Point { x: 8, y: 7 }
Vaporizing Point { x: 19, y: 19 }
Vaporizing Point { x: 0, y: 1 }
Vaporizing Point { x: 3, y: 4 }
Vaporizing Point { x: 5, y: 6 }
Vaporizing Point { x: 6, y: 7 }
Vaporizing Point { x: 7, y: 8 }
Vaporizing Point { x: 0, y: 2 }
Vaporizing Point { x: 11, y: 12 }
Vaporizing Point { x: 3, y: 5 }
Vaporizing Point { x: 4, y: 6 }
Vaporizing Point { x: 2, y: 5 }
Vaporizing Point { x: 3, y: 6 }
Vaporizing Point { x: 9, y: 11 }
Vaporizing Point { x: 2, y: 7 }
Vaporizing Point { x: 3, y: 8 }
Vaporizing Point { x: 1, y: 7 }
Vaporizing Point { x: 5, y: 10 }
Vaporizing Point { x: 3, y: 9 }
Vaporizing Point { x: 6, y: 11 }
Vaporizing Point { x: 12, y: 15 }
Vaporizing Point { x: 2, y: 9 }
Vaporizing Point { x: 5, y: 11 }
Vaporizing Point { x: 8, y: 13 }
Vaporizing Point { x: 4, y: 11 }
Vaporizing Point { x: 2, y: 10 }
Vaporizing Point { x: 12, y: 16 }
Vaporizing Point { x: 11, y: 16 }
Vaporizing Point { x: 6, y: 14 }
Vaporizing Point { x: 8, y: 15 }
Vaporizing Point { x: 3, y: 13 }
Vaporizing Point { x: 9, y: 16 }
Vaporizing Point { x: 6, y: 15 }
Vaporizing Point { x: 17, y: 19 }
 */
