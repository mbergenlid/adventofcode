use itertools::Itertools;

pub fn solve_part_1(input: &str) -> String {
    let mut lines = input.lines();

    let register_a = lines.next().unwrap()["Register A: ".len()..]
        .parse::<i64>()
        .unwrap();
    let register_b = lines.next().unwrap()["Register B: ".len()..]
        .parse::<i64>()
        .unwrap();
    let register_c = lines.next().unwrap()["Register C: ".len()..]
        .parse::<i64>()
        .unwrap();

    let program = lines.nth(1).unwrap()["Program: ".len()..]
        .split(",")
        .map(|c| c.parse::<u8>().unwrap())
        .collect_vec();

    let registers = Registers {
        a: register_a,
        b: register_b,
        c: register_c,
    };
    run_program(&program, registers).into_iter().join(",")
}

pub fn solve_part_2(input: &str) -> usize {
    let mut lines = input.lines();

    let _ = lines.next().unwrap()["Register A: ".len()..]
        .parse::<i64>()
        .unwrap();
    let register_b = lines.next().unwrap()["Register B: ".len()..]
        .parse::<i64>()
        .unwrap();
    let register_c = lines.next().unwrap()["Register C: ".len()..]
        .parse::<i64>()
        .unwrap();

    let program = lines.nth(1).unwrap()["Program: ".len()..]
        .split(",")
        .map(|c| c.parse::<u8>().unwrap())
        .collect_vec();

    fn _solve(program: &[u8], current_a: i64, b: i64, c: i64, out_index: usize) -> Option<i64> {
        let out = program[out_index];
        for a_start in 0..8 {
            let x = run_program(program, Registers { a: (current_a << 3) | a_start, b, c });
            if let Some(&last) = x.first() {
                if last == out as i64 {
                    if out_index == 0 {
                        return Some(current_a << 3 | a_start);
                    } else {
                        let s = _solve(program, current_a << 3 | a_start, b, c, out_index-1);
                        if s.is_some() {
                            return s;
                        }
                    }
                }
            }
        }
        None
    }

    _solve(&program, 0, register_b, register_c, program.len() - 1).unwrap() as usize
}

fn run_program(program: &[u8], mut registers: Registers) -> Vec<i64> {
    let mut pc = 0;
    let mut output = Vec::new();
    while pc < program.len() {
        let instr = program[pc];
        let operand = program[pc + 1];

        match instr {
            0 => {
                registers.a /= 1 << registers.combo_operand(operand);
            }
            1 => {
                registers.b ^= operand as i64;
            }
            2 => {
                registers.b = registers.combo_operand(operand) & 0b0111;
            }
            3 => {
                if registers.a != 0 {
                    pc = operand as usize;
                    continue;
                }
            }
            4 => {
                registers.b ^= registers.c;
            }
            5 => output.push(registers.combo_operand(operand) & 0b0111),
            6 => {
                registers.b =
                    registers.a / (1 << registers.combo_operand(operand));
            }
            7 => {
                registers.c =
                    registers.a / (1 << registers.combo_operand(operand));
            }
            _ => panic!("Unknown opcode {}", instr),
        }

        pc += 2;
    }

    output
}

struct Registers {
    a: i64,
    b: i64,
    c: i64,
}

impl Registers {
    fn combo_operand(&self, value: u8) -> i64 {
        match value {
            0..=3 => value as i64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => panic!("Invalid combo operand {}", value),
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn part_1() {
        assert_eq!(super::solve_part_1(INPUT), "4,6,3,5,6,3,5,2,1,0");
    }

    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part_2() {
        assert_eq!(super::solve_part_2(INPUT_2), 117440);
    }

    const INPUT_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";
}
