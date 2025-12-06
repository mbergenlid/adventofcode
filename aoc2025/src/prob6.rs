pub fn solve_part_1(input: &str) -> usize {
    let (last_index, last_line) = input
        .lines()
        .enumerate()
        .last()
        .expect("Expect at least one line");
    let numbers = input
        .lines()
        .take(last_index)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u64>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (problem_num, operator) in last_line.split_whitespace().enumerate() {
        let mut result = numbers[0][problem_num];
        for idx in 1..last_index {
            match operator {
                "+" => result += numbers[idx][problem_num],
                "*" => result *= numbers[idx][problem_num],
                _ => panic!("Invalid operand {operator}"),
            }
        }
        sum += result;
    }
    sum as usize
}

pub fn solve_part_2(input: &str) -> usize {
    let (last_index, last_line) = input
        .lines()
        .enumerate()
        .last()
        .expect("Expect at least one line");
    let operators = last_line.split_whitespace().collect::<Vec<_>>();
    let lines = input.lines().take(last_index).collect::<Vec<_>>();

    let mut col = lines[0].len() - 1;
    let mut total = 0_usize;
    for problem_idx in (0..operators.len()).rev() {
        let mut result = match operators[problem_idx] {
            "+" => 0,
            "*" => 1,
            op => panic!("Invalid operator {op}"),
        };
        let mut blank_column = false;
        while !blank_column {
            //Parse one number
            let mut number = 0;
            blank_column = true;
            for line in &lines {
                if let Some(digit) = line.chars().nth(col).unwrap().to_digit(10) {
                    blank_column = false;
                    number = number * 10 + digit
                }
            }
            if !blank_column {
                match operators[problem_idx] {
                    "+" => result += number as usize,
                    "*" => result *= number as usize,
                    op => panic!("Invalid operator {op}"),
                }
            }
            if col == 0 {
                blank_column = true;
            } else {
                col -= 1;
            }
        }
        total += result;
    }
    total
}

#[cfg(test)]
mod test {
    use crate::prob6::{solve_part_1, solve_part_2};

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(INPUT), 4277556);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(INPUT), 3263827);
    }

    const INPUT: &'static str = "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
";
}
