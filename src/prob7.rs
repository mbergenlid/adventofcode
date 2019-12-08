use crate::intcode::IntCode;
use std::sync::mpsc::channel;

pub fn solve_part_1() {
    let (max_thrust, _) = solve_for_part1(IntCode::new(input()));
    println!("Part 1: {}", max_thrust);
}

pub fn solve_part_2() {
    let code = IntCode::new(input());
    let mut max_thruster = 0;
    for a in 5..10 {
        for b in (5..10).filter(|&i| i != a) {
            for c in (5..10).filter(|&i| i != a && i != b) {
                for d in (5..10).filter(|&i| i != a && i != b && i != c) {
                    for e in (5..10).filter(|&i| i != a && i != b && i != c && i != d) {
                        let tv = thruster_value_part_2(&code, &[a, b, c, d, e]);
                        if tv > max_thruster {
                            max_thruster = tv;
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: {}", max_thruster);
}

fn solve_for_part1(code: IntCode) -> (i32, [i32; 5]) {
    let mut max_thruster = 0;
    let mut settings = [0, 0, 0, 0, 0];
    for a in 0..5 {
        for b in (0..5).filter(|&i| i != a) {
            for c in (0..5).filter(|&i| i != a && i != b) {
                for d in (0..5).filter(|&i| i != a && i != b && i != c) {
                    for e in (0..5).filter(|&i| i != a && i != b && i != c && i != d) {
                        let tv = thruster_value(code.clone(), &[a, b, c, d, e]);
                        if tv > max_thruster {
                            max_thruster = tv;
                            settings = [a, b, c, d, e];
                        }
                    }
                }
            }
        }
    }
    return (max_thruster, settings);
}
fn thruster_value(code: IntCode, phase_settings: &[i32]) -> i32 {
    let mut amp1 = code.clone();

    let out1 = amp1.run([phase_settings[0], 0].iter());
    let out2 = code.clone().run([phase_settings[1], out1[0]].iter());
    let out3 = code.clone().run([phase_settings[2], out2[0]].iter());
    let out4 = code.clone().run([phase_settings[3], out3[0]].iter());
    let out5 = code.clone().run([phase_settings[4], out4[0]].iter());
    out5[0]
}

fn thruster_value_part_2(code: &IntCode, phase_settings: &[i32]) -> i32 {
    let (sender1, receiver1) = channel::<i32>();
    let (sender2, receiver2) = channel();
    let (sender3, receiver3) = channel();
    let (sender4, receiver4) = channel();
    let (sender5, receiver5) = channel();

    let (ctrl_sender, ctrl_receiver) = channel();

    sender1.send(phase_settings[0]).unwrap();
    sender1.send(0).unwrap();
    sender2.send(phase_settings[1]).unwrap();
    sender3.send(phase_settings[2]).unwrap();
    sender4.send(phase_settings[3]).unwrap();
    sender5.send(phase_settings[4]).unwrap();

    code.clone().run_async(receiver1, sender2);
    code.clone().run_async(receiver2, sender3);
    code.clone().run_async(receiver3, sender4);
    code.clone().run_async(receiver4, sender5);
    code.clone().run_async(receiver5, ctrl_sender);

    let mut all_results = Vec::new();
    for value in ctrl_receiver.iter() {
        all_results.push(value);
        sender1.send(value).unwrap_or(());
    }

    *all_results.last().unwrap()
}

fn input() -> Vec<i32> {
    vec![
        3, 8, 1001, 8, 10, 8, 105, 1, 0, 0, 21, 42, 51, 60, 77, 94, 175, 256, 337, 418, 99999, 3,
        9, 1001, 9, 4, 9, 102, 5, 9, 9, 1001, 9, 3, 9, 102, 5, 9, 9, 4, 9, 99, 3, 9, 102, 2, 9, 9,
        4, 9, 99, 3, 9, 1001, 9, 3, 9, 4, 9, 99, 3, 9, 101, 4, 9, 9, 1002, 9, 4, 9, 101, 5, 9, 9,
        4, 9, 99, 3, 9, 1002, 9, 5, 9, 101, 3, 9, 9, 102, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 1, 9,
        4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9,
        4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9,
        3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101,
        2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4,
        9, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9,
        2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9,
        3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 1001, 9, 1, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3, 9, 1001, 9, 1,
        9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 3,
        9, 102, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 99, 3, 9, 1001, 9, 2, 9, 4, 9, 3, 9, 101,
        2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4, 9, 3, 9, 101, 2, 9, 9, 4,
        9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9, 1002, 9, 2, 9, 4, 9, 3, 9, 102, 2, 9, 9, 4, 9, 3, 9,
        1002, 9, 2, 9, 4, 9, 3, 9, 101, 1, 9, 9, 4, 9, 99,
    ]
}

#[cfg(test)]
mod test {

    use super::IntCode;
    #[test]
    fn test() {
        assert_eq!(
            super::thruster_value(
                IntCode::new(vec!(
                    3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
                )),
                vec!(4, 3, 2, 1, 0).as_slice()
            ),
            43210
        );
        assert_eq!(
            super::thruster_value(
                IntCode::new(vec!(
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                )),
                vec!(0, 1, 2, 3, 4).as_slice()
            ),
            54321
        );
    }
    #[test]
    fn another_test() {
        assert_eq!(
            super::solve_for_part1(IntCode::new(vec!(
                3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0
            )),),
            (43210, [4, 3, 2, 1, 0])
        );
    }

    #[test]
    fn max_thruster_test() {
        assert_eq!(
            super::solve_for_part1(IntCode::new(vec!(
                3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4,
                23, 99, 0, 0
            )),),
            (54321, [0, 1, 2, 3, 4])
        );
    }

    #[test]
    fn max_thruster2_test() {
        assert_eq!(
            super::solve_for_part1(IntCode::new(vec!(
                3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33,
                1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
            )),),
            (65210, [1, 0, 4, 3, 2])
        );
    }

    #[test]
    fn asd() {
        assert_eq!(
            super::thruster_value_part_2(
                &IntCode::new(vec![
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5,
                ]),
                &[9, 8, 7, 6, 5],
            ),
            139629729
        );
    }
}
