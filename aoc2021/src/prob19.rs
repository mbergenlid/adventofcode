use std::{
    collections::{HashSet, VecDeque},
    hash::Hash,
    ops::{Add, Sub},
    str::FromStr,
};

use itertools::Itertools;

pub fn solve_part_1(input: &str) -> usize {
    let scanners = orientate_scanners(input);
    let all_beacons = scanners
        .into_iter()
        .flat_map(|s| s.beacons.into_iter())
        .collect::<HashSet<_>>();

    all_beacons.len()
}

pub fn solve_part_2(input: &str) -> usize {
    let scanners = orientate_scanners(input);

    let mut max_distance = 0;
    for s1 in &scanners {
        for s2 in &scanners {
            let distance = s1
                .position
                .unwrap()
                .manhattan_distance_to(&s2.position.unwrap());
            max_distance = distance.max(max_distance);
        }
    }
    max_distance
}

fn orientate_scanners(input: &str) -> Vec<Scanner> {
    let mut scanners = input
        .split("\n\n")
        .map(|s| s.parse::<Scanner>().unwrap())
        .collect::<Vec<_>>();

    let mut processed_scanners = Vec::new();
    let mut first_scanner = scanners.remove(0);
    first_scanner.position = Some(Point([0, 0, 0]));

    let mut unprocessed_scanners = scanners;
    let mut scanners_to_process = VecDeque::new();
    scanners_to_process.push_back(first_scanner);
    while let Some(next_scanner) = scanners_to_process.pop_front() {
        let (new_processed, still_unprocessed): (Vec<_>, Vec<_>) = unprocessed_scanners
            .into_iter()
            .partition_map(|s| match s.intersects(&next_scanner) {
                Some(new_scanner) => itertools::Either::Left(new_scanner),
                None => itertools::Either::Right(s),
            });
        unprocessed_scanners = still_unprocessed;
        scanners_to_process.extend(new_processed);
        processed_scanners.push(next_scanner);
    }

    return processed_scanners;
}

struct Scanner {
    name: String,
    position: Option<Point>,
    beacons: Vec<Point>,
}

impl Scanner {
    fn intersects(&self, other: &Scanner) -> Option<Scanner> {
        for orientation in ALL_ORIENTATIONS {
            for p1 in other.beacons.iter().copied() {
                for p2 in self.beacons.iter().copied() {
                    //assume these two are the same
                    //How many points would be matching then
                    let diff = p1 - p2.rotate(&orientation);

                    let other_beacons = other.beacons.iter().copied().collect::<HashSet<_>>();
                    let self_beacons = self
                        .beacons
                        .iter()
                        .copied()
                        .map(|p| p.rotate(&orientation) + diff)
                        .collect::<HashSet<_>>();

                    let intersect = self_beacons
                        .intersection(&other_beacons)
                        .copied()
                        .collect::<Vec<_>>();

                    if intersect.len() >= 12 {
                        return Some(Scanner {
                            name: self.name.clone(),
                            beacons: self_beacons.into_iter().collect(),
                            position: Some(diff),
                        });
                    }
                }
            }
        }
        None
    }
}

impl FromStr for Scanner {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let name = lines.next().ok_or(format!("No scanner name in {}", s))?;

        let beacons = lines
            .map(|line| {
                let (x, y, z) = line
                    .split(",")
                    .map(|x| {
                        x.trim()
                            .parse::<i32>()
                            .expect(&format!("Not a number {}", x))
                    })
                    .collect_tuple()
                    .unwrap();

                Point([x, y, z])
            })
            .collect::<Vec<Point>>();
        Ok(Scanner {
            name: name.to_string(),
            position: None,
            beacons,
        })
    }
}

const X: i32 = 1;
const Y: i32 = 2;
const Z: i32 = 3;

struct Orientation([i32; 3]);

