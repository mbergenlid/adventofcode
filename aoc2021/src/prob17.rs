use crate::prob17::Result::{Hit, Missed, OverShot};
use std::cmp::max;
use std::collections::HashSet;
use itertools::all;

pub fn solve_part_1(_input: &str) -> usize {
    //target area: x=236..262, y=-78..-58
    let target = TargetArea {
        top_left: (236, -58),
        bottom_right: (262, -78),
    };

    let lower_x = (1889f64.sqrt() / 2f64 - 0.5).ceil() as i32;
    let upper_x = (3f64 * 233f64.sqrt() / 2f64 - 0.5).floor() as i32;
    solve_1(&target, lower_x, upper_x)
}

fn solve_1(target: &TargetArea, lower_x: i32, upper_x: i32) -> usize {
    let mut max_height = 0;
    for y_velocity in -100..1000 {
        for x_velocity in 0..1000 {
            let mut probe = Probe {
                position: (0, 0),
                velocity: (x_velocity, y_velocity),
            };
            let res = simulate(&mut probe, &target);
            match res {
                Hit(height) => {
                    println!("{:?}, {:?} {:?}", (x_velocity, y_velocity), probe, res);
                    if height > max_height {
                        max_height = height
                    }
                }
                OverShot => {
                    break;
                }
                Missed => {} // { if max_height > 0 { return max_height as usize }; }
            }
        }
    }
    max_height as usize
}

fn solve_2(target: &TargetArea) -> usize {
    let mut all_vectors = HashSet::new();
    for y_velocity in -100..100 { //Some arbitrary range that just seemed to work 🤷
        for x_velocity in 0..1000 {
            let mut probe = Probe {
                position: (0, 0),
                velocity: (x_velocity, y_velocity),
            };
            let res = simulate(&mut probe, &target);
            match res {
                Hit(height) => {
                    all_vectors.insert((x_velocity, y_velocity));
                }
                OverShot => {}
                Missed => {}
            }
        }
    }

    all_vectors.len()
}

#[derive(Debug)]
struct Probe {
    position: (i32, i32),
    velocity: (i32, i32),
}

impl Probe {
    fn update(&mut self) {
        self.position.0 += self.velocity.0;
        self.position.1 += self.velocity.1;
        if self.velocity.0 < 0 {
            self.velocity.0 += 1;
        } else if self.velocity.0 > 0 {
            self.velocity.0 -= 1;
        }
        self.velocity.1 -= 1;
    }

    fn is_inside_area(&self, target: &TargetArea) -> bool {
        self.position.0 >= target.top_left.0
            && self.position.0 <= target.bottom_right.0
            && self.position.1 <= target.top_left.1
            && self.position.1 >= target.bottom_right.1
    }

    fn overshot(&self, target: &TargetArea) -> bool {
        self.position.0 > target.bottom_right.0
    }

    fn missed(&self, target: &TargetArea) -> bool {
        self.position.1 < target.bottom_right.1
    }
}

struct TargetArea {
    top_left: (i32, i32),
    bottom_right: (i32, i32),
}

#[derive(Debug)]
enum Result {
    Hit(i32),
    OverShot,
    Missed,
}

fn simulate(probe: &mut Probe, target: &TargetArea) -> Result {
    let mut max_height = probe.position.0;
    loop {
        probe.update();
        if probe.position.1 > max_height {
            max_height = probe.position.1;
        }
        if probe.is_inside_area(target) {
            return Hit(max_height);
        } else if probe.overshot(target) {
            return OverShot;
        } else if probe.missed(target) {
            return Missed;
        }
    }
}

pub fn solve_part_2(_input: &str) -> usize {
    solve_2(&TargetArea {
        top_left: (236, -58),
        bottom_right: (262, -78),
    })
}

#[cfg(test)]
mod test {
    use crate::prob17::TargetArea;

    #[test]
    fn test_1() {
        let res = super::solve_1(
            &TargetArea {
                top_left: (20, -5),
                bottom_right: (30, -10),
            },
            6,
            7,
        );
        assert_eq!(res, 45);
    }

    #[test]
    fn test_2() {
        let res = super::solve_2(&TargetArea {
            top_left: (20, -5),
            bottom_right: (30, -10),
        });
        assert_eq!(res, 112);
    }

    const TESTCASE: &'static str = "target area: x=20..30, y=-10..-5";
}
