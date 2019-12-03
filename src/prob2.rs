pub fn solve_part_1() {
    let result = run(input());

    println!("Part 1: {}", result[0]);
}

pub fn solve_part_2() {
    let mut result = 0;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut input = input();
            input[1] = noun;
            input[2] = verb;
            let out = run(input);
            if out[0] == 19690720 {
                result = 100 * noun + verb;
                break;
            }
        }
    }

    println!("Part 2: {}", result);
}

fn run(mut code: Vec<u32>) -> Vec<u32> {
    let mut pc = 0;
    let mut op_code = code[pc];

    while op_code != 99 {
        let op1 = code[pc + 1] as usize;
        let op2 = code[pc + 2] as usize;
        let result = match op_code {
            1 => code[op1] + code[op2],
            2 => code[op1] * code[op2],
            _ => panic!("Illegal opcode"),
        };

        let dest = code[pc + 3];
        code.as_mut_slice()[dest as usize] = result;
        pc += 4;
        op_code = code[pc];
    }
    return code;
}

fn input() -> Vec<u32> {
    vec![
        1, 12, 2, 3, 1, 1, 2, 3, 1, 3, 4, 3, 1, 5, 0, 3, 2, 1, 10, 19, 1, 19, 5, 23, 2, 23, 9, 27,
        1, 5, 27, 31, 1, 9, 31, 35, 1, 35, 10, 39, 2, 13, 39, 43, 1, 43, 9, 47, 1, 47, 9, 51, 1, 6,
        51, 55, 1, 13, 55, 59, 1, 59, 13, 63, 1, 13, 63, 67, 1, 6, 67, 71, 1, 71, 13, 75, 2, 10,
        75, 79, 1, 13, 79, 83, 1, 83, 10, 87, 2, 9, 87, 91, 1, 6, 91, 95, 1, 9, 95, 99, 2, 99, 10,
        103, 1, 103, 5, 107, 2, 6, 107, 111, 1, 111, 6, 115, 1, 9, 115, 119, 1, 9, 119, 123, 2, 10,
        123, 127, 1, 127, 5, 131, 2, 6, 131, 135, 1, 135, 5, 139, 1, 9, 139, 143, 2, 143, 13, 147,
        1, 9, 147, 151, 1, 151, 2, 155, 1, 9, 155, 0, 99, 2, 0, 14, 0,
    ]
}

#[cfg(test)]
mod test {

    #[test]
    fn sample_input() {
        assert_eq!(super::run(vec![1, 0, 0, 0, 99]), vec!(2, 0, 0, 0, 99));
        assert_eq!(super::run(vec![2, 3, 0, 3, 99]), vec!(2, 3, 0, 6, 99));
        assert_eq!(
            super::run(vec![2, 4, 4, 5, 99, 0]),
            vec!(2, 4, 4, 5, 99, 9801)
        );
        assert_eq!(
            super::run(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]),
            vec!(30, 1, 1, 4, 2, 5, 6, 0, 99)
        );
        assert_eq!(
            super::run(vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]),
            vec!(3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50)
        );
    }
}
