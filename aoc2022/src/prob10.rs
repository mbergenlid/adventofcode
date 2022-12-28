pub fn solve_part_1(input: &str) -> usize {
    let mut signal_strength = 0;
    let mut cycle = 0;
    let mut x_register = 1;
    for instr in input.lines() {
        if instr.starts_with("noop") {
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                signal_strength += cycle * x_register;
            }
        } else if instr.starts_with("addx") {
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                signal_strength += cycle * x_register;
            }
            cycle += 1;
            if cycle == 20 || (cycle - 20) % 40 == 0 {
                signal_strength += cycle * x_register;
            }
            x_register += instr["addr ".len()..].parse::<i32>().expect("Not a number");
        }
    }

    signal_strength as usize
}

#[derive(Default)]
struct CRT {
    data: String,
    current_pixel: u16,
}

impl CRT {
    fn draw_pixel(&mut self, x_register: i32) {
        let current_pixel = self.current_pixel as i32;
        if current_pixel >= x_register - 1 && current_pixel <= x_register + 1 {
            self.data.push('#');
        } else {
            self.data.push('.');
        }
        self.current_pixel += 1;
        if self.current_pixel == 40 {
            self.data.push('\n');
            self.current_pixel = 0
        }
    }
}

pub fn solve_part_2(input: &str) -> usize {
    let mut crt = CRT::default();
    let mut x_register = 1;
    for instr in input.lines() {
        if instr.starts_with("noop") {
            crt.draw_pixel(x_register);
        } else if instr.starts_with("addx") {
            crt.draw_pixel(x_register);
            crt.draw_pixel(x_register);
            x_register += instr["addr ".len()..].parse::<i32>().expect("Not a number");
        }
    }

    println!("{}", crt.data);
    0
}

#[cfg(test)]
mod test {
    use crate::prob10::{solve_part_1, solve_part_2};

    #[test]
    fn test() {
        assert_eq!(solve_part_1(INPUT), 13140);
    }

    #[test]
    fn test2() {
        assert_eq!(solve_part_2(INPUT), 0);
    }

    const INPUT: &'static str = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
}
