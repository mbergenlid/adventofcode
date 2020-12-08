use crate::prob8::Error::{InvalidInstruction, ParameterNotInteger};
use crate::prob8::Instruction::{ACC, JMP, NOP};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

pub fn solve_part_1() -> i32 {
    let mut boot_code = include_str!("../inputs/prob8")
        .parse::<BootCode>()
        .unwrap();

    boot_code.run();
    boot_code.acc
}

pub fn solve_part_2() -> i32 {
    part_2(include_str!("../inputs/prob8"))
}

fn part_2(code: &str) -> i32 {
    let mut boot_code = code.parse::<BootCode>().unwrap();

    let mut last_modified_instruction = usize::MAX;

    while boot_code.pc != boot_code.code.len() {
        boot_code.reset();
        if let Some((index, instr)) = boot_code
            .code
            .iter()
            .enumerate()
            .skip(last_modified_instruction.wrapping_add(1))
            .find(|(_, i)| i.is_jmp_or_nop())
        {
            boot_code.code[index] = instr.swap();
            last_modified_instruction = index;
        }
        boot_code.run();
        boot_code.code[last_modified_instruction] =
            boot_code.code[last_modified_instruction].swap();
    }
    boot_code.acc
}

struct BootCode {
    code: Vec<Instruction>,
    pc: usize,
    acc: i32,
}

impl BootCode {
    fn run(&mut self) {
        let mut instructions_executed = HashSet::new();
        let mut pc = self.pc;
        let mut acc = self.acc;
        while !instructions_executed.contains(&pc) && pc < self.code.len() {
            instructions_executed.insert(pc);
            match self.code[pc] {
                ACC(v) => {
                    acc += v;
                    pc += 1;
                }
                JMP(v) => {
                    pc = ((pc as i32) + v) as usize;
                }
                NOP(_) => {
                    pc += 1;
                }
            }
        }
        self.pc = pc;
        self.acc = acc;
    }

    fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }
}

impl FromStr for BootCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(BootCode {
            code: s
                .lines()
                .map(|instr| instr.parse::<Instruction>().unwrap_or_else(|_| panic!()))
                .collect::<Vec<_>>(),
            pc: 0,
            acc: 0,
        })
    }
}

#[derive(Debug)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

impl Instruction {
    fn is_jmp_or_nop(&self) -> bool {
        match self {
            JMP(_) => true,
            NOP(_) => true,
            _ => false,
        }
    }

    fn swap(&self) -> Instruction {
        match self {
            JMP(v) => NOP(*v),
            NOP(v) => JMP(*v),
            i => panic!("Can not swap {:?}", i),
        }
    }
}

#[derive(Debug)]
enum Error {
    ParameterNotInteger,
    InvalidInstruction(String),
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split_whitespace();
        let (instruction, parameter) = (
            parts.next().unwrap_or_default(),
            parts.next().unwrap_or_default(),
        );
        match instruction {
            "acc" => Ok(ACC(parameter.parse::<i32>()?)),
            "jmp" => Ok(JMP(parameter.parse::<i32>()?)),
            "nop" => Ok(NOP(parameter.parse::<i32>()?)),
            _ => Err(InvalidInstruction(instruction.to_string())),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(_e: ParseIntError) -> Self {
        ParameterNotInteger
    }
}

#[cfg(test)]
mod test {
    use crate::prob8::part_2;

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6"
            ),
            8
        );
    }
}
