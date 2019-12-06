pub struct IntCode {
    pub code: Vec<i32>,
}

impl IntCode {
    pub fn new(code: Vec<i32>) -> IntCode {
        IntCode { code: code }
    }

    pub fn run(&mut self, input: i32) -> Vec<i32> {
        let mut output: Vec<i32> = Vec::new();
        let mut pc = 0;
        let mut op_code: String = format!("{:06}", self.code[pc]);

        while op_code != "99" {
            //println!("IP: {}, OP: {}, {:?}", pc, op_code, &self.code[pc..pc + 5]);
            match &op_code[op_code.len() - 2..] {
                "01" => {
                    self.store(
                        self.code[pc + 3] as usize,
                        self.get_parameter(&op_code, pc, 1) + self.get_parameter(&op_code, pc, 2),
                    );
                    pc += 4;
                }
                "02" => {
                    self.store(
                        self.code[pc + 3] as usize,
                        self.get_parameter(&op_code, pc, 1) * self.get_parameter(&op_code, pc, 2),
                    );
                    pc += 4;
                }
                "03" => {
                    self.store(self.code[pc + 1] as usize, input);
                    pc += 2;
                }
                "04" => {
                    println!("{}", self.get_parameter(&op_code, pc, 1));
                    output.push(self.get_parameter(&op_code, pc, 1));
                    pc += 2;
                }
                "05" => {
                    if self.get_parameter(&op_code, pc, 1) != 0 {
                        pc = self.get_parameter(&op_code, pc, 2) as usize;
                    } else {
                        pc += 3;
                    }
                }
                "06" => {
                    if self.get_parameter(&op_code, pc, 1) == 0 {
                        pc = self.get_parameter(&op_code, pc, 2) as usize;
                    } else {
                        pc += 3;
                    }
                }
                "07" => {
                    if self.get_parameter(&op_code, pc, 1) < self.get_parameter(&op_code, pc, 2) {
                        self.store(self.code[pc + 3] as usize, 1);
                    } else {
                        self.store(self.code[pc + 3] as usize, 0);
                    }
                    pc += 4;
                }
                "08" => {
                    if self.get_parameter(&op_code, pc, 1) == self.get_parameter(&op_code, pc, 2) {
                        self.store(self.code[pc + 3] as usize, 1);
                    } else {
                        self.store(self.code[pc + 3] as usize, 0);
                    }
                    pc += 4;
                }
                "99" => {
                    println!("Done!");
                    return output;
                }
                c => panic!("Illegal opcode {}", c),
            };

            //println!(
            //    "225: {}, 6: {}, {}",
            //    self.code[225], self.code[6], self.input
            //);

            op_code = format!("{:06}", self.code[pc]);
        }
        return output;
    }

    fn get_parameter(&self, op_code: &str, pc: usize, index: usize) -> i32 {
        let idx = op_code.len() - 2 - index;
        match &op_code[idx..idx + 1] {
            "0" => self.code[self.code[pc + index] as usize],
            "1" => self.code[pc + index],
            p => panic!("Unknown parameter type {}", p),
        }
    }

    fn store(&mut self, dest: usize, data: i32) {
        self.code.as_mut_slice()[dest as usize] = data;
    }
}

#[cfg(test)]
mod test {
    use super::IntCode;
    #[test]
    fn test() {
        let mut code = IntCode {
            code: vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
        };
        assert_eq!(code.run(8), vec!(1));

        assert_eq!(
            IntCode::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]).run(9),
            vec!(0)
        );

        assert_eq!(
            IntCode::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).run(9),
            vec!(0)
        );
        assert_eq!(
            IntCode::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).run(4),
            vec!(1)
        );
    }
}
