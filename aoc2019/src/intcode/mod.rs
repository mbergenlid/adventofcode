use std::collections::HashMap;
use std::slice::Iter;
use std::sync::mpsc::{channel, Receiver, Sender, RecvError, TryRecvError};
use std::thread;
#[derive(Clone)]
pub struct IntCode {
    relative_base: usize,
    pub code: Vec<i64>,
    extra_memory: HashMap<usize, i64>,
    default_input: Option<i64>,
}

impl IntCode {
    pub fn new(code: Vec<i64>) -> IntCode {
        IntCode {
            relative_base: 0,
            code,
            extra_memory: HashMap::new(),
            default_input: None,
        }
    }

    pub fn with_default_input(mut self, default_input: i64) -> IntCode {
        self.default_input = Some(default_input);
        self
    }

    pub fn run(&mut self, input: Iter<i64>) -> Vec<i64> {
        let clone = self.clone();
        let (input_sender, input_receiver) = channel();
        let (output_sender, output_receiver) = channel();
        clone.run_async(input_receiver, output_sender);

        for &i in input {
            input_sender.send(i).unwrap();
        }

        output_receiver.iter().collect::<Vec<i64>>()
    }

    pub fn run_1<I: Iterator<Item=i64>>(&mut self, input: I) -> Vec<i64> {
        let clone = self.clone();
        let (input_sender, input_receiver) = channel();
        let (output_sender, output_receiver) = channel();
        clone.run_async(input_receiver, output_sender);

        for i in input {
            input_sender.send(i).unwrap();
        }

        output_receiver.iter().collect::<Vec<i64>>()
    }

    pub fn run_async<I: 'static + Input + Send>(mut self, input: I, output: Sender<i64>) {
        thread::spawn(move || {
            let mut pc = 0;
            let mut op_code: String = format!("{:06}", self.code[pc]);

            while op_code != "99" {
                //println!("{} {} {:?}", op_code, pc, &self.code[pc..pc + 4]);
                match &op_code[op_code.len() - 2..] {
                    "01" => {
                        self.store(
                            self.get_parameter_address(&op_code, pc, 3) as usize,
                            self.get_parameter(&op_code, pc, 1)
                                + self.get_parameter(&op_code, pc, 2),
                        );
                        pc += 4;
                    }
                    "02" => {
                        self.store(
                            self.get_parameter_address(&op_code, pc, 3) as usize,
                            self.get_parameter(&op_code, pc, 1)
                                * self.get_parameter(&op_code, pc, 2),
                        );
                        pc += 4;
                    }
                    "03" => {
                        self.store(
                            self.get_parameter_address(&op_code, pc, 1),
                            self.read_input(&input),
                        );
                        pc += 2;
                    }
                    "04" => {
                        output.send(self.get_parameter(&op_code, pc, 1)).unwrap();
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
                        if self.get_parameter(&op_code, pc, 1) < self.get_parameter(&op_code, pc, 2)
                        {
                            self.store(self.get_parameter_address(&op_code, pc, 3), 1);
                        } else {
                            self.store(self.get_parameter_address(&op_code, pc, 3), 0);
                        }
                        pc += 4;
                    }
                    "08" => {
                        if self.get_parameter(&op_code, pc, 1)
                            == self.get_parameter(&op_code, pc, 2)
                        {
                            self.store(self.get_parameter_address(&op_code, pc, 3), 1);
                        } else {
                            self.store(self.get_parameter_address(&op_code, pc, 3), 0);
                        }
                        pc += 4;
                    }
                    "09" => {
                        self.relative_base = ((self.relative_base as i64)
                            + self.get_parameter(&op_code, pc, 1))
                            as usize;
                        pc += 2;
                    }
                    "99" => {
                        return;
                    }
                    c => panic!("Illegal opcode {} at {}", c, pc),
                };

                //println!(
                //    "225: {}, 6: {}, {}",
                //    self.code[225], self.code[6], self.input
                //);

                op_code = format!("{:06}", self.code[pc]);
            }
        });
    }

    fn read_input<I: Input>(&self, input: &I) -> i64 {
        if self.default_input.is_some() {
            match input.try_recv() {
                Ok(value) => value,
                Err(_) => self.default_input.unwrap(),
            }
        } else {
            input.recv().unwrap()
        }
    }

    fn get_parameter_address(&self, op_code: &str, pc: usize, index: usize) -> usize {
        let idx = op_code.len() - 2 - index;
        match &op_code[idx..idx + 1] {
            "0" => self.get(pc + index) as usize,
            "1" => pc + index,
            "2" => ((self.relative_base as i64) + self.get(pc + index)) as usize,
            p => panic!("Unknown parameter type {}", p),
        }
    }

    fn get_parameter(&self, op_code: &str, pc: usize, index: usize) -> i64 {
        self.get(self.get_parameter_address(op_code, pc, index))
    }

    fn get(&self, src: usize) -> i64 {
        if src >= self.code.len() {
            *self.extra_memory.get(&src).unwrap_or(&0)
        } else {
            self.code[src]
        }
    }

    fn store(&mut self, dest: usize, data: i64) {
        if dest >= self.code.len() {
            self.extra_memory.insert(dest, data);
        } else {
            self.code.as_mut_slice()[dest as usize] = data;
        }
    }
}

pub trait Input {
    fn recv(&self) -> Result<i64, RecvError>;
    fn try_recv(&self) -> Result<i64, TryRecvError>;
}

impl Input for Receiver<i64> {
    fn recv(&self) -> Result<i64, RecvError> {
        self.recv()
    }

    fn try_recv(&self) -> Result<i64, TryRecvError> {
        self.try_recv()
    }
}

#[cfg(test)]
mod test {
    use super::IntCode;
    use std::sync::mpsc::channel;
    #[test]
    fn test() {
        let mut code = IntCode::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        assert_eq!(code.run([8].iter()), vec!(1));

        assert_eq!(
            IntCode::new(vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]).run([9].iter()),
            vec!(0)
        );

        assert_eq!(
            IntCode::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).run([9].iter()),
            vec!(0)
        );
        assert_eq!(
            IntCode::new(vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]).run([4].iter()),
            vec!(1)
        );
    }

    #[test]
    fn prepare_for_problem_9() {
        let (sender, receiver) = channel();
        let (_, ignored) = channel();
        IntCode::new(vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ])
        .run_async(ignored, sender);

        let out: Vec<_> = receiver.iter().collect();
        assert_eq!(
            out,
            vec!(109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99)
        );
    }

    #[test]
    fn large_numbers() {
        let result = IntCode::new(vec![104, 1125899906842624, 99]).run([0].iter());
        assert_eq!(result, vec!(1125899906842624));

        assert_eq!(
            IntCode::new(vec!(1102, 34915192, 34915192, 7, 4, 7, 99, 0)).run([].iter()),
            vec!(34915192 * 34915192)
        );
    }
}