const ALL_ORIENTATIONS: [Orientation; 24] = [
    //
    Orientation([X, Y, Z]),
    Orientation([X, -Z, Y]),
    Orientation([X, -Y, -Z]),
    Orientation([X, Z, -Y]),
    //
    Orientation([-X, Y, -Z]),
    Orientation([-X, -Z, -Y]),
    Orientation([-X, -Y, Z]),
    Orientation([-X, Z, Y]),
    //
    Orientation([Y, -Z, -X]),
    Orientation([Y, -X, Z]),
    Orientation([Y, Z, X]),
    Orientation([Y, X, -Z]),
    //
    Orientation([-Y, Z, -X]),
    Orientation([-Y, X, Z]),
    Orientation([-Y, -Z, X]),
    Orientation([-Y, -X, -Z]),
    //
    Orientation([Z, Y, -X]),
    Orientation([Z, X, Y]),
    Orientation([Z, -Y, X]),
    Orientation([Z, -X, -Y]),
    //
    Orientation([-Z, Y, X]),
    Orientation([-Z, X, -Y]),
    Orientation([-Z, -Y, -X]),
    Orientation([-Z, -X, Y]),
    //
];

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Point([i32; 3]);

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Point([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}
impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Point {
    fn rotate(&self, orientation: &Orientation) -> Self {
        let mut res = [0; 3];

        let first = orientation.0[0];
        let first_index = first.abs() - 1;

        res[0] = self.0[first_index as usize] * first.sign();
        res[1] = self.0[orientation.0[1].abs() as usize - 1] * orientation.0[1].sign();
        res[2] = self.0[orientation.0[2].abs() as usize - 1] * orientation.0[2].sign();

        return Self(res);
    }

    fn manhattan_distance_to(&self, other: &Point) -> usize {
        ((self.0[0] - other.0[0]).abs()
            + (self.0[1] - other.0[1]).abs()
            + (self.0[2] - other.0[2]).abs()) as usize
    }
}

trait Signum {
    fn sign(self) -> Self;
}

impl Signum for i32 {
    #[inline]
    fn sign(self) -> Self {
        if self > 0 {
            return 1;
        } else if self < 0 {
            return -1;
        } else {
            return 0;
        }
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_orientation() {
        let vec = Point([8, 0, 7]);

        let rotations: Vec<_> = ALL_ORIENTATIONS.iter().map(|o| vec.rotate(o)).collect();

        assert!(rotations.contains(&Point([8, 0, 7])));
        assert!(rotations.contains(&Point([-8, -7, 0])));
        assert!(rotations.contains(&Point([-7, 0, 8])));
        assert!(rotations.contains(&Point([7, 0, 8])));
        assert!(rotations.contains(&Point([0, 7, -8])));
    }

    #[test]
    fn test_intersects() {
        let mut scanner = TESTCASE
            .split("\n\n")
            .map(|s| s.parse::<Scanner>().unwrap())
            .collect::<Vec<_>>();

        let scanner1 = scanner.remove(1);
        let res = scanner1.intersects(&scanner[0]);
        assert!(res.is_some());
        let intersection = res.unwrap().beacons.iter().copied().collect::<HashSet<_>>();
        let expected = vec![
            Point([-618, -824, -621]),
            Point([-537, -823, -458]),
            Point([-447, -329, 318]),
            Point([404, -588, -901]),
            Point([544, -627, -890]),
            Point([528, -643, 409]),
            Point([-661, -816, -575]),
            Point([390, -675, -793]),
            Point([423, -701, 434]),
            Point([-345, -311, 381]),
            Point([459, -707, 401]),
            Point([-485, -357, 347]),
        ]
        .into_iter()
        .collect::<HashSet<_>>();

        for p in expected {
            assert!(
                intersection.contains(&p),
                "{:?} should contain {:?}",
                intersection,
                p
            );
        }
    }

    #[test]
    fn test_1() {
        let res = super::solve_part_1(TESTCASE);
        assert_eq!(res, 79);
    }

    #[test]
    fn test_2() {
        let res = super::solve_part_2(TESTCASE);
        assert_eq!(res, 3621);
    }

    const TESTCASE: &'static str = "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14";
}
